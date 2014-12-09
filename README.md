obj-rs [![][version-i]][crates] [![][buildstat-i]][travis]
========

A parser for wavefront `.obj` and `.mtl` file format.

There are informative error messages with line number reporting.
The full file format is not supported *yet*. Patches for adding support for
unimplemented parts of the format are very welcome.

###### References

- [.obj spec](http://www.martinreddy.net/gfx/3d/OBJ.spec)
- [.mtl spec](http://www.fileformat.info/format/material)
- [PistonDevelopers/wavefront_obj](//github.com/PistonDevelopers/wavefront_obj)
- [csherratt/obj]                 (//github.com/csherratt/obj)
- [syoyo/tinyobjloader]           (//github.com/syoyo/tinyobjloader)

--------

[BSD 2-Clause](LICENSE.md)

[crates]:       //crates.io/crates/obj-rs
[travis]:       //travis-ci.org/simnalamburt/obj-rs

[version-i]:    https://img.shields.io/badge/cargo-v0.0.4-red.svg?style=flat
[buildstat-i]:  https://img.shields.io/travis/simnalamburt/obj-rs/master.svg?style=flat
