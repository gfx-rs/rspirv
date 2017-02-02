rspirv
======

[![Version](https://img.shields.io/crates/v/rspirv.svg)](https://crates.io/crates/rspirv)
[![Documentation](https://docs.rs/rspirv/badge.svg)](https://docs.rs/rspirv)
[![Build Status](https://travis-ci.org/google/rspirv.svg?branch=master)](https://travis-ci.org/google/rspirv)
[![Build status](https://ci.appveyor.com/api/projects/status/qc9p4bjrbw4osyho/branch/master?svg=true)](https://ci.appveyor.com/project/antiagainst/rspirv/branch/master)

**R**u**s**t implementation of S**PIR**-**V** module processing functionalities.
It aims to provide:

* APIs for processing SPIR-V modules
* Command line tools building on top of the APIs for common processing tasks

[SPIR-V][spirv] is a common intermediate language for representing graphics
shaders and compute kernels for multiple Khronos APIs, such as [Vulkan][vulkan],
[OpenGL][opengl], and [OpenCL][opencl].

rspirv defines a common SPIR-V [memory representation][doc-mr] (MR) as the
medium for various purposes. rspirv also provides a [parser][doc-parser] to
parse a given SPIR-V binary module into its MR.

[SPIRV-Tools][spirv-tools] is the Khronos Group's official C++ implementation of
SPIR-V binary parser, assembler, disassembler, optimizer, and validator. rspirv
is not a Rust language binding for that project; it is a complete rewrite using
Rust.

### Disclaimer

This is not an official Google product (experimental or otherwise), it is just
code that happens to be owned by Google.

Documentation
-------------

The current implementation supports SPIR-V 1.1 (Revision 4).

rspirv APIs contains:
* The [SPIR-V header][doc-header] (all SPIR-V structs, enums, and constants)
* The whole [SPIR-V grammar][doc-grammar] (instruction layouts and their
  operands)
* A [memory representation][doc-mr] of SPIR-V modules
* A SPIR-V [binary][doc-binary] loading and parsing Rust module

The [parser][doc-parser] handles decoding and parsing of SPIR-V binary modules
according to the [grammar][doc-grammar], the parsed instructions are sent to
the [consumer][doc-consumer].

The Khronos SPIR-V [JSON grammar][json-grammar] is leveraged to generate parts
of the source code using [`build.rs`](rspirv/build.rs).

Detailed documentation about the APIs in this crate is available at
[![Documentation](https://docs.rs/rspirv/badge.svg)](https://docs.rs/rspirv).

Status
------

I plan to implement serveral functionalities:

- [x] SPIR-V [memory representation][doc-mr] (MR)
- [ ] SPIR-V module builder
- [ ] SPIR-V module assemebler
- [x] SPIR-V binary module [parser][doc-parser]
- [x] SPIR-V binary module disassemebler
- [ ] HLSL/GLSL to SPIR-V frontend (maybe)
- [ ] SPIR-V MR to LLVM IR transformation (maybe)

The SPIR-V binary module parser is almost feature complete; the only feature
(that I am aware of) missing is 64-bit selectors in `OpSwitch`.

Usage
-----

This project uses custom derive, which became available in the stable channel
since [1.15][rust-1.15]. So to compile with a compiler from the stable channel,
please make sure that the version is >= 1.15.

First add to your `Cargo.toml`:

```toml
[dependencies]
rspirv = "0.2"
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

let buffer: Vec<u8> = vec![
    // Magic number.           Version number: 1.0.
    0x03, 0x02, 0x23, 0x07,    0x00, 0x00, 0x01, 0x00,
    // Generator number: 0.    Bound: 0.
    0x00, 0x00, 0x00, 0x00,    0x00, 0x00, 0x00, 0x00,
    // Reserved word: 0.
    0x00, 0x00, 0x00, 0x00,
    // OpMemoryModel.          Logical.
    0x0e, 0x00, 0x03, 0x00,    0x00, 0x00, 0x00, 0x00,
    // GLSL450.
    0x01, 0x00, 0x00, 0x00];

match rspirv::mr::load(&buffer) {
    Ok(module) => println!("{}", module.disassemble()),
    Err(err) => println!("{}", err),
}

// Output:
//
// ; SPIRV
// ; Version: 1.0
// ; Generator: Khronos Group
// ; Bound: 0
// OpMemoryModel Logical GLSL450
```

Parsing a SPIR-V binary module:

```rust
use rspirv::binary::Parser;
use rspirv::mr::{Loader, Operand};
use rspirv::spirv::{AddressingModel, MemoryModel};

let bin = vec![
    // Magic number.           Version number: 1.0.
    0x03, 0x02, 0x23, 0x07,    0x00, 0x00, 0x01, 0x00,
    // Generator number: 0.    Bound: 0.
    0x00, 0x00, 0x00, 0x00,    0x00, 0x00, 0x00, 0x00,
    // Reserved word: 0.
    0x00, 0x00, 0x00, 0x00,
    // OpMemoryModel.          Logical.
    0x0e, 0x00, 0x03, 0x00,    0x00, 0x00, 0x00, 0x00,
    // GLSL450.
    0x01, 0x00, 0x00, 0x00];
let mut loader = Loader::new();  // You can use your own consumer here.
{
    let mut p = Parser::new(&bin, &mut loader);
    p.parse().unwrap();
}
let module = loader.module();

assert_eq!((1, 0), module.header.unwrap().version());
let m = module.memory_model.as_ref().unwrap();
assert_eq!(Operand::AddressingModel(AddressingModel::Logical),
           m.operands[0]);
assert_eq!(Operand::MemoryModel(MemoryModel::GLSL450),
           m.operands[1]);
```

Contributions
-------------

This project is licensed under the [Apache 2](LICENSE) license. Please see
[CONTRIBUTING](CONTRIBUTING.md) before contributing.

### Authors

This project is initialized and mainly developed by Lei Zhang
([@antiagainst][me]).

[spirv]: https://www.khronos.org/registry/spir-v/
[vulkan]: https://www.khronos.org/vulkan/
[opengl]: https://www.opengl.org/
[opencl]: https://www.khronos.org/opencl/
[me]: https://github.com/antiagainst
[json-grammar]: https://github.com/KhronosGroup/SPIRV-Headers/tree/master/include/spirv
[spirv-tools]: https://github.com/KhronosGroup/SPIRV-Tools
[doc-mr]: https://docs.rs/rspirv/0.2.0/rspirv/mr/index.html
[doc-parser]: https://docs.rs/rspirv/0.2.0/rspirv/binary/struct.Parser.html
[doc-header]: https://docs.rs/rspirv/0.2.0/rspirv/spirv/index.html
[doc-grammar]: https://docs.rs/rspirv/0.2.0/rspirv/grammar/index.html
[doc-binary]: https://docs.rs/rspirv/0.2.0/rspirv/binary/index.html
[doc-consumer]: https://docs.rs/rspirv/0.2.0/rspirv/binary/trait.Consumer.html
[rust-1.15]: https://blog.rust-lang.org/2017/02/02/Rust-1.15.html
