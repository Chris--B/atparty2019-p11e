@cargo check || goto :EOF

@echo CARGO_TARGET_DIR=%CARGO_TARGET_DIR%
@mkdir dist

cargo build --target=x86_64-pc-windows-msvc --release
@cp -v %CARGO_TARGET_DIR%/x86_64-pc-windows-msvc/release/p11e.exe  dist/p11e-error-x64.exe

cargo build --target=i686-pc-windows-msvc   --release
@cp -v %CARGO_TARGET_DIR%/i686-pc-windows-msvc/release/p11e.exe    dist/p11e-error-x86.exe

cargo build --target=x86_64-pc-windows-msvc --release --no-default-features
@cp -v %CARGO_TARGET_DIR%/x86_64-pc-windows-msvc/release/p11e.exe  dist/p11e-quiet-x64.exe

cargo build --target=i686-pc-windows-msvc   --release --no-default-features
@cp -v %CARGO_TARGET_DIR%/i686-pc-windows-msvc/release/p11e.exe    dist/p11e-quiet-x86.exe

@rem This one's usually the smallest
@cp -v dist/p11e-quiet-x86 dist/p11e-compo.exe

@echo --- Sizes ---
@find dist -type f -exec stat -c "%%-60n -- %%s bytes" {} ;
