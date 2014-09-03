extern crate beef;

use std::os;
use std::io::{BufferedWriter,File};

use beef::parser;
use beef::compiler;

fn usage() {
    let args = os::args();
    println!("Usage: {} <filename> <outfile>", args.get(0));
}

fn parse_and_compile(filename: &str, outfile: &str) {
    let mut out = File::create(&Path::new(outfile));
    match parser::parse_file(filename) {
        Some(program) => compiler::compile(program.as_slice(), &mut out),
        None => fail!("Failed to parse {}", filename)
    }

    println!("Wrote output to {} \\\\o/", outfile);
    println!("Maybe try something like:");
    println!("  nasm -f macho32 {} -o {}.o", outfile, outfile);
    println!("  ld -macosx_version_min 10.7.0 -lSystem -o {}.out {}.o", outfile, outfile);
    println!("");
    println!("(You better believe macosx, I hardcoded the syscalls)");
}

fn main() {
    let args = os::args();
    match args.len() {
        0 => unreachable!(),
        3 => parse_and_compile(args[1].as_slice(), args[2].as_slice()),
        _ => usage(),
    }
}
