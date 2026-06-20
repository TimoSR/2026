; x86-64 MASM — Microsoft x64 calling convention
; First arg: rcx, second arg: rdx, return value: rax

.CODE

asm_add PROC
    mov rax, rcx    ; rax = a
    add rax, rdx    ; rax += b
    ret
asm_add ENDP

END
