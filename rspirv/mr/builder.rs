// Copyright 2017 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(dead_code)]

use mr;
use spirv;

/// The memory representation builder.
///
/// Constructs a [`Module`](struct.Module.html) by aggregating results from
/// method calls for various instructions.
pub struct Builder {
    module: mr::Module,
    next_id: u32,
}

impl Builder {
    /// Creates a new empty builder.
    pub fn new() -> Builder {
        Builder {
            module: mr::Module::new(),
            next_id: 1,
        }
    }
}

include!("build_type.rs");
