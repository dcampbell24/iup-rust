IUP Rust [![Build Status](https://travis-ci.org/dcampbell24/iup-rust.svg)](https://travis-ci.org/dcampbell24/iup-rust)[![Join the chat at https://gitter.im/dcampbell24/iup-rust](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/dcampbell24/iup-rust?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
=======================================

This library provides a high level wrapper around [IUP][1], a multi-platform
toolkit for building graphical user interfaces. See [rust-iup-sys](https://github.com/dcampbell24/rust-iup-sys) for low level bindings.

[IUP][1] is a multi-platform toolkit for building graphical user interfaces.

It's purpose is to allow a program to run in different systems without changes - the toolkit
provides the application portability. Supported systems include: GTK+, Motif and Windows.

IUP has some advantages over other interface toolkits available:

 + **Simplicity:** due to the small number of functions and to its attribute mechanism,
   the learning curve for a new user is often faster.
 + **Portability:** the same functions are implemented in each one of the platforms, thus
   assuring the interface system's portability.
 + **Customization:** the dialog specification language (LED) is a mechanisms in which it
   is possible to customize an application for a specific user with a simple-syntax text file.
 + **Flexibility:** its abstract layout mechanism provides flexibility to dialog creation.
 + **Extensibility:** the programmer can create new interface elements as needed.

The Rust binding provides a way to do things in a more Rustic way but without moving out of
IUP base nameclatures and philosophy in such a way that one can program on this binding by reading the
original [IUP documentation][1].

[Documentation](http://dcampbell24.github.io/iup-rust/)
---------------

Click the link above or run `cargo doc` on this repository to view the documentation locally.

Installation
------------

See [rust-iup-sys](https://github.com/dcampbell24/rust-iup-sys) for
information on installing the IUP system libraries needed to use this library.
After you have the IUP system libraries just add this to your `Cargo.toml`:

    [dependencies.iup]
    git = "https://github.com/dcampbell24/iup-rust"

Contribute
----------

Contributions are welcome both in the form of ideas and of code. If you want to work on something, please open a issue to let others know what you are working on. If you are not sure what to work on, check our issues to see what must be worked on.

If you find any issues with the library, please create a GitHub issue for it.

[1]: http://www.tecgraf.puc-rio.br/iup/
