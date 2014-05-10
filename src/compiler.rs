use parser;
use parser::{OpCode,Loop,Program};

struct Context {
    isn: uint,
}

impl Context {
    fn new() -> Context {
        Context {
            isn: 0
        }
    }
}

fn effective_len(program: &Program) -> uint {
    let mut len = 0;
    for op in program.iter() {
        match *op {
            Loop(ref l) => len += effective_len(l) + 1,
            _       => len += 1,
        }
    }
    len
}

pub fn compile<W: Writer>(program: &[OpCode], outfile: &mut W) {
    let mut ctx = Context::new();

    outfile.write(PRELUDE.as_bytes());
    let final = inner(program, outfile, &mut ctx);
    outfile.write(format!("    isn{}:\n", ctx.isn).as_bytes());
    outfile.write(EPILOGUE.as_bytes());
}

#[allow(unused_must_use)]
fn inner<W: Writer>(program: &[OpCode], outfile: &mut W, ctx: &mut Context) {
    macro_rules! write(
        ($op:expr) => (
            outfile.write($op.as_bytes());
            )
        )

    macro_rules! write_s(
        ($op:expr) => (
            {
            write!($op.to_owned());
            ()
            }

            )
        )


    let len = program.len();
    let mut pc = 0;
    while pc < len {
        write!(format!("    isn{}:\n", ctx.isn));
        ctx.isn += 1;
        match program[pc] {
            parser::Rshift  => write_s!("    add     esi, dword 1\n"),
            parser::Lshift  => write_s!("    sub     esi, dword 1\n"),
            parser::Inc     => write_s!("    add     [esi], dword 1\n"),
            parser::Dec     => write_s!("    sub     [esi], dword 1\n"),
            parser::Putc    => write_s!("    call    dot\n"),
            parser::Getc    => fail!("Getc not implemented"),
            parser::Loop(ref l) => {
                let jmp = format!("    jmp     isn{}\n", ctx.isn - 1);
                write_s!("    cmp      [esi], byte 0\n");
                write!(format!("    je      isn{}\n", ctx.isn + effective_len(l)));
                inner(l.as_slice(), outfile, ctx);
                write!(jmp);
            }
        }
        pc += 1;
    }
}

// {{{ boilerplate
static PRELUDE: &'static str = "
global start

section .bss
tape:    resb     30000

section .text

dot:
    push    dword 1
    push    esi
    push    dword 1
    mov     eax, 4
    sub     esp, 4
    int     0x80
    add     esp, 16
    ret

start:
    ; Begin, setup rcx as our index
    mov     esi, tape
";

static EPILOGUE: &'static str = "
    ; End, exit zero because everything probably went super well
    push    dword 0
    mov     eax, 1
    push    dword 0
    int     0x80
";
// }}}
