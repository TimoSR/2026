/* x86-64 GAS — System V AMD64 ABI (Linux / macOS)
   First arg: rdi, second arg: rsi, return value: rax  */

.intel_syntax noprefix
.global asm_add

asm_add:
    mov rax, rdi
    add rax, rsi
    ret
