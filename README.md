## 2019 @Party 64k Intro

This was an entry for @Party's 2019 Intro comp. The final shipped size was 27,136 bytes.

Binary downloads are available [here](https://github.com/Chris--B/atparty2019-p11e/releases).

It uses OpenGL and was written with both C and Rust (against `rustc 1.36.0-nightly (dec4c5201 2019-05-24)`: may have issues compiling as-is on future versions). See the `Building` second below for details.

It was a lot of fun and I learned a lot about size coding from this. For more information:
- About @Party, see [their website](http://atparty-demoscene.net/)
- About 64k Intros, see [this excellent writeup](http://www.lofibucket.com/articles/64k_intro.html)
- About Rust, see [https://rust-lang.org](https://www.rust-lang.org/community)

### Special Thanks

This demo was only possible due to the help and support of my friends and coworkers.
Thanks everyone who helped, directly or indirectly.
```
Cookie Fairy    - For patiently supporting the time sink this project was :)
jimbo00000      - For convincing me to use OpenGL and walking me through audio
TheWearyGamer   - For helping me understand audio
impakt          - Without whom I wouldn't have ever found @Party

DrClaw
Ella572
Falco
octopusprime314
```

## Fellow Developers

### Building

If you're not familiar with Rust, I recommend walking through a hello world project first. This project uses a couple of advanced features and may be confusing if you're never used it before. But your mileage may vary, and you shouldn't need to look into any of it if it works right; so don't sweat it.

It currently expects a nightly version of Rust to compile. This is due to a few things that are unstable:
- `[no_std]` binary
- `[feature(start, lang_times)`]: related to above. With no std, we need to supply the panic handler ourselves. It prints and loops forever. See `src/rt.rs`.
- `[panic_info_message]`: Information about panics when they happen. Probably not necessary in the final release builds.
- `[feature(non_ascii_idents)]`: Because I'm a bad person. The demo was inspired by a French artist and I wanted to name my shaders and their variables (in Rust) with his name: accent and all.
- `[feature(core_intrinsics)]`: This will never be stable, but we drop a breakpoint in the panic handler. This could be removed, and instead call a process termination routine from `winapi`.

I used a specific nightly from May 2019, so that is likely your best bet if the current nightly doesn't build anymore.
```
rustup toolchain add nightly-2019-05-25
```

After you have you Rust setup, it's a straight forward vanilla build:
```
cargo +nightly-2019-05-25 build
```

The default build includes logging, but this can be disabled with `--no-default-features`.

Also included are Windows scripts that build for x86 and x64:
```
set CARGO_TARGET_DIR=target # or w/e
.\tools\package.cmd
```
This will package everything in a datetime stamped `dist/p11e-*` folder.

### Code Layout

A brief summary of the architecture of the code. This design isn't all that bad and I may want to reuse it, so I document it here for future me and anyone reading this.

#### Source Files

See `Cargo.toml` for a list of dependencies needed to build. These are handled
by Cargo, but the list is interesting on its own:
```
[dependencies]
libc = "0.2.54"
libm = "0.1"
cfg-if = "0.1"
nalgebra = "0.18"
winapi = '0.3'
```
It would be nice to not need any of those (except `winapi`, which really can't be helped).

In terms of stuff I wrote, generated, or copied:
```bash
    # Compiles C code in `loader/` and `fft/`
    build.rs

    # Rust Source
    src/
        # OpenGL Loader and bindings
        #   These are generated from .\tools\generate_oglrs.cmd, and then tweaked.
        #       1. x86 needs these as extern "system", not extern "C".
        #          I'm not aware of a way to do this with bindgen
        #       2. I call the function pointers directly
        #           - This was a design mistake
        #           - I manually convert the Option<fn()> types into fn() types
        #   Stuff was added as I needed it.
        #   Handling the whole `dev_build` feature is done manually too....
        ogl.rs

        # Audio code
        #    This uses the win32 function waveOutWrite() and friends to generate
        #    a buffer of audio and play it on start.
        #    It's called from `demo_main()`.
        #    The interesting part is `write_song()`, which makes noise out of
        #    nothing using sawrooth wave forms.
        #    If you do audio, this is pretty basic. Lots of learning happened
        #    in writing this code.
        audio.rs

        # Debug only ("dev_build") functions and macros.
        #    Without std, we lost println!() and friends, so I added them back.
        #    This also has a dummy type that implements core::fmt::Write,
        #    so that anything we can use in println!() can go to `stdout` or
        #    `OutputDebugString`.
        #    This entire file should be no-op and not compiled without the
        #    "dev_build" feature enabled.
        debug.rs

        # Rust runtime requirements - mostly panic handling.
        rt.rs

        # Entry point (`demo_main()`), window/ogl initialization,
        #    and the render loop.
        main.rs

        # Manual bindings to the fft source code under `./fft`.
        #   Unused.
        fft.rs

    # OpenGL GLSL shaders
    glsl/
        # Development shaders.
        # Nothing fancy - just a lot of hardcoded constants.
        sérusier.frag
        sérusier.vert

        # Same as development shaders, but manually minified.
        # I removed excess spaces and newlines, comments, and added a couple of
        # `#define` for common types.
        # Doing this saved like 6k, and made them smaller than using SPIRV.
        min.sérusier.frag
        min.sérusier.vert

    # OpenGL loader in C. Loads the appropriate library and populates a structure
    #    with the function pointers. There's pretty basic error checking.
    loader/
        # Computer generated header with OpenGL4.6 defines.
        #   I love glad for this stuff, since I don't trust system headers or
        #   their inconsistent naming and pathing.
        glad_46.h

        # OpenGL Loader header. This is used to generate `ogl.rs` with bindgen.
        #   It defines the loader struct, which houses all of the pfns.
        ogl.h
        # Implements the loader.
        #   This is in C instead of Rust because I wanted to minimize what
        #   I needed to generate. In hindsight, I don't know what this saved me.
        #   We enable depth buffer in here too.
        ogl.c

    # Misc scripts that I ran enough to write down.
    #    They are written to be run form the root folder:
    #       `> .\tools\package.cmd`
    tools/
        # I would use this with cargo-watch: https://github.com/passcod/cargo-watch
        #   `cargo watch -c -x ./tools/watch.cmd`
        watch.cmd

        # Runs bindgen on the glad header mentioned above.
        #   Useful for speeding the process of adding a new OGL function...
        #   but that process still sucked.
        generate_oglrs.cmd

        # Packages the important release builds into a datetime stamped folder.
        #   These can be zipped and moved to another computer.
        #   They include four binaries:
        #       x86 vs x64          - target architecture, x86 is smaller
        #       quiet vs dev_build  - Enable logging, which bloats the binary
        package.cmd

    # Unused Fast Fourier Transform in C.
    #   The plan was to implement PaulStretch on gm_dls for audio, but this was
    #   dropped in favor of sawtooth waveforms (because they were easier to do).
    #   It was sourced from: https://www.nayuki.io/page/free-small-fft-in-multiple-languages
    fft/
        fft.h
        fft.c
