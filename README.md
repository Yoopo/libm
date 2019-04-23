# Rust implementation of libm

## Folder architecture

The folder structure is inspired by librsvg, which succesfully port a c project to rust keeping a compatible interface
https://gitlab.gnome.org/GNOME/librsvg

* libm : source code expositon a c ABI libm compatible version
* libm_internals : implementation of math function in rust
* libm_crate : rust libm crate

## Notes

try the relibc architecture and build
