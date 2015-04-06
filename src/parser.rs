use std::io::Read;
use std::fs::File;
use std::old_io::{BufferedReader};

pub type Program = Vec<OpCode>;

#[derive(Debug)]
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
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => panic!(err),
    };

    macro_rules! push {
        ($op:expr) => (
            match loop_stack.pop() { // Oh god why
                Some(mut v) => {
                    v.push($op);
                    loop_stack.push(v);
                },
                None    => program.push($op)
            }
            );
        }

    for c in file.chars() {
        match c.unwrap() {
            '<' => push!(OpCode::Lshift),
            '>' => push!(OpCode::Rshift),
            '.' => push!(OpCode::Putc),
            ',' => push!(OpCode::Getc),
            '+' => push!(OpCode::Inc),
            '-' => push!(OpCode::Dec),
            // Deal with loops at "compile" time
            '[' => {
                loop_stack.push(vec!());
            },
            ']' => {
                match loop_stack.pop() {
                    Some(code) => push!(OpCode::Loop(code)),
                    None => panic!("Unbalanced braces"),
                }
            }
            _   => {}
        }
    }
    Some(program)
}
