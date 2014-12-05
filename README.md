obj-rs [![Cargo version][cargo-i]][cargo-a] [![Build Status][travis-i]][travis-a]
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

MIT License

[cargo-i]: https://img.shields.io/badge/cargo-0.0.2-yellow.svg
[cargo-a]: https://crates.io/crates/obj-rs
[travis-i]: https://travis-ci.org/simnalamburt/obj-rs.svg?branch=master
[travis-a]: https://travis-ci.org/simnalamburt/obj-rs
