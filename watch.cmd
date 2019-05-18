@echo off
cargo fmt || goto :EOF
cargo run --release || goto :EOF
post-build.cmd
