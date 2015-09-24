#![feature(io)]
#![feature(convert)]
#![feature(thread_sleep)]
#![crate_name = "beef"]

#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![feature(asm)]
#![feature(libc, old_io)]

extern crate libc;

pub mod parser;
pub mod eval;
pub mod context;
pub mod compiler;
pub mod jit;
