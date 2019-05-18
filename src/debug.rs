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
