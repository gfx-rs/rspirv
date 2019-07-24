// Copyright 2016 Google Inc.
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

use std::fs;
use std::io::Read;

use rspirv::binary::Disassemble;

fn main() {
    let matches = clap::App::new("rspirv-dis")
        .version(env!("CARGO_PKG_VERSION"))
        .about("SPIR-V binary module disassembler from the rspirv project")
        .arg(clap::Arg::with_name("input").index(1).required(true))
        .get_matches();

    let input = matches.value_of("input").unwrap();
    let mut f = fs::File::open(input).expect("cannot open file");
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).expect("cannot read file");

    match rspirv::mr::load_bytes(&buffer) {
        Ok(module) => println!("{}", module.disassemble()),
        Err(err) => println!("{}", err),
    }
}
