extern crate beef;

use std::os;
use std::io::stdio::{stdout,stdin};

use beef::context::Context;
use beef::parser;
use beef::eval;

fn usage() {
    let args = os::args();
    println!("Usage: {} <filename>", args.get(0));
}

fn parse_and_eval(filename: &str) {
    // let file = File::open(&Path::new(filename));
    let mut output = stdout();
    let mut input  = stdin();
    let mut ctx = Context::new();
    match parser::parse_file(filename) {
        Some(program) => eval::eval(program.as_slice(), &mut ctx, &mut output, &mut input),
        None => fail!("Failed to parse {}", filename)
    }
}

fn main() {
    let args = os::args();
    match args.len() {
        0 => unreachable!(),
        2 => parse_and_eval(*args.get(1)),
        _ => usage(),
    }
}
