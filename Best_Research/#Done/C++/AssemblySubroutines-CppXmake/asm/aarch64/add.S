/* AArch64 GAS -- AAPCS64 calling convention
   First argument: x0, second argument: x1, return value: x0 */

#if defined(__APPLE__)
.globl _asm_add
_asm_add:
#else
.globl asm_add
asm_add:
#endif
    add x0, x0, x1
    ret
