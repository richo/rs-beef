#![crate_id = "beef"]

#![license = "MIT"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![feature(macro_rules)]

pub mod parser;
pub mod eval;
pub mod context;
