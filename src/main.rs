// Basic no_std binary
#![no_std]
#![feature(start, lang_items)]
// Useful - but we could remove these if we needed to.
#![feature(core_intrinsics)]
#![feature(panic_info_message)]

use core::{
    ptr,
    mem,
};

use winapi::{
    um::wingdi as gdi,
    um::winuser as user,
    shared::windef,
};

// This gets us the println!() macro, so it must be declared first.
#[macro_use]
mod debug;

// Rust runtime defines
mod rt;

/// Contains platform-specific handles for managing a window and OpenGL context
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
                panic!("gdi::SwapBuffers(0x{:x}) failed!", self.h_dc as usize);
            }
        }
    }
}

extern {
    fn it_works() -> u32;
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    println!("Hello, {}!", "world");
    unsafe {
        println!("ffi: {}", it_works());
    }
    0
}

/// Creates a window and an OpenGL rendering context
///
/// The OpenGL rendering context is also made current.
pub fn init_window() -> Option<Window> {
    let h_wnd:  windef::HWND;
    let h_dc:   windef::HDC;
    let h_glrc: windef::HGLRC;

    // We're in C-API land. What else can we do?
    unsafe {
        // Create Window handle
        h_wnd = user::CreateWindowExA(
            0 as _,                                    // DWORD     dwExStyle
            b"edit\0".as_ptr() as *const i8,           // LPCSTR    lpClassName
            b"yo\0".as_ptr() as *const i8,             // LPCSTR    lpWindowName
            user::WS_POPUP | user::WS_VISIBLE,         // DWORD     dwStyle
            0,                                         // int       X
            0,                                         // int       Y
            0,                                         // int       nWidth
            0,                                         // int       nHeight
            ptr::null_mut(),                           // HWND      hWndParent
            ptr::null_mut(),                           // HMENU     hMenu
            ptr::null_mut(),                           // HINSTANCE hInstance
            ptr::null_mut()                            // LPVOID    lpPara
        );
        if h_wnd == ptr::null_mut() {
            return None;
        }

        // Get device context handle
        h_dc = user::GetDC(h_wnd);
        if h_dc == ptr::null_mut() {
            return None;
        }

        let pixel_desc = gdi::PIXELFORMATDESCRIPTOR {
                dwFlags: gdi::PFD_DRAW_TO_WINDOW |
                         gdi::PFD_SUPPORT_OPENGL |
                         gdi::PFD_DOUBLEBUFFER,
                iPixelType: gdi::PFD_TYPE_RGBA,

            .. mem::zeroed()
        };
        // Important to set the pixel format before creating the rendering context.
        let pixel_format = gdi::ChoosePixelFormat(h_dc, &pixel_desc);
        gdi::SetPixelFormat(h_dc, pixel_format, &pixel_desc);

        // Get the OpenGL rendering context handle
        h_glrc = gdi::wglCreateContext(h_dc);
        if h_glrc == ptr::null_mut() {
            return None;
        }

        let res: i32 = gdi::wglMakeCurrent(h_dc, h_glrc);
        if res == 0 {
            println!("gdi::wglMakeCurrent(0x{:x}, 0x{:x}) failed", h_dc as usize, h_glrc as usize);
            return None;
        }
    }

    Some(Window {
        h_wnd,
        h_dc,
        h_glrc,
    })
}
