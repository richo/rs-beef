use parser;
use parser::{OpCode};
use context::Context;

pub fn eval<W: Writer, R: Reader>(program: &[OpCode], ctx: &mut Context, output: &mut W, input: &mut R) {
    // Does the spec have strong feelings about which way/how far the tape
    // goes?
    let mut pc = 0;
    // let _in  = Reader::new(stdin());
    let len = program.len();

    while pc < len {
        match program[pc] {
            parser::Lshift  => ctx.idx -= 1,
            parser::Rshift  => ctx.idx += 1,
            parser::Inc     => ctx.tape[ctx.idx] += 1,
            parser::Dec     => ctx.tape[ctx.idx] -= 1,
            parser::Putc    => { output.write_u8(ctx.tape[ctx.idx]); () },
            parser::Getc    => { ctx.tape[ctx.idx] = input.read_u8().unwrap(); () },
            parser::Loop(ref l) => {
                while ctx.tape[ctx.idx] != 0 {
                    eval(l.as_slice(), ctx, output, input);
                };
            }
        }
        pc += 1;
    }
}

