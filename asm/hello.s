global start

section .bss
tape:    resb     30000

section .text

; dot:
;     push    dword msg.len
;     push    dword msg
;     push    dword 1
;     mov     eax, 4
;     sub     esp, 4
;     int     0x80
;     add     esp, 16
;     ret

dot:
    push    dword 1
    push    ecx
    push    dword 1
    mov     eax, 4
    sub     esp, 4
    int     0x80
    add     esp, 16
    ret

lshift:
    add     ecx, byte 1

rshift:
    sub     ecx, byte 1

inc:
    add     [ecx], byte 1

sub:
    sub     [ecx], byte 1

start:
    add     [tape], byte 104
    mov     ecx, tape
    call    dot

    push    dword 0
    mov     eax, 1
    push    dword 0
    int     0x80
