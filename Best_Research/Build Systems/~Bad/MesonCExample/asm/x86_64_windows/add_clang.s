/* x86-64 Windows ABI for clang integrated assembler
   First arg: rcx, second arg: rdx, return value: rax */

.intel_syntax noprefix
.globl asm_add

asm_add:
    mov rax, rcx
    add rax, rdx
    ret
