use std::io::File;
use std::io::{BufferedReader};

pub type Program = Vec<OpCode>;

#[deriving(Show)]
pub enum OpCode {
    Lshift,
    Rshift,
    Putc,
    Getc,
    Inc,
    Dec,
    Loop(Vec<OpCode>),
}

pub fn parse_file(filename: &str) -> Option<Program> {
    let mut program: Program = vec!();
    let mut loop_stack: Vec<Vec<OpCode>> = vec!();
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
                    None => panic!("Unbalanced braces"),
                }
            }
            _   => {}
        }
    }
    Some(program)
}
