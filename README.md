rspirv
======

[![Build Status](https://travis-ci.org/google/rspirv.svg?branch=master)](https://travis-ci.org/google/rspirv)
[![Version](https://img.shields.io/crates/v/rspirv.svg)](https://crates.io/crates/rspirv)


**R**u**s**t implementation of S**PIR**-**V** module processing
functionalities. It aims to provide:

* APIs for processing SPIR-V modules
* Command line tools building on top of the APIs for common processing tasks

[SPIR-V][spirv] is a common intermediate language for representing graphics
shaders and compute kernels for multiple Khronos APIs, such as [Vulkan][vulkan],
[OpenGL][opengl], and [OpenCL][opencl].

### Disclaimer

This is not an official Google product (experimental or otherwise), it is just
code that happens to be owned by Google.

Documentation
-------------

[![Documentation](https://docs.rs/rspirv/badge.svg)](https://docs.rs/rspirv)

Status
------

This project is far from being complete; it's in a very early stage of
development. I plan to implement serveral functionalities:

- [x] SPIR-V memory representation (MR)
- [ ] SPIR-V module builder
- [ ] SPIR-V module assemebler
- [x] SPIR-V binary module loader
- [x] SPIR-V binary module disassemebler
- [ ] HLSL/GLSL to SPIR-V frontend (maybe)
- [ ] SPIR-V MR to LLVM IR transformation (maybe)

Right now the SPIR-V binary module disassemebler still lacks support for some
features like extended instruction sets and 64-bit selectors in `OpSwitch`.

Usage
-----

This project uses features only enabled in nightly versions of the Rust
compiler; so to build it, a nightly version of the compiler is required.

First add to your `Cargo.toml`:

```toml
[dependencies]
rspirv = "0.1"
```

Then add to your crate root:

```rust
extern crate rspirv;
```

Example
-------

Loading a SPIR-V binary module into memory and printing its disassembly:

```rust
use rspirv;
use rspirv::binary::Disassemble;

// buffer is a Vec<u8> containing the raw data of a SPIR-V binary module.

match rspirv::mr::load(buffer) {
    Ok(module) => println!("{}", module.disassemble()),
    Err(err) => println!("{}", err),
}
```

Contributions
-------------

This project is licensed under the [Apache 2](LICENSE) license. Please see
[CONTRIBUTING](CONTRIBUTING.md) before contributing.

### Authors

This project is initialized and mainly developed by Lei Zhang
([@antiagainst][antiagainst]).

[spirv]: https://www.khronos.org/registry/spir-v/
[vulkan]: https://www.khronos.org/vulkan/
[opengl]: https://www.opengl.org/
[opencl]: https://www.khronos.org/opencl/
[antiagainst]: https://github.com/antiagainst
