#![crate_name = "beef"]

#![license = "MIT"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![feature(macro_rules)]
#![feature(asm)]

extern crate libc;

pub mod parser;
pub mod eval;
pub mod context;
pub mod compiler;
pub mod jit;
