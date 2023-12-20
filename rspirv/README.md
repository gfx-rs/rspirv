core of the rspirv project
==========================

[![Crate][img-crate-rspirv]][crate-rspirv]
[![Documentation][img-doc-rspirv]][doc-rspirv]

The core crate of the rspirv project providing APIs for processing SPIR-V
modules:

* The whole [SPIR-V grammar][doc-grammar] (instruction layouts and their
  operands)
* A [data representation][doc-dr] of SPIR-V modules and its loader and builder
* A structured representation of SPIR-V modules (under developing)
* SPIR-V [binary][doc-binary] module decoding and parsing functionalities
* A [lifting infrastructure][doc-lift] for converting data representation into
  structured representation.

This crate defines a common SPIR-V [data representation][doc-dr] (DR) as the
medium for various purposes. It also provides a [builder][doc-builder] to
build the DR interactively and a [parser][doc-parser] to parse a given SPIR-V
binary module into its DR.
The [parser][doc-parser] handles decoding and parsing of SPIR-V binary modules
according to the [grammar][doc-grammar], the parsed instructions are sent to
the [consumer][doc-consumer].

The data representation, as the name shows, focuses on presenting the data
within a SPIR-V module; a higher level structured representation is currently
under developing.

Usage
-----

First add to your `Cargo.toml`:

```toml
[dependencies]
rspirv = "0.12.0"
```

Examples
--------

Please see the [documentation][doc-rspirv] and project's
[README][project-readme] for examples.

[img-crate-rspirv]: https://img.shields.io/crates/v/rspirv.svg
[img-doc-rspirv]: https://docs.rs/rspirv/badge.svg
[crate-rspirv]: https://crates.io/crates/rspirv
[doc-rspirv]: https://docs.rs/rspirv
[project-readme]: https://github.com/gfx-rs/rspirv/blob/master/README.md
[doc-grammar]: https://docs.rs/rspirv/*/rspirv/grammar/index.html
[doc-dr]: https://docs.rs/rspirv/*/rspirv/dr/index.html
[doc-lift]: https://docs.rs/rspirv/*/rspirv/lift/index.html
[doc-builder]: https://docs.rs/rspirv/*/rspirv/dr/struct.Builder.html
[doc-binary]: https://docs.rs/rspirv/*/rspirv/binary/index.html
[doc-parser]: https://docs.rs/rspirv/*/rspirv/binary/struct.Parser.html
[doc-consumer]: https://docs.rs/rspirv/*/rspirv/binary/trait.Consumer.html
