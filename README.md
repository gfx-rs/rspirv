rspirv
======

[![Build Status](https://travis-ci.org/google/rspirv.svg?branch=master)](https://travis-ci.org/google/rspirv)

**R**u**s**t implementation of S**PIR**-**V** module processing
functionalities. It aims to provide:

* APIs for processing SPIR-V modules
* Command line tools building on top of the APIs for common processing tasks

[SPIR-V][spirv] is a common intermediate language for representing graphics
shaders and compute kernels for multiple Khronos APIs, such as Vulkan,
OpenGL, and OpenCL.

### Disclaimer

This is not an official Google product (experimental or otherwise), it is just
code that happens to be owned by Google.

Status
------

This project is far from being complete; it's in a very early stage of
development. However, the memory representation of SPIR-V modules is defined
and a SPIR-V binary module disassemebler is provided, although lacking the
support for some features like extended instruction sets and 64-bit selectors
in `OpSwitch`.

Getting and building rspirv
---------------------------

This project uses features only enabled in nightly versions of the Rust
compiler; so to build it, a nightly version of the compiler is required.

```sh
git clone git@github.com:google/rspirv.git

# Build the disassembler
cd rspirv/dis
Cargo build

# Build the core library and its doc
cd rspirv/rspirv
Cargo build
Cargo doc
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

### Authors

This project is initialized and mainly developed by Lei Zhang (@antiagainst).

[spirv]: https://www.khronos.org/registry/spir-v/
