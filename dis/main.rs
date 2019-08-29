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

    match rspirv::dr::load_bytes(&buffer) {
        Ok(module) => println!("{}", module.disassemble()),
        Err(err) => println!("{}", err),
    }
}
