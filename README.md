# Rust implementation of libm

## Folder architecture

The folder structure is inspired by librsvg, which succesfully port a c project to rust keeping a compatible interface
https://gitlab.gnome.org/GNOME/librsvg

- libm : source code expositon a c ABI libm compatible version
- libm_internals : implementation of math function in rust
- libm_crate : rust libm crate

## TODO

use git submodules for test
use docker for tests ?
Check FLT_EVAL method as feature(rint)
proper testing : remove path to get static lib
make it no_std again

## Notes

try the relibc architecture and build

Internal should use result istead of errno like
Openlibm deprecated ?
https://github.com/JuliaLang/julia/issues/18102

Take a look at sleef (SIMD libm)
May be directly integrated in llvm ?

Try and merge all current PR on GitHub in my fork
Merge 153

Port newlib for f32 ?

Need benchmark, use test ?
Quick check
fuzzing

split 32 bit and 64 bit impl for calrity

newlib
./configure --host=i686-pc-linux --with-newlib && make
new lib may need cross compile to be tested (x86_64 not suported)

#define LDBL_EPSILON
