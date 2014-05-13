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

fn assemble_into(program: Program, assembler: &mut Assembler) {
    // let asm: &[Option<u8>];
    for isn in program.iter() {
        let asm = match *isn {
            // LOLOLOL
            parser::Rshift => [ Some(0x48), Some(0x83), Some(0xC6), Some(0x01), None      ],
            parser::Lshift => [ Some(0x48), Some(0x83), Some(0xEE), Some(0x01), None      ],
            parser::Dec    => [ Some(0x80), Some(0x2E), Some(0x01), None      , None      ],
            parser::Inc    => [ Some(0x80), Some(0x06), Some(0x01), None      , None      ],
            // Lololol, seriously just use a register to hang onto putc
            parser::Putc   => [ Some(0xFF), Some(0xD3), None      , None      , None      ],
            parser::Getc   => fail!("Getc not imlemented"),
            parser::Loop(ref l)=> fail!("Loop not implemented"),
        };

        for i in asm.iter() {
            match *i {
                Some(h) => {
                    assembler.byte(h);
                },
                None => {}
            }
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

    // Setup esi as a pointer to the tape
    // TODO Setup a return address (lol?)
    // let tape_prelude = vec!(0xbe, 0x00, 0x20, 0x00, 0x10, 0xf8);
    // for isn in tape_prelude.iter() {
    //     // *tape = *isn;
    //     // tape += 1;
    // }

    let mut ctx: Context = unsafe { mem::init() };
    let mut asm = Assembler { ptr: start_text as *u8 };
    let mut text = start_text as *mut u8;

    println!("Old text segment at {}", text);

    ctx.putc = asm.ptr;
    // TODO replace all these ctx vars with offsets from a known base

    // Setup the syscall handler for putc right at the start:
    asm.byte_array([ 0x6A, 0x01, 0x51, 0x6A, 0x01, 0x48, 0xC7, 0xC0,
                     0x04, 0x00, 0x00, 0x00, 0x48, 0x83, 0xEC, 0x04,
                     0xCD, 0x80, 0x48, 0x83, 0xC4, 0x10, 0xC3 ]);


    // Entry point for the real executable
    ctx.text = asm.ptr;

    // Load putc into rbx TODO Fixup address
    asm.byte_array([ 0x48, 0xBB,
                   // Load address
                   0x00, 0xA0, 0x4F, 0x05, 0x01,
                   0x00, 0x00, 0x00 ]);

    println!("Putc function located at: {}", ctx.putc);
    println!("Text segment located at: {}", ctx.text);

    assemble_into(program, &mut asm);

    let pid = unsafe { getpid() as uint };
    println!("Sleeping forever to allow debugger attach, relevantly, pid: {}", pid);

    sleep(10000);

    println!("So I gess we're jumpin' jumpin'");

    // unsafe {
    //     asm!("callq $0" :: "r"(ctx.text));
    // }


    0 as *libc::c_void
}
