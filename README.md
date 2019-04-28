# Rust implementation of libm

## Folder architecture

The folder structure is inspired by librsvg, which succesfully port a c project to rust keeping a compatible interface
https://gitlab.gnome.org/GNOME/librsvg

- libm : source code expositon a c ABI libm compatible version
- libm_internals : implementation of math function in rust
- libm_crate : rust libm crate

## TODO

- backport llvm intrinsic for wasm

  - fork and check if rustc wasm atrget is borken
  - https://github.com/rust-lang-nursery/compiler-builtins/pull/248

use git submodules for test
Do not compare staticlink Vs dynamic lib in bench for fair testing
Compare static link libm Vs dynlibm (rust carte will static link our math lib it may be an argument for it)
use docker for tests ?
Check FLT_EVAL method as feature(rint)
proper testing : remove path to get static lib
make relibm no std
restore original comment
backport last change from musl
continue to document change in readme or in a separate doc
mark musl last updated commit hash in every function
backport the intrinsic cfg in rust crate (may us a cfg in lib which use intrinsic)
name the c libm relibm ?
Use f32::EPSILON; instead of EPS ?
backport japardic old test aka no std test
script that monitor commit on musl an alert at compile time of deprecation
Use unstable flag to hide untested method (cuda, ...)
clean up feature flags to simplify modularity (musl Vs newlib with or with out intrinsic / cuda)
test all feature combinations
add new lib test

## Notes

Should relibm be no std (panic handler ?) => YES
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
Quick check against libm
Proptest
Pallette use libm for no std
Use rust cont instead of #define LDBL_EPSILON

Is cuda llvm.nvvm.sin.approx.f equivalent to use llvm llvm.sin.f32
will llvm choose it on cuda target ?

Check clang fast-math-flag ‘afn’

- How do we enable it as feature in libm
- https://github.com/rust-lang/rust/issues/40063

libm origine ?
https://github.com/rustwasm/team/issues/84
