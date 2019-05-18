@echo off

stat -c "%%n -- %%s bytes" %CARGO_TARGET_DIR%\release\p11e.exe
