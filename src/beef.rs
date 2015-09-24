#![feature(convert)]
extern crate beef;

use std::os;
use std::env;
use std::io::{stdout,stdin};

use beef::context::Context;
use beef::parser;
use beef::eval;

fn usage() {
    let args: Vec<_> = env::args().collect();
    println!("Usage: {} <filename>", args[0]);
}

fn parse_and_eval(filename: &str) {
    // let file = File::open(&Path::new(filename));
    let mut output = stdout();
    let mut input  = stdin();
    let mut ctx = Context::new();
    match parser::parse_file(filename) {
        Some(program) => eval::eval(program.as_slice(), &mut ctx, &mut output, &mut input),
        None => panic!("Failed to parse {}", filename)
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    match args.len() {
        0 => unreachable!(),
        2 => parse_and_eval(&args[1]),
        _ => usage(),
    }
}
