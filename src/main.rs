extern crate getopts;
extern crate rspirv;

use std::env;
use std::fs;
use std::io::Read;

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

    let mut reader = rspirv::binary::Reader::new();
    let module = reader.process(buffer);
    println!("{:#?}", module);
}
