@echo off
bindgen loader\ogl.h^
    --whitelist-function ogl_load^
    --no-layout-tests^
    --use-core^
    --ctypes-prefix=libc^
    > src/ogl.rs
rem git diff -- src/ogl.rs
echo Now go remove all of the Option types on the functions in src/ogl.rs. :(
