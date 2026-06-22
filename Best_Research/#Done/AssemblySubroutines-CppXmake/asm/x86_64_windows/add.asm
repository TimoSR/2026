; x86-64 MASM -- Microsoft x64 calling convention
; First argument: rcx, second argument: rdx, return value: rax

.CODE

asm_add PROC
    mov rax, rcx    ; rax = left
    add rax, rdx    ; rax += right
    ret
asm_add ENDP

END
