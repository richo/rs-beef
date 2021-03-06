use parser;
use parser::{OpCode};
use context::Context;

use std::io::{Write,Read};

pub fn eval<W: Write, R: Read>(program: &[OpCode], ctx: &mut Context, output: &mut W, input: &mut R) {
    // Does the spec have strong feelings about which way/how far the tape
    // goes?
    let mut pc = 0;
    // let _in  = Reader::new(stdin());
    let len = program.len();

    while pc < len {
        match program[pc] {
            OpCode::Lshift  => ctx.idx -= 1,
            OpCode::Rshift  => ctx.idx += 1,
            OpCode::Inc     => ctx.tape[ctx.idx] += 1,
            OpCode::Dec     => ctx.tape[ctx.idx] -= 1,
            OpCode::Putc    => { output.write(&[ctx.tape[ctx.idx]]); () },
            OpCode::Getc    => { ctx.tape[ctx.idx] = input.bytes().next().unwrap().unwrap(); () },
            OpCode::Loop(ref l) => {
                while ctx.tape[ctx.idx] != 0 {
                    eval(l.as_slice(), ctx, output, input);
                };
            }
        }
        pc += 1;
    }
}

