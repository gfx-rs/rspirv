rspirv
======

[![Build Status](https://travis-ci.org/google/rspirv.svg?branch=master)](https://travis-ci.org/google/rspirv)
[![Build status](https://ci.appveyor.com/api/projects/status/qc9p4bjrbw4osyho/branch/master?svg=true)](https://ci.appveyor.com/project/antiagainst/rspirv/branch/master)

**R**u**s**t implementation of S**PIR**-**V** module processing functionalities.
It aims to provide:

* APIs for processing SPIR-V modules
* Command line tools building on top of the APIs for common processing tasks

rspirv defines a common SPIR-V [memory representation][doc-mr] (MR) as the
medium for various purposes. rspirv also provides a [builder][doc-builder] to
build the MR iteractively and a [parser][doc-parser] to parse a given SPIR-V
binary module into its MR.

[SPIR-V][spirv] is a common intermediate language for representing graphics
shaders and compute kernels for multiple Khronos APIs, such as [Vulkan][vulkan],
[OpenGL][opengl], and [OpenCL][opencl].

[SPIRV-Tools][spirv-tools] is the Khronos Group's official C++ implementation of
SPIR-V binary parser, assembler, disassembler, optimizer, and validator. rspirv
is not a Rust language binding for that project; it is a complete rewrite using
Rust.

### Disclaimer

This is not an official Google product (experimental or otherwise), it is just
code that happens to be owned by Google.

Documentation
-------------

The current implementation supports SPIR-V 1.1 (Revision 5).

Multiple crates are published from this project:

|      Name      |   Crate   |   Docs   |
| :------------: | :-------: | :------: |
| rspirv         | [![Crate][img-crate-rspirv]][crate-rspirv]   | [![Documentation][img-doc-rspirv]][doc-rspirv]   |
| spirv\_headers | [![Crate][img-crate-headers]][crate-headers] | [![Documentation][img-doc-headers]][doc-headers] |

In total rspirv APIs contains:
* The [SPIR-V header][doc-headers] (all SPIR-V structs, enums, and constants)
* The whole [SPIR-V grammar][doc-grammar] (instruction layouts and their
  operands)
* A [memory representation][doc-mr] of SPIR-V modules and its loader and builder
* SPIR-V [binary][doc-binary] module decoding and parsing functionalities

The Khronos SPIR-V [JSON grammar][json-grammar] is leveraged to generate parts
of the source code using Cargo [build scripts](codegen).

Please see the links to docs.rs for detailed documentation.

Status
------

I plan to implement serveral functionalities:

- [x] SPIR-V [memory representation][doc-mr] (MR)
- [x] SPIR-V module builder
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

Examples
--------

Building a SPIR-V module, assembling it, parsing it, and then disassembling it:

```rust
extern crate rspirv;
extern crate spirv_headers as spirv;

use rspirv::binary::Assemble;
use rspirv::binary::Disassemble;

fn main() {
    // Building
    let mut b = rspirv::mr::Builder::new();
    b.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::GLSL450);
    let void = b.type_void();
    let voidf = b.type_function(void, vec![void]);
    b.begin_function(void,
                     None,
                     (spirv::FUNCTION_CONTROL_DONT_INLINE |
                      spirv::FUNCTION_CONTROL_CONST),
                     voidf)
     .unwrap();
    b.begin_basic_block(None).unwrap();
    b.ret().unwrap();
    b.end_function().unwrap();
    let module = b.module();

    // Assembling
    let code = module.assemble();
    assert!(code.len() > 20);  // Module header contains 5 words
    assert_eq!(spirv::MAGIC_NUMBER, code[0]);

    // Parsing
    let mut loader = rspirv::mr::Loader::new();
    rspirv::binary::parse_words(&code, &mut loader).unwrap();
    let module = loader.module();

    // Disassembling
    assert_eq!(module.disassemble(),
               "; SPIR-V\n\
                ; Version: 1.1\n\
                ; Generator: Unknown\n\
                ; Bound: 5\n\
                OpMemoryModel Logical GLSL450\n\
                %1 = OpTypeVoid\n\
                %2 = OpTypeFunction %1 %1\n\
                %3 = OpFunction  %1  DontInline|Const %2\n\
                %4 = OpLabel\n\
                OpReturn\n\
                OpFunctionEnd");
}
```

Contributions
-------------

This project is licensed under the [Apache 2](LICENSE) license. Please see
[CONTRIBUTING](CONTRIBUTING.md) before contributing.

### Authors

This project is initialized and mainly developed by Lei Zhang
([@antiagainst][me]).

[img-crate-rspirv]: https://img.shields.io/crates/v/rspirv.svg
[img-doc-rspirv]: https://docs.rs/rspirv/badge.svg
[crate-rspirv]: https://crates.io/crates/rspirv
[doc-rspirv]: https://docs.rs/rspirv
[img-crate-headers]: https://img.shields.io/crates/v/spirv_headers.svg
[img-doc-headers]: https://docs.rs/spirv_headers/badge.svg
[crate-headers]: https://crates.io/crates/spirv_headers
[doc-headers]: https://docs.rs/spirv_headers
[spirv]: https://www.khronos.org/registry/spir-v/
[vulkan]: https://www.khronos.org/vulkan/
[opengl]: https://www.opengl.org/
[opencl]: https://www.khronos.org/opencl/
[me]: https://github.com/antiagainst
[json-grammar]: https://github.com/KhronosGroup/SPIRV-Headers/tree/master/include/spirv
[spirv-tools]: https://github.com/KhronosGroup/SPIRV-Tools
[doc-mr]: https://docs.rs/rspirv/*/rspirv/mr/index.html
[doc-builder]: https://docs.rs/rspirv/*/rspirv/mr/struct.Builder.html
[doc-parser]: https://docs.rs/rspirv/*/rspirv/binary/struct.Parser.html
[doc-grammar]: https://docs.rs/rspirv/*/rspirv/grammar/index.html
[doc-binary]: https://docs.rs/rspirv/*/rspirv/binary/index.html
[rust-1.15]: https://blog.rust-lang.org/2017/02/02/Rust-1.15.html
