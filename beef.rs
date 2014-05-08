use std::os;
use std::io::{Reader,BufferedReader};
use std::io::File;
use std::io::stdio::{stdout,stdin,stdout_raw};

static TAPE_WIDTH: uint = 30000;

type Program = Vec<Op>;

#[deriving(Show)]
enum Op {
    Lshift,
    Rshift,
    Putc,
    Getc,
    Inc,
    Dec,
    Lbrace,
    Rbrace
}

fn usage() {
    let args = os::args();
    println!("Usage: {} <filename>", args[0]);
}

// fn _loop(content: &[Op], tape: &mut [u8]) {
//     let idx = 0;

//     if tape[idx] == 0 {
//         return;
//     }

//     while tape[idx] != Rbrace {
//     }
// }

struct Context {
    idx: uint,
    tape: [u8, ..TAPE_WIDTH],
}

fn matching_brace(program: &Vec<Op>, start: uint) -> uint {
    let mut depth = 0;
    let mut pos = 0;
    loop {
        println!("analysing: {}", *program.get(start + pos));
        match *program.get(start + pos) {
            Lbrace => depth += 1,
            Rbrace => {
                if depth == 0 { break }
                depth -= 1;
            }
            _ => {},
        }
        pos += 1;
    }
    return pos;

}

fn eval(program: &Program, frame_loc: uint, mut ctx: Context ) -> uint {
    // Does the spec have strong feelings about which way/how far the tape
    // goes?
    let mut pc = frame_loc;
    let mut _out = stdout();
    // let _in  = Reader::new(stdin());
    let mut _in  = stdin();

    while pc < program.len() {
        let op = *program.get(pc);
        println!("Evaluating: {}, pc: {} curval: {}", op, pc, ctx.tape[ctx.idx]);
        match op {
            Lshift  => ctx.idx -= 1,
            Rshift  => ctx.idx += 1,
            Inc     => ctx.tape[ctx.idx] += 1,
            Dec     => ctx.tape[ctx.idx] -= 1,
            Putc    => { _out.write_u8(ctx.tape[ctx.idx]); () },
            Getc    => { ctx.tape[ctx.idx] = _in.read_u8().unwrap(); () },
            Rbrace => {
                if ctx.tape[ctx.idx] == 0 {
                    return pc;
                } else {
                    println!("Jumping back to {} isn: {}", frame_loc - 1, program.get(frame_loc - 2));
                    pc = (frame_loc - 1) // -2 is the increment and skip back one

                }
            }
            Lbrace => {
                // Specialcase skips
                if ctx.tape[ctx.idx] == 0 {
                    pc += matching_brace(program, pc);
                } else {
                    pc = eval(program, pc + 1, ctx); // +1 ensures we execute after the brace
                    ()
                }
            }
        }
        pc += 1;
    }
    return 0;
}

fn parse_and_eval(filename: &str) {
    let mut program: Program = vec!();
    // let file = File::open(&Path::new(filename));
    let mut file = BufferedReader::new(File::open(&Path::new(filename)));

    for c in file.chars() {
        match c.unwrap() {
            '<' => program.push(Lshift),
            '>' => program.push(Rshift),
            '.' => program.push(Putc),
            ',' => program.push(Getc),
            '+' => program.push(Inc),
            '-' => program.push(Dec),
            '[' => program.push(Lbrace),
            ']' => program.push(Rbrace),
            _   => {}
        }
    }

    let mut context: Context = Context {
        idx : TAPE_WIDTH / 2,
        tape: [0, ..TAPE_WIDTH],
    };
    eval(&program, 0, context);
}

fn main() {
    let args = os::args();
    match args.len() {
        0 => unreachable!(),
        2 => parse_and_eval(args[1]),
        _ => usage(),
    }
}
