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

extern crate getopts;
extern crate rspirv;

use std::env;
use std::fs;
use std::io::Read;

use rspirv::binary::Disassemble;

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [options] <spirv-binary>", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "Print help message.");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let input = if matches.free.len() == 1 {
        &matches.free[0]
    } else {
        print_usage(&program, opts);
        return;
    };

    let mut f = fs::File::open(input).unwrap();
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).unwrap();

    match rspirv::mr::load(buffer) {
        Ok(module) => println!("{}", module.disassemble()),
        Err(err) => println!("{}", err),
    }
}
