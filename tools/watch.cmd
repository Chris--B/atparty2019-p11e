@cargo fmt   || goto :EOF
@cargo check || goto :EOF

rem cargo build --target=x86_64-pc-windows-msvc
rem cargo build --target=i686-pc-windows-msvc
cargo build --target=x86_64-pc-windows-msvc --release
cargo build --target=i686-pc-windows-msvc   --release

@echo --- Sizes ---
@echo %cd%
@find -type f -name p11e.exe -exec stat -c "%%-60n -- %%s bytes" {} ;
cargo run --target=i686-pc-windows-msvc
