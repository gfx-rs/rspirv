# Making a new release

1. If you don't have cargo-release installed run `cargo install cargo-release`
1. For `rspirv` run `cargo release` in the `rspriv` directory, this should follow Semantic Versioning
1. For `spirv_headers` we don't follow Semantic Versioning, instead we follow the versioning SPIR-V uses, and reserve the patch version to do our own version increments.