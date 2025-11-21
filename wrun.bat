@echo off

set EJR_INCLUDE_DIR="C:\msys64\mingw64\include"
cargo run --target x86_64-pc-windows-gnu -- %*