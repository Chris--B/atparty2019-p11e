@cargo fmt   || goto :EOF
@cargo check || goto :EOF

cargo build --target=x86_64-pc-windows-msvc
cargo build --target=i686-pc-windows-msvc
cargo build --release --target=x86_64-pc-windows-msvc
cargo build --release --target=i686-pc-windows-msvc

@echo --- Sizes ---
@echo %cd%
@find -name p11e.exe -exec stat -c "%%-60n -- %%s bytes" {} ;
cargo run --target=i686-pc-windows-msvc
