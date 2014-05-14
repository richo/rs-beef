use libc;
use libc::mmap;
use std::os;
use std::mem;
use parser;
use parser::{OpCode,Loop,Program};
use compiler;
use std::io::timer::sleep;
use libc::funcs::posix88::unistd::getpid;

struct Context {
    putc: *u8,
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
}

static FRAME_SIZE: u8 = 5;
fn assemble_into(program: Program, assembler: &mut Assembler) {
    // let asm: &[Option<u8>];
    for isn in program.iter() {
        let asm: [Option<u8>, ..FRAME_SIZE] = match *isn {
            // LOLOLOL
            // add 1, rsi
            parser::Rshift => [ Some(0x48), Some(0x83), Some(0xC6), Some(0x01), None      ],
            // sub 1, rsi
            parser::Lshift => [ Some(0x48), Some(0x83), Some(0xEE), Some(0x01), None      ],
            // sub 1, [rsi]
            parser::Dec    => [ Some(0x80), Some(0x2E), Some(0x01), None      , None      ],
            // add 1, [rsi]
            parser::Inc    => [ Some(0x80), Some(0x06), Some(0x01), None      , None      ],
            // call r12
            parser::Putc   => [ Some(0x41), Some(0xFF), Some(0xD4), None      , None      ],
            // TODO call r13
            parser::Getc   => fail!("Getc not imlemented"),
            // TODO: Maintain a jump table, use successive instructions to inc
            // and dec it so that we know which jump target to use. These can just
            // be pointer offsets into the text page.
            parser::Loop(ref l)=> fail!("Loop not implemented"),
        };

        for i in asm.iter() {
            assembler.byte(match *i {
                Some(h) => { h },
                None => { 0x90 }
            })
        }
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

    // Entry point for the real executable
    ctx.text = asm.ptr;

    println!("Putc function located at: {}", ctx.putc);
    println!("Text segment located at: {}", ctx.text);

    assemble_into(program, &mut asm);

    // let pid = unsafe { getpid() as uint };
    // println!("Sleeping forever to allow debugger attach, relevantly, pid: {}", pid);
    // sleep(10000);
    // println!("So I gess we're jumpin' jumpin'");
    // unsafe { ::core::intrinsics::breakpoint(); }

    unsafe {
        asm!("movq  $0, %r12" :: "r"(ctx.putc));
        asm!("movq  $0, %rax
             callq  *%rax" :: "r"(ctx.text));
    }

    0 as *libc::c_void
}
