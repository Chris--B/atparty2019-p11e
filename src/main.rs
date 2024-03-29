// Basic no_std binary
#![no_std]
#![feature(start, lang_items)]
// Useful - but we could remove these if we needed to.
#![feature(core_intrinsics)]
#![feature(panic_info_message)]
// Because I *can*.
#![feature(non_ascii_idents)]
#![cfg_attr(not(feature = "dev_build"), windows_subsystem = "windows")]

use core::{
    mem,
    ptr,
    sync::atomic::{
        AtomicBool,
        AtomicI32,
        Ordering,
    },
};

use nalgebra;
use winapi::um::errhandlingapi::GetLastError;
use winapi::{
    shared::windef,
    um::libloaderapi,
    um::wingdi as gdi,
    um::winuser as user,
};

// This gets us the println!() macro, so it must be declared first.
#[macro_use]
#[allow(dead_code)]
mod debug;

// Rust runtime defines
mod rt;

// OpenGL Loader
#[allow(bad_style, dead_code)]
mod ogl;

// Audio Utility
#[allow(dead_code)]
mod audio;

// https://www.nayuki.io/page/free-small-fft-in-multiple-languages
#[allow(dead_code)]
mod fft;

/// Contains platform-specific handles for managing a window and OpenGL context
#[derive(Debug)]
struct Window {
    /// Handle to Window
    h_wnd: windef::HWND,

    /// Handle to Device Context
    h_dc: windef::HDC,

    /// Handle to OpenGL Render Context
    h_glrc: windef::HGLRC,
}

type Mat4 = nalgebra::Matrix4<f32>;
type Point3 = nalgebra::Point3<f32>;
type Vec3 = nalgebra::Vector3<f32>;

/// Field of View in screen-space y coordinates, in RADIANS
const FOV: f32 = 45. * 3.14 / 180.;
static PENDING_RESIZE: AtomicI32 = AtomicI32::new(-1);
static PAUSED: AtomicBool = AtomicBool::new(false);

unsafe extern "system" fn wnd_proc(
    h_wnd: windef::HWND,
    msg: u32,
    w_param: usize,
    l_param: isize,
) -> isize {
    const VK_T: i32 = b't' as _;
    match msg {
        user::WM_SIZE => {
            // println!("WM_Size: {} x {}", width, height);
            PENDING_RESIZE.store(l_param as i32, Ordering::SeqCst);
        },
        user::WM_CHAR => {
            match w_param as i32 {
                // ESC closes the window
                user::VK_ESCAPE => {
                    user::PostQuitMessage(0);
                    return 0;
                },
                user::VK_SPACE => {
                    PAUSED.fetch_xor(true, Ordering::SeqCst);
                },
                VK_T => {
                    println!("time = {}", get_time());
                },
                _ => {},
            }
        },
        user::WM_DESTROY => {
            user::PostQuitMessage(0);
            return 0;
        },
        _ => {},
    }
    user::DefWindowProcA(h_wnd, msg, w_param, l_param)
}

#[cfg(feature = "enable_logging")]
static mut INFO_BUFFER: [u8; 4096] = [0; 4096];

fn check_compilation(
    &gl: &ogl::GlFuncs,
    glsl: u32,
    glsl_source: &[u8],
    name: &str,
) {
    #[cfg(not(feature = "enable_logging"))]
    {
        mem::drop(gl);
        mem::drop(glsl);
        mem::drop(glsl_source);
        mem::drop(name);
    }
    #[cfg(feature = "enable_logging")]
    unsafe {
        print!("Checking compilation result of {}...", name);

        let mut success: i32 = 0;
        (gl.GetShaderiv)(glsl, ogl::GL_COMPILE_STATUS, &mut success);

        if success == 0 {
            println!("failed");

            (gl.GetShaderInfoLog)(
                glsl,
                INFO_BUFFER.len() as i32,
                ptr::null_mut(),
                &mut INFO_BUFFER as *mut _ as *mut i8,
            );
            let info_str = core::str::from_utf8(&INFO_BUFFER).unwrap();

            println!("Full Source:");
            let source = core::str::from_utf8(glsl_source).unwrap_or_default();
            for (lineno, line) in source.split('\n').enumerate() {
                let lineno = lineno + 1;
                println!("{:>3}: {}", lineno, line.trim_end());
            }

            println!("{}", info_str);
            abort!("Failed to compile.");
        } else {
            println!("ok");
        }
    }
}

