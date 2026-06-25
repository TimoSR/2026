/* x86-64 GAS -- System V AMD64 ABI
   First argument: rdi, second argument: rsi, return value: rax */

.intel_syntax noprefix

#if defined(__APPLE__)
.globl _asm_add
_asm_add:
#else
.globl asm_add
asm_add:
#endif
    mov rax, rdi    /* rax = left */
    add rax, rsi    /* rax += right */
    ret
