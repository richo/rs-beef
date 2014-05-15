use libc;
use libc::mmap;
use std::os;
use std::mem;
use parser;
use parser::{OpCode,Loop,Program};
use compiler;
use std::io::timer::sleep;
use libc::funcs::posix88::unistd::getpid;
use core::ptr;

struct Context {
    putc: *u8,
    getc: *u8,
    text: *u8,
}

struct Assembler {
    ptr: *u8
}

impl Assembler {
    fn byte(&mut self, byte: u8) {
        unsafe {
            let mut ptr: *mut u8 = self.ptr as *mut u8;
            *ptr = byte;
            self.ptr = self.ptr.offset(1);
        }
    }

    fn byte_array(&mut self, bytes: &[u8]) {
        for i in bytes.iter() {
            self.byte(*i);
        }
    }

    fn instructions(&mut self, asm: Instructions) {
        for i in asm.iter() {
            self.byte(match *i {
                Some(h) => { h },
                None => { 0x90 }
            })
        }
    }
}

enum Reg {
    Rsi,
    R12,
}

type Instructions = [Option<u8>, ..x64::FRAME_SIZE];
mod x64 {
    use parser;

    pub static FRAME_SIZE: u8 = 5;
    // TODO Actuall check which register this is
    pub fn addi(reg: super::Reg, v: u8) -> super::Instructions {
        [ Some(0x48), Some(0x83), Some(0xC6), Some(  v ), None      ]
    }

    pub fn deci(reg: super::Reg, v: u8) -> super::Instructions {
        [ Some(0x48), Some(0x83), Some(0xEE), Some(  v ), None      ]
    }

    // Indirect

    pub fn i_addi(reg: super::Reg, v: u8) -> super::Instructions {
        [ Some(0x80), Some(0x06), Some(0x01), None      , None      ]
    }

    pub fn i_deci(reg: super::Reg, v: u8) -> super::Instructions {
        [ Some(0x80), Some(0x2E), Some(0x01), None      , None      ]
    }

    pub fn i_call(reg: super::Reg) -> super::Instructions{
        // Assumes r12
        [ Some(0x41), Some(0xFF), Some(0xD4), None      , None      ]
    }

    pub fn i_cmp(reg: super::Reg, v: u8) -> super::Instructions{
        [ Some(0x66), Some(0x83), Some(0x3E), Some(  v ), None      ]
    }

    pub fn jmp(v: uint) -> super::Instructions {
        [ None, None, None, None, None ]
    }

    pub fn jne(v: uint) -> super::Instructions {
        [ None, None, None, None, None ]
    }

    pub fn effective_len(program: &parser::Program) -> uint {
        let mut len = 0;
        let frame_size = FRAME_SIZE as uint;
        for op in program.iter() {
            match *op {
                // Absurd guess. Who even knows.
                parser::Loop(ref l) => len += effective_len(l) + frame_size * 2,
                _       => len += frame_size,
            }
        }
        len
    }
}

fn assemble_into(program: &Program, assembler: &mut Assembler) {
    // let asm: &[Option<u8>];
    for isn in program.iter() {
        let asm: Instructions = match *isn {
            // LOLOLOL
            // add 1, rsi
            parser::Rshift => x64::addi(Rsi, 1),
            // sub 1, rsi
            parser::Lshift => x64::deci(Rsi, 1),
            // add 1, [rsi]
            parser::Inc    => x64::i_addi(Rsi, 1),
            // sub 1, [rsi]
            parser::Dec    => x64::i_deci(Rsi, 1),
            // call r12
            parser::Putc   => x64::i_call(R12),
            // TODO call r13
            parser::Getc   => fail!("Getc not imlemented"),
            // TODO: Maintain a jump table, use successive instructions to inc
            // and dec it so that we know which jump target to use. These can just
            // be pointer offsets into the text page.
            parser::Loop(ref l)=> {
                let ret = assembler.ptr;
                // Kludge, assemble all we wan
                assembler.instructions(x64::i_cmp(Rsi, 0));
                [ None, None, None, None, None]

            }
        };

        assembler.instructions(asm);
    }

}

pub fn load(program: Program, tape_size: uint) -> *libc::c_void {
    let tape = unsafe {
        libc::mmap(0 as *libc::c_void, tape_size as u64,
        libc::PROT_READ | libc::PROT_WRITE,
        libc::MAP_ANON | libc::MAP_PRIVATE, 0, 0) as *libc::c_void
    };
    if tape == libc::MAP_FAILED {
        fail!("Couldn't mmap tape: {}", os::last_os_error());
    }
    unsafe {
        ptr::zero_memory(tape as *mut u8, tape_size);
    }

    let text_size = compiler::effective_len(&program) * 4; // 32 bit wide instruction, probably
    let start_text = unsafe {
        libc::mmap(0 as *libc::c_void, text_size as u64,
        // TODO Remove the x bit before we jmp in
        libc::PROT_WRITE | libc::PROT_READ | libc::PROT_EXEC,
        libc::MAP_ANON | libc::MAP_PRIVATE, 0, 0) as *libc::c_void
    };
    if start_text == libc::MAP_FAILED {
        fail!("Couldn't mmap text: {}", os::last_os_error());
    }

    let mut ctx: Context = unsafe { mem::init() };
    let mut asm = Assembler { ptr: start_text as *u8 };
    let mut text = start_text as *mut u8;

    // Somewhat less lurky solution:
    // mov    eax, 0x2000004
    // mov    edi, 0x1
    // ; rsi is already a pointer to buf
    // mov    edx, 0x1
    // syscall
    // ret
    ctx.putc = asm.ptr;
    asm.byte_array([0xB8, 0x04, 0x00, 0x00, 0x02, 0xBF, 0x01, 0x00, 0x00, 0x00, 0xBA, 0x01, 0x00, 0x00, 0x00, 0x0F, 0x05, 0xC3]);

    // mov    eax, 0x2000003
    // mov    edi, 0x1
    // ; rsi is already a pointer to buf
    // mov    edx, 0x1
    // syscall
    // ret
    ctx.getc = asm.ptr;
    asm.byte_array([0xB8, 0x03, 0x00, 0x00, 0x02, 0xBF, 0x01, 0x00, 0x00, 0x00, 0xBA, 0x01, 0x00, 0x00, 0x00, 0x0F, 0x05, 0xC3]);

    // Entry point for the real executable
    ctx.text = asm.ptr;

    println!("Putc function located at: {}", ctx.putc);
    println!("Text segment located at: {}", ctx.text);

    assemble_into(&program, &mut asm);

    // let pid = unsafe { getpid() as uint };
    // println!("Sleeping forever to allow debugger attach, relevantly, pid: {}", pid);
    // sleep(10000);
    // println!("So I gess we're jumpin' jumpin'");
    // unsafe { ::core::intrinsics::breakpoint(); }

    unsafe {
        asm!("movq  $0, %r12" :: "r"(ctx.putc));
        asm!("movq  $0, %r13" :: "r"(ctx.getc));
        asm!("movq  $0, %rsi" :: "r"(tape));
        asm!("movq  $0, %rax
             callq  *%rax" :: "r"(ctx.text));
    }

    0 as *libc::c_void
}
