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
    let file = File::open(&Path::new(outfile)).unwrap();
    let mut out = BufferedWriter::new(file);
    match parser::parse_file(filename) {
        Some(program) => compiler::compile(program.as_slice(), &mut out),
        None => fail!("Failed to parse {}", filename)
    }
}

fn main() {
    let args = os::args();
    match args.len() {
        0 => unreachable!(),
        3 => parse_and_compile(*args.get(1), *args.get(2)),
        _ => usage(),
    }
}
