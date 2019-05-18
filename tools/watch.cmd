@echo off
cargo fmt || goto :EOF
cargo run --release || goto :EOF
tools/post-build.cmd
