IUP-SYS Rust
=============

[![Crates.io](https://img.shields.io/crates/v/iup-sys.svg)](https://crates.io/crates/iup-sys)

These are low level Rust bindings to [IUP](http://webserver2.tecgraf.puc-rio.br/iup/).
Bindings to CD and IM are not included.

Installation
-------------

To use this library you must install IUP. See [download tips][2] for more
information. Note that this crate only links with IUP, not IM or CD.

[2]: http://www.tecgraf.puc-rio.br/iup/en/download_tips.html

After installing IUP, add this crate as a dependecny in your Cargo.toml file:
``` toml
[dependencies]
iup-sys = "0.0"
```

License
--------

This project is licensed under the MIT license. See LICENSE for the full license
and LICENSE-TECGRAF for the license of the IUP library this library binds to.
