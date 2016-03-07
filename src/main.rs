extern crate getopts;
extern crate nom;
extern crate rspirv;

use nom::Producer;
use std::env;

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

    let mut p = nom::FileProducer::new(input, 1024 * 5).unwrap();
    let mut c = rspirv::binary::Reader::new();

    loop {
        match p.apply(&mut c) {
            &nom::ConsumerState::Done(_, _) => {
                println!("Succeed.");
                break;
            }
            &nom::ConsumerState::Error(_) => {
                println!("Failed.");
                break;
            }
            &nom::ConsumerState::Continue(_) => continue,
        }
    }
}
