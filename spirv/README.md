spirv-headers of the rspirv project
===================================

[![Crate][img-crate-headers]][crate-headers]
[![Documentation][img-doc-headers]][doc-headers]

The headers crate for the rspirv project which provides Rust definitions of
SPIR-V structs, enums, and constants.

Usage
-----

First add to your `Cargo.toml`:

```toml
[dependencies]
rspirv_headers = "1.1"
```

Then add to your crate root:

```rust
extern crate spirv_headers;
```

Examples
--------

Please see the [documentation][doc-headers] and project'sx
[README][project-readme] for examples.

[img-crate-headers]: https://img.shields.io/crates/v/spirv_headers.svg
[img-doc-headers]: https://docs.rs/spirv_headers/badge.svg
[crate-headers]: https://crates.io/crates/spirv_headers
[doc-headers]: https://docs.rs/spirv_headers
[project-readme]: https://github.com/google/rspirv/blob/master/README.md