fn check_linkage(&gl: &ogl::GlFuncs, prog: u32, names: &[&str]) {
    #[cfg(not(feature = "enable_logging"))]
    {
        mem::drop(gl);
        mem::drop(prog);
        mem::drop(names);
    }

    #[cfg(feature = "enable_logging")]
    unsafe {
        print!("Checking link result of ");
        if let Some(name) = names.first() {
            print!("{}", name);
        }
        for name in names.iter().skip(1) {
            print!(", {}", name);
        }
        print!("...");

        let mut success: i32 = 0;
        (gl.GetProgramiv)(prog, ogl::GL_LINK_STATUS, &mut success);

        if success == 0 {
            println!("failed");

            (gl.GetProgramInfoLog)(
                prog,
                INFO_BUFFER.len() as i32,
                ptr::null_mut(),
                &mut INFO_BUFFER as *mut _ as *mut i8,
            );
            let info_str = core::str::from_utf8(&INFO_BUFFER).unwrap();

            println!("{}", info_str);
            abort!("Failed to link");
        } else {
            println!("ok");
        }
    }
}

// Get a number vaguely related to time.
// It currently loops every 50 seconds on my laptop. ish.
fn get_time() -> f64 {
    // I took short cuts, so this is needed to make this work.
    const SCALE: f64 = 2_144_392.0 / 10_000_000.0;

    unsafe {
        let mut freq = mem::zeroed();
        winapi::um::profileapi::QueryPerformanceFrequency(&mut freq);

        let mut counter = mem::zeroed();
        winapi::um::profileapi::QueryPerformanceCounter(&mut counter);

        let itime = *counter.QuadPart();
        let ifreq = *freq.QuadPart();
        let seconds = itime / ifreq;
        let subsecs = itime % ifreq;
        SCALE * (seconds as f64 + (subsecs as f64 / ifreq as f64))
    }
}

fn generate_view_mat(time: f32) -> Mat4 {
    // Sorted by [t0, t1] pairs
    const EYES: &[(f32, f32, [f32; 3], [f32; 3])] = &[
        // t0, t1, eye0, eye1
        (0., 4., [-20., -20., 2.5], [-5., -20., 2.5]),
        (4., 8., [-30., 30., 5.], [-15., 15., 5.]),
        // (8., 16., [-20., -20., 5.], [-20., 20., 5.]),
    ];
    let time = time % EYES.last().unwrap().1;

    let mut eye = Point3::new(30., 30., 30.);
    for datum in EYES {
        let (t0, t1, eye0, eye1) = *datum;
        if t0 <= time && time <= t1 {
            let eye0 = Vec3::from(eye0);
            let eye1 = Vec3::from(eye1);
            let t = (time - t0) / (t1 - t0);
            eye = Point3::from(t * eye0 + (1. - t) * eye1);
            break;
        }
    }

    const FOCUS: &[(f32, f32, [f32; 3], [f32; 3])] = &[
        // t0, t1, focus0, focus1
        (0., 4., [15., 0., 2.5], [0., 0., 2.5]),
        (4., 8., [0., 0., 0.], [0., 0., 5.]),
        // (8., 16., [0., 0., 0.], [0., 0., 0.]),
    ];
    let mut focus = Point3::new(0., 0., -5.);
    for datum in FOCUS {
        let (t0, t1, focus0, focus1) = *datum;
        if t0 <= time && time <= t1 {
            let focus0 = Vec3::from(focus0);
            let focus1 = Vec3::from(focus1);
            let t = (time - t0) / (t1 - t0);
            focus = Point3::from(t * focus0 + (1. - t) * focus1);
            break;
        }
    }

    Mat4::look_at_rh(&eye, &focus, &Vec3::new(0., 0., 1.))
}

