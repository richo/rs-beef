pub static TAPE_WIDTH: uint = 30000;

pub struct Context {
    pub idx: uint,
    pub tape: [u8, ..TAPE_WIDTH],
}

impl Context {
    pub fn new() -> Context {
        Context {
            idx : 0,
            tape: [0, ..TAPE_WIDTH],
        }
    }
}

