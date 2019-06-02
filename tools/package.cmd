@cargo check || goto :EOF

@echo off
for /f "delims=/ tokens=1-3" %%a in ("%DATE:~4%") do (
    for /f "delims=:. tokens=1-4" %%m in ("%TIME: =0%") do (
        set DIST_DIR=dist\p11e-%%c-%%b-%%a-%%m%%n%%o%%p
    )
)
@echo on

@echo CARGO_TARGET_DIR=%CARGO_TARGET_DIR%
@mkdir dist
@mkdir %DIST_DIR%

cargo build --target=x86_64-pc-windows-msvc --release
@cp -v %CARGO_TARGET_DIR%/x86_64-pc-windows-msvc/release/p11e.exe  %DIST_DIR%/p11e-error-x64.exe

cargo build --target=i686-pc-windows-msvc   --release
@cp -v %CARGO_TARGET_DIR%/i686-pc-windows-msvc/release/p11e.exe    %DIST_DIR%/p11e-error-x86.exe

cargo build --target=x86_64-pc-windows-msvc --release --no-default-features
@cp -v %CARGO_TARGET_DIR%/x86_64-pc-windows-msvc/release/p11e.exe  %DIST_DIR%/p11e-quiet-x64.exe

cargo build --target=i686-pc-windows-msvc   --release --no-default-features
@cp -v %CARGO_TARGET_DIR%/i686-pc-windows-msvc/release/p11e.exe    %DIST_DIR%/p11e-quiet-x86.exe

@rem This one's usually the smallest
@cp -v %DIST_DIR%/p11e-quiet-x86 %DIST_DIR%/p11e-compo.exe

@echo --- Sizes ---
@find %DIST_DIR% -type f -exec stat -c "%%-60n -- %%s bytes" {} ;
