use core::{
    self,
    fmt,
};

// An implementation of `fmt::Write` that forwards all writes into stdout.
//
// Characters are written one at a time through `putchar`. This saves us the
// headache of managing a temporary buffer and using `puts` or `printf`.
// On Windows, it additionally writes to `OutputDebugStringA`.
pub struct DebugWriter;

impl fmt::Write for DebugWriter {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        for byte in text.as_bytes() {
            let byte = *byte;
            unsafe {
                #[cfg(windows)]
                {
                    // Write to debug output
                    let string: [i8; 2] = [byte as i8, 0];
                    winapi::um::debugapi::OutputDebugStringA(&string[0]);
                }

                // Write to the console
                libc::putchar(byte as i32);
            }
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($toks:tt)*) => {
        if cfg!(feature = "enable_logging") {
            use core::fmt::Write;
            match write!(crate::debug::DebugWriter {}, $($toks)*) {
                Ok(()) => {},
                Err(ref err) => panic!("print!(): {:#?}", err),
            }
        }
    };
}

#[macro_export]
macro_rules! println {
    ($($toks:tt)*) => {
        if cfg!(feature = "enable_logging") {
            use core::fmt::Write;
            match writeln!(crate::debug::DebugWriter {}, $($toks)*) {
                Ok(()) => {},
                Err(ref err) => panic!("dbg_println!(): {:#?}", err),
            }
        }
    };
}

#[macro_export]
macro_rules! abort {
    ($($toks:tt)*) => {
        println!($($toks)*);
        panic!("");
    };
}

#[derive(Copy, Clone)]
struct Hex(usize);

impl fmt::Debug for Hex {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "0x{:x}", self.0)
    }
}

impl fmt::Debug for crate::ogl::GlFuncs {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = fmt.debug_struct("GlFuncs");
        debug
            .field("hOpenGL", &Hex(self.hOpenGL as usize))
            .field("glCreateShader", &Hex(self.CreateShader as usize))
            .field("glShaderSource", &Hex(self.ShaderSource as usize))
            .field("glCompileShader", &Hex(self.CompileShader as usize))
            .field("glAttachShader", &Hex(self.AttachShader as usize))
            .field("glCreateProgram", &Hex(self.CreateProgram as usize))
            .field("glLinkProgram", &Hex(self.LinkProgram as usize))
            .field("glUseProgram", &Hex(self.UseProgram as usize))
            .field("glClearColor", &Hex(self.ClearColor as usize))
            .field("glClear", &Hex(self.Clear as usize))
            .field("glClearDepth", &Hex(self.ClearDepth as usize))
            .field("glDrawArrays", &Hex(self.DrawArrays as usize));

        #[cfg(feature = "dev_build")]
        {
            debug
                .field("glGetShaderiv", &Hex(self.GetShaderiv as usize))
                .field(
                    "glGetShaderInfoLog",
                    &Hex(self.GetShaderInfoLog as usize),
                )
                .field("glGetProgramiv", &Hex(self.GetProgramiv as usize))
                .field(
                    "glGetProgramInfoLog",
                    &Hex(self.GetProgramInfoLog as usize),
                )
                .field("glViewport", &Hex(self.Viewport as usize));
        }
        debug.finish()
    }
}
