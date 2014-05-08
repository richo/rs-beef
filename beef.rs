#![feature(macro_rules)]

use std::os;
use std::io::{Reader,BufferedReader};
use std::io::File;
use std::io::stdio::{stdout,stdin,stdout_raw};

static TAPE_WIDTH: uint = 30000;

type Program = Vec<OpCode>;

#[deriving(Show)]
enum OpCode {
    Lshift,
    Rshift,
    Putc,
    Getc,
    Inc,
    Dec,
    Loop(Vec<OpCode>),
}

fn usage() {
    let args = os::args();
    println!("Usage: {} <filename>", args[0]);
}

struct Context {
    idx: uint,
    tape: [u8, ..TAPE_WIDTH],
}

fn eval(program: &Vec<OpCode>, ctx: &mut Context ) {
    // Does the spec have strong feelings about which way/how far the tape
    // goes?
    let mut pc = 0;
    let mut _out = stdout();
    // let _in  = Reader::new(stdin());
    let mut _in  = stdin();

    while pc < program.len() {
        match *program.get(pc) {
            Lshift  => ctx.idx -= 1,
            Rshift  => ctx.idx += 1,
            Inc     => ctx.tape[ctx.idx] += 1,
            Dec     => ctx.tape[ctx.idx] -= 1,
            Putc    => { _out.write_u8(ctx.tape[ctx.idx]); () },
            Getc    => { ctx.tape[ctx.idx] = _in.read_u8().unwrap(); () },
            Loop(ref l) => {
                while ctx.tape[ctx.idx] != 0 {
                    eval(l, ctx);
                };
            }
        }
        pc += 1;
    }
}

fn parse_and_eval(filename: &str) {
    let mut program: Program = vec!();
    let mut loop_stack: Vec<Vec<OpCode>> = vec!();
    // let file = File::open(&Path::new(filename));
    let mut file = BufferedReader::new(File::open(&Path::new(filename)));

    macro_rules! push(
        ($op:expr) => (
            match loop_stack.pop() { // Oh god why
                Some(mut v) => {
                    v.push($op);
                    loop_stack.push(v);
                },
                None    => program.push($op)
            }
            );
        )

    for c in file.chars() {
        match c.unwrap() {
            '<' => push!(Lshift),
            '>' => push!(Rshift),
            '.' => push!(Putc),
            ',' => push!(Getc),
            '+' => push!(Inc),
            '-' => push!(Dec),
            // Deal with loops at "compile" time
            '[' => {
                loop_stack.push(vec!());
            },
            ']' => {
                match loop_stack.pop() {
                    Some(code) => push!(Loop(code)),
                    None => fail!("Unbalanced braces"),
                }
            }
            _   => {}
        }
    }

    let mut context: Context = Context {
        idx : TAPE_WIDTH / 2,
        tape: [0, ..TAPE_WIDTH],
    };
    eval(&program, &mut context);
}

fn main() {
    let args = os::args();
    match args.len() {
        0 => unreachable!(),
        2 => parse_and_eval(args[1]),
        _ => usage(),
    }
}
