use libc;
use libc::mmap;
use std::os;
use std::io::Error;
use parser;
use parser::{OpCode,Program};
use compiler;
use std::old_io::timer::sleep;
use std::time::duration::Duration;
use libc::funcs::posix88::unistd::getpid;

struct Context {
    putc: *const u8,
    text: *const u8,
}

fn assemble_into(program: Program, mut text_segment: *mut u8) {
    // let asm: &[Option<u8>];
    for isn in program.iter() {
        let asm = match *isn {
            // LOLOLOL
            OpCode::Rshift => [ Some(0x48), Some(0x83), Some(0xC6), Some(0x01), None      ],
            OpCode::Lshift => [ Some(0x48), Some(0x83), Some(0xEE), Some(0x01), None      ],
            OpCode::Dec    => [ Some(0x80), Some(0x2E), Some(0x01), None      , None      ],
            OpCode::Inc    => [ Some(0x80), Some(0x06), Some(0x01), None      , None      ],
            // Lololol, seriously just use a register to hang onto putc
            OpCode::Putc   => [ Some(0xFF), Some(0xD3), None      , None      , None      ],
            OpCode::Getc   => panic!("Getc not imlemented"),
            OpCode::Loop(ref l)=> panic!("Loop not implemented"),
        };

        for i in asm.iter() {
            match *i {
                Some(h) => {
                    unsafe {
                        *text_segment = h;
                        text_segment = text_segment.offset(1);
                    }
                },
                None => {}
            }
        }
    }

}

pub fn load(program: Program, tape_size: usize) -> *const libc::c_void {
    let tape = unsafe {
        libc::mmap(0 as *mut libc::c_void, tape_size as u64,
        libc::PROT_READ | libc::PROT_WRITE,
        libc::MAP_ANON | libc::MAP_PRIVATE, 0, 0) as *mut libc::c_void
    };
    if tape == libc::MAP_FAILED {
        panic!("Couldn't mmap tape: {}", Error::last_os_error());
    }

    let text_size = compiler::effective_len(&program) * 4; // 32 bit wide instruction, probably
    let start_text = unsafe {
        libc::mmap(0 as *mut libc::c_void, text_size as u64,
        // TODO Remove the x bit before we jmp in
        libc::PROT_WRITE | libc::PROT_READ | libc::PROT_EXEC,
        libc::MAP_ANON | libc::MAP_PRIVATE, 0, 0) as *mut libc::c_void
    };
    if start_text == libc::MAP_FAILED {
        panic!("Couldn't mmap text: {}", Error::last_os_error());
    }

    // Setup esi as a pointer to the tape
    // TODO Setup a return address (lol?)
    // let tape_prelude = vec!(0xbe, 0x00, 0x20, 0x00, 0x10, 0xf8);
    // for isn in tape_prelude.iter() {
    //     // *tape = *isn;
    //     // tape += 1;
    // }

    // TODO Don't build this up.
    let mut text = start_text as *mut u8;

    println!("Old text segment at {:?}", text);

    let ctx_putc = text as *const u8;
    // Setup the syscall handler for putc right at the start:
    let putc_text = [ 0x6A, 0x01, 0x51, 0x6A, 0x01, 0x48, 0xC7, 0xC0,
               0x04, 0x00, 0x00, 0x00, 0x48, 0x83, 0xEC, 0x04,
               0xCD, 0x80, 0x48, 0x83, 0xC4, 0x10, 0xC3 ];
    for i in putc_text.iter() {
                   unsafe {
                       *text = *i;
                       text = text.offset(1);
                   }
               }


    // Entry point for the real executable
    let ctx_text = text as *const u8;

    // Load putc into rbx
    for i in [ 0x48, 0xBB, 0x00, 0xA0, 0x4F, 0x05, 0x01, 0x00, 0x00, 0x00 ].iter() {
        unsafe {
            *text = *i;
            text = text.offset(1);
        }
    }

    let ctx = Context {
        putc: ctx_putc,
        text: ctx_text,
    };

    println!("Putc function located at: {:?}", ctx.putc);
    println!("Text segment located at: {:?}", ctx.text);

    assemble_into(program, ctx.text as *mut u8);

    let pid = unsafe { getpid() as usize };
    println!("Sleeping forever to allow debugger attach, relevantly, pid: {}", pid);

    sleep(Duration::seconds(1));

    println!("So I gess we're jumpin' jumpin'");

    unsafe {
        asm!("jmp $0" :: "0"(ctx.text));
    }


    0 as *mut libc::c_void
}
