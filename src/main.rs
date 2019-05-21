// Basic no_std binary
#![no_std]
#![feature(start, lang_items)]
// Useful - but we could remove these if we needed to.
#![feature(core_intrinsics)]
#![feature(panic_info_message)]
// Because I *can*.
#![feature(non_ascii_idents)]

use core::{
    mem,
    ptr,
};

use winapi::um::errhandlingapi::GetLastError;
use winapi::{
    shared::windef,
    um::libloaderapi,
    um::wingdi as gdi,
    um::winuser as user,
};

// This gets us the println!() macro, so it must be declared first.
#[macro_use]
mod debug;

// Rust runtime defines
mod rt;

// OpenGL Loader
#[allow(bad_style)]
mod ogl;

/// Contains platform-specific handles for managing a window and OpenGL context
#[derive(Debug)]
pub struct Window {
    /// Handle to Window
    h_wnd: windef::HWND,

    /// Handle to Device Context
    h_dc: windef::HDC,

    /// Handle to OpenGL Render Context
    h_glrc: windef::HGLRC,
}

impl Window {
    pub fn swap_buffers(&mut self) {
        unsafe {
            let res: i32 = gdi::SwapBuffers(self.h_dc);
            if res == 0 {
                abort!("gdi::SwapBuffers(0x{:x}) failed!", self.h_dc as usize);
            }
        }
    }
}

unsafe extern "system" fn wnd_proc(
    h_wnd: windef::HWND,
    msg: u32,
    w_param: usize,
    l_param: isize,
) -> isize {
    match msg {
        user::WM_SIZE => {
            let width = l_param & 0xffff;
            let height = l_param >> 16;
            println!("WM_Size: {} x {}", width, height);
        },
        user::WM_CHAR => {
            match w_param as i32 {
                // ESC closes the window
                user::VK_ESCAPE => {
                    user::PostQuitMessage(0);
                    return 0;
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

fn check_compilation(
    &gl: &ogl::GlFuncs,
    glsl: u32,
    glsl_source: &[u8],
    name: &str,
) {
    #[cfg(feature = "enable_logging")]
    unsafe {
        print!("Checking compilation result of {}...", name);

        let mut success: i32 = 0;
        (gl.GetShaderiv)(glsl, ogl::GL_COMPILE_STATUS, &mut success);

        if success == 0 {
            println!("failed");

            let mut info_buf: [u8; 512] = mem::zeroed();
            (gl.GetShaderInfoLog)(
                glsl,
                info_buf.len() as i32,
                ptr::null_mut(),
                &mut info_buf as *mut _ as *mut i8,
            );
            let info_str = core::str::from_utf8(&info_buf).unwrap();

            println!("Full Source:");
            let source = core::str::from_utf8(glsl_source).unwrap_or_default();
            for (lineno, line) in source.split('\n').enumerate() {
                let lineno = lineno + 1;
                println!("{:>3}: {}", lineno, line);
            }

            println!("{}", info_str);
            abort!();
        } else {
            println!("ok");
        }
    }
}

fn check_linkage(&gl: &ogl::GlFuncs, prog: u32, names: &[&str]) {
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

            let mut info_buf: [u8; 512] = mem::zeroed();
            (gl.GetProgramInfoLog)(
                prog,
                info_buf.len() as i32,
                ptr::null_mut(),
                &mut info_buf as *mut _ as *mut i8,
            );
            let info_str = core::str::from_utf8(&info_buf).unwrap();

            println!("{}", info_str);
            abort!();
        } else {
            println!("ok");
        }
    }
}


#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
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

        // Create Window handle
        let ex_style = user::WS_EX_APPWINDOW | user::WS_EX_WINDOWEDGE;
        let style = user::WS_OVERLAPPEDWINDOW | user::WS_VISIBLE;
        h_wnd = user::CreateWindowExA(
            ex_style,        // DWORD     dwExStyle
            THING_NAME,      // LPCSTR    lpClassName
            THING_NAME,      // LPCSTR    lpWindowName
            style,           // DWORD     dwStyle
            0,               // int       X
            0,               // int       Y
            800,             // int       nWidth
            600,             // int       nHeight
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

        // user::ShowCursor(0);

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

    // Setup render state
    unsafe {
        (gl.ClearColor)(0., 0., 0., 0.);
    }

    // Load Shaders
    let sérusier_prog: u32;
    unsafe {
        // Load Vertex Shader

        let mut vert_src = include_bytes!("../glsl/sérusier.vert").clone();
        // Replaces the last newline with a NUL
        vert_src[vert_src.len() - 1] = 0;
        let vert_src = vert_src;

        let sérusier_vert: u32 = (gl.CreateShader)(ogl::GL_VERTEX_SHADER);

        let p_vert_src: *const i8 = vert_src.as_ptr() as *const i8;
        (gl.ShaderSource)(sérusier_vert, 1, &p_vert_src, ptr::null());

        (gl.CompileShader)(sérusier_vert);
        check_compilation(&gl, sérusier_vert, &vert_src, "sérusier.vert");

        // Load Fragment Shader
        let mut frag_src = include_bytes!("../glsl/sérusier.frag").clone();
        // Replaces the last newline with a NUL
        frag_src[frag_src.len() - 1] = 0;
        let frag_src = frag_src;

        let sérusier_frag: u32 = (gl.CreateShader)(ogl::GL_FRAGMENT_SHADER);

        let p_frag_src: *const i8 = frag_src.as_ptr() as *const i8;
        (gl.ShaderSource)(sérusier_frag, 1, &p_frag_src, ptr::null());

        (gl.CompileShader)(sérusier_frag);
        check_compilation(&gl, sérusier_frag, &frag_src, "sérusier.frag");

        // Link everything
        sérusier_prog = (gl.CreateProgram)();
        (gl.AttachShader)(sérusier_prog, sérusier_vert);
        (gl.AttachShader)(sérusier_prog, sérusier_frag);
        (gl.LinkProgram)(sérusier_prog);
        check_linkage(&gl, sérusier_prog, &[
            "sérusier.vert",
            "sérusier.frag",
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
    while keep_running {
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

        unsafe {
            (gl.Clear)(ogl::GL_COLOR_BUFFER_BIT | ogl::GL_DEPTH_BUFFER_BIT);

            // TODO: Render things. Triangles maybe?
            (gl.UseProgram)(sérusier_prog);
            (gl.DrawArrays)(ogl::GL_TRIANGLE_STRIP, 0, 6);

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
