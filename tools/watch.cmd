@echo off
cargo fmt || goto :EOF
cargo build --release || goto :EOF
stat -c "%%n -- %%s bytes" %CARGO_TARGET_DIR%\release\p11e.exe
cargo run --release
