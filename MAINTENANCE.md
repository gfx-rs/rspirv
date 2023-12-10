# Prerequisites

`rspirv` uses `cargo-release` for release management. If you don't have `cargo-release` installed run `cargo install cargo-release`. Publishing to crates.io is handled by a github workflow.

# Making a new release

Order of releases crates is important to prevent breakage. `spirv` should released before `rspirv` because it has an obvious dependency.

In the corresponding crate directory execute `cargo release <version> -x`.
