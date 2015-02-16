const TAPE_WIDTH: usize = 30000;

pub struct Context {
    pub idx: usize,
    pub tape: [u8; TAPE_WIDTH],
}

impl Context {
    pub fn new() -> Context {
        Context {
            idx : 0,
            // FIXME
            tape: [0; TAPE_WIDTH],
        }
    }
}