#[start]
#[no_mangle]
fn demo_main(_argc: isize, _argv: *const *const u8) -> isize {
    // Initialize a Window and rendering context
    let window: Window;
    unsafe {
        let h_wnd: windef::HWND;
        let h_dc: windef::HDC;
        let h_glrc: windef::HGLRC;

        // Create Window class
        let h_instance = libloaderapi::GetModuleHandleA(ptr::null_mut());

        const THING_NAME: *const i8 = b"yo\0".as_ptr() as *const _;

        let wc = user::WNDCLASSA {
            style: user::CS_OWNDC,
            lpfnWndProc: Some(wnd_proc),
            hInstance: h_instance,
            hIcon: user::LoadIconW(ptr::null_mut(), user::IDI_WINLOGO),
            hCursor: user::LoadCursorW(ptr::null_mut(), user::IDC_ARROW),
            lpszClassName: THING_NAME,

            ..mem::zeroed()
        };
        user::RegisterClassA(&wc);

        let mut dm_screen_settings = mem::zeroed();
        let ok = user::EnumDisplaySettingsA(
            ptr::null(),
            user::ENUM_CURRENT_SETTINGS,
            &mut dm_screen_settings,
        );
        if ok == 0 {
            abort!("EnumDisplaySettingsA() failed");
        }
        let ret = user::ChangeDisplaySettingsA(
            &mut dm_screen_settings,
            user::CDS_FULLSCREEN,
        );
        if ret != user::DISP_CHANGE_SUCCESSFUL {
            abort!("ChangeDisplaySettingsA() failed with: {}", ret);
        }

        // Create Window handle
        let ex_style = 0;
        let style = user::WS_POPUP | user::WS_VISIBLE;
        h_wnd = user::CreateWindowExA(
            ex_style,        // DWORD     dwExStyle
            THING_NAME,      // LPCSTR    lpClassName
            THING_NAME,      // LPCSTR    lpWindowName
            style,           // DWORD     dwStyle
            0,               // int       X
            0,               // int       Y
            1920,            // int       nWidth
            1080,            // int       nHeight
            ptr::null_mut(), // HWND      hWndParent
            ptr::null_mut(), // HMENU     hMenu
            ptr::null_mut(), // HINSTANCE hInstance
            ptr::null_mut(), // LPVOID    lpPara
        );
        if h_wnd == ptr::null_mut() {
            abort!("CreateWindowExA failed: {}", GetLastError());
        }

        // Get device context handle
        h_dc = user::GetDC(h_wnd);
        if h_dc == ptr::null_mut() {
            abort!("GetDC failed: {}", GetLastError());
        }

        let pixel_desc = gdi::PIXELFORMATDESCRIPTOR {
            dwFlags: gdi::PFD_SUPPORT_OPENGL | gdi::PFD_DOUBLEBUFFER,
            iPixelType: gdi::PFD_TYPE_RGBA,
            cDepthBits: 32,

            ..mem::zeroed()
        };
        // Important to set the pixel format before creating the rendering
        // context.
        let pixel_format = gdi::ChoosePixelFormat(h_dc, &pixel_desc);
        gdi::SetPixelFormat(h_dc, pixel_format, &pixel_desc);

        // Get the OpenGL rendering context handle
        h_glrc = gdi::wglCreateContext(h_dc);
        if h_glrc == ptr::null_mut() {
            abort!("wglCreateContext failed: {}", GetLastError());
        }

        let res: i32 = gdi::wglMakeCurrent(h_dc, h_glrc);
        if res == 0 {
            abort!(
                "gdi::wglMakeCurrent(0x{:x}, 0x{:x}) failed",
                h_dc as usize,
                h_glrc as usize
            );
        }

        user::ShowCursor(0);

        window = Window {
            h_wnd,
            h_dc,
            h_glrc,
        };
    }
    println!("Window created! {:#?}", window);

    // Load OpenGL functions
    let mut gl: ogl::GlFuncs;
    unsafe {
        gl = mem::zeroed();
        let res: i32 = ogl::ogl_load(&mut gl);
        if res != 0 {
            abort!("ogl_load() failed: {}\n{:#?}", res, gl);
        }
    }
    let gl = gl;
    println!("{:#?}", gl);

    // Print driver information
    // This is really helpful for debugging since we don't work on Intel.
    // (Hybrid graphics are hard)
    #[cfg(feature = "dev_build")]
    unsafe {
        println!(
            "GL VERSION      {}",
            debug::ptr_to_str((gl.GetString)(ogl::GL_VERSION))
        );
        println!(
            "GL GLSL_VERSION {}",
            debug::ptr_to_str((gl.GetString)(ogl::GL_SHADING_LANGUAGE_VERSION))
        );
        println!(
            "GL VENDOR       {}",
            debug::ptr_to_str((gl.GetString)(ogl::GL_VENDOR))
        );
        println!(
            "GL RENDERER     {}",
            debug::ptr_to_str((gl.GetString)(ogl::GL_RENDERER))
        );
        println!("");
    }

    // Setup render state
    unsafe {
        (gl.ClearColor)(0., 0., 0., 0.);
    }

    // Load Shaders
    let sérusier_prog: u32;
    unsafe {
        // Load Vertex Shader

        let mut vert_src = include_bytes!("../glsl/min.sérusier.vert").clone();
        println!("\"glsl/min.sérusier.vert\": {} bytes", vert_src.len());
        // Replaces the last newline with a NUL
        vert_src[vert_src.len() - 1] = 0;
        let vert_src = vert_src;

        let sérusier_vert: u32 = (gl.CreateShader)(ogl::GL_VERTEX_SHADER);

        let p_vert_src: *const i8 = vert_src.as_ptr() as *const i8;
        (gl.ShaderSource)(sérusier_vert, 1, &p_vert_src, ptr::null());

        (gl.CompileShader)(sérusier_vert);
        check_compilation(&gl, sérusier_vert, &vert_src, "min.sérusier.vert");

        // Load Fragment Shader
        let mut frag_src = include_bytes!("../glsl/min.sérusier.frag").clone();
        println!("\"glsl/min.sérusier.frag\": {} bytes", frag_src.len());
        // Replaces the last newline with a NUL
        frag_src[frag_src.len() - 1] = 0;
        let frag_src = frag_src;

        let sérusier_frag: u32 = (gl.CreateShader)(ogl::GL_FRAGMENT_SHADER);

        let p_frag_src: *const i8 = frag_src.as_ptr() as *const i8;
        (gl.ShaderSource)(sérusier_frag, 1, &p_frag_src, ptr::null());

        (gl.CompileShader)(sérusier_frag);
        check_compilation(&gl, sérusier_frag, &frag_src, "min.sérusier.frag");

        // Link everything
        sérusier_prog = (gl.CreateProgram)();
        (gl.AttachShader)(sérusier_prog, sérusier_vert);
        (gl.AttachShader)(sérusier_prog, sérusier_frag);
        (gl.LinkProgram)(sérusier_prog);
        check_linkage(&gl, sérusier_prog, &[
            "min.sérusier.vert",
            "min.sérusier.frag",
        ]);

        // Logging yay
        println!("Shader handles:");
        println!("    sérusier_vert == {}", sérusier_vert);
        println!("    sérusier_frag == {}", sérusier_frag);
        println!("    sérusier_prog == {}", sérusier_prog);
    }

    let mut frame: u32 = 0;
    let mut keep_running: bool = true;
    let mut ret_code: isize = 0;

    // Needed in debug builds
    #[allow(unused_mut)]
    let mut proj = Mat4::new_perspective(
        1920. / 1080., // aspect ratio
        FOV,           // fovy
        0.1,           // znear
        1e3,           // zfar
    );

    audio::play();

    let start = get_time();
    println!("start time = {}", start);
    while keep_running {
        let time = (get_time() - start) as f32;
        if time > 8. {
            keep_running = false;
        }

        // Win32 boilerplate
        unsafe {
            // Process all outstanding messages from Windows
            loop {
                let mut msg: user::MSG = mem::zeroed();

                // Returns nonzero if there's a message
                let got_msg = user::PeekMessageA(
                    &mut msg,
                    ptr::null_mut(),
                    0,
                    0,
                    user::PM_REMOVE,
                );
                if got_msg != 0 {
                    match msg.message {
                        user::WM_QUIT => {
                            keep_running = false;
                            ret_code = msg.wParam as isize;
                        },
                        _ => {
                            user::TranslateMessage(&mut msg);
                            user::DispatchMessageA(&mut msg);
                        },
                    };
                } else {
                    break;
                }
            }
        }

        if !keep_running {
            // The final present will fail if we don't break early.
            break;
        }

        if PAUSED.load(Ordering::SeqCst) {
            continue;
        }

        // If we're resizing, do the GL thing
        #[cfg(feature = "dev_build")]
        // TODO: Fullscreen by default!
        unsafe {
            let packed: u32 =
                PENDING_RESIZE.swap(core::i32::MAX, Ordering::SeqCst) as u32;
            if packed != core::i32::MAX as u32 {
                let width = packed & 0xffff;
                let height = packed >> 16;

                proj = Mat4::new_perspective(
                    width as f32 / height as f32, // aspect ratio
                    FOV,                          // fovy
                    0.1,                          // znear
                    1e3,                          // zfar
                );

                (gl.Viewport)(0, 0, width as i32, height as i32);
            }
        }

        // Update state
        let view = generate_view_mat(time);

        unsafe {
            (gl.Clear)(ogl::GL_COLOR_BUFFER_BIT | ogl::GL_DEPTH_BUFFER_BIT);

            (gl.UseProgram)(sérusier_prog);

            // Set uniforms
            let u_proj_view = proj * view;
            (gl.UniformMatrix4fv)(
                0, // location
                1, // count
                0, // transpose?
                u_proj_view.data.as_slice().as_ptr(),
            );

            (gl.Uniform1f)(1, time);

            // (gl.DrawArrays)(ogl::GL_TRIANGLES, 0, 4 * 6);
            (gl.DrawArrays)(ogl::GL_TRIANGLES, 0, 36 * 65);

            // Block until rendering finishes and the swapchain presents (??)
            let res = gdi::SwapBuffers(window.h_dc);
            if res == 0 {
                println!("gdi::SwapBuffers() failed: {}", res);
            } else {
                frame += 1;
            }
        }
    }

    println!("Rendered {} frames", frame);

    ret_code
}
