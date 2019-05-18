///! Rust Runtime Defines
///!
///! Defines to satisfy Rust runtime linkage. These are only necessary when
///! building with #![no_std].

use core;

#[panic_handler]
unsafe fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    if cfg!(feature = "enable_logging") {
        let default_message = format_args!("");
        let message = info.message().unwrap_or(&default_message);
        let loc = info.location();

        // Note: This can panic. If that happens, we infinite loop.
        // Don't double panic.
        println!(
            "[FATAL] {}:{}:{}\t{}",
            loc.map(|l| l.file()).unwrap_or_default(),
            loc.map(|l| l.line()).unwrap_or_default(),
            loc.map(|l| l.column()).unwrap_or_default(),
            message,
        );
    }

    // Loop because this function cannot return, but breakpoint() might.
    loop {
        core::intrinsics::breakpoint();
    }
}

// Not sure what this is for - but we need it to build.
#[lang = "eh_personality"]
fn eh_personality() -> usize {
    0
}
