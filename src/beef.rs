extern crate beef;

use std::os;
use std::io::stdio::{stdout,stdin};

use beef::parser;
use beef::parser::{OpCode};

static TAPE_WIDTH: uint = 30000;

fn usage() {
    let args = os::args();
    println!("Usage: {} <filename>", args.get(0));
}

struct Context {
    idx: uint,
    tape: [u8, ..TAPE_WIDTH],
}

fn eval<W: Writer>(program: &[OpCode], ctx: &mut Context, output: &mut W) {
    // Does the spec have strong feelings about which way/how far the tape
    // goes?
    let mut pc = 0;
    // let _in  = Reader::new(stdin());
    let mut _in  = stdin();
    let len = program.len();

    while pc < len {
        match program[pc] {
            parser::Lshift  => ctx.idx -= 1,
            parser::Rshift  => ctx.idx += 1,
            parser::Inc     => ctx.tape[ctx.idx] += 1,
            parser::Dec     => ctx.tape[ctx.idx] -= 1,
            parser::Putc    => { output.write_u8(ctx.tape[ctx.idx]); () },
            parser::Getc    => { ctx.tape[ctx.idx] = _in.read_u8().unwrap(); () },
            parser::Loop(ref l) => {
                while ctx.tape[ctx.idx] != 0 {
                    eval(l.as_slice(), ctx, output);
                };
            }
        }
        pc += 1;
    }
}

fn parse_and_eval(filename: &str) {
    // let file = File::open(&Path::new(filename));
    let mut output = stdout();
    let mut context: Context = Context {
        idx : 0,
        tape: [0, ..TAPE_WIDTH],
    };
    match parser::parse_file(filename) {
        Some(program) => eval(program.as_slice(), &mut context, &mut output),
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
