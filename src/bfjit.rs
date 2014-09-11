extern crate beef;

use std::os;

use beef::parser;
use beef::compiler;
use beef::context;
use beef::jit;

fn usage() {
    let args = os::args();
    println!("Usage: {} <filename>", args.get(0));
}

fn parse_and_exec(filename: &str) {
    let program = match parser::parse_file(filename) {
        Some(program) => program,
        None => fail!("Failed to parse {}", filename),
    };

    let entry_point = jit::load(program, context::TAPE_WIDTH);
}

fn main() {
    let args = os::args();
    match args.len() {
        0 => unreachable!(),
        2 => parse_and_exec(args[1].as_slice()),
        _ => usage(),
    }
}
