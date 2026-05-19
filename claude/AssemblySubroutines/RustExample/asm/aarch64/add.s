/* AArch64 GAS — AAPCS64 calling convention
   First arg: x0, second arg: x1, return value: x0  */

.global asm_add

asm_add:
    add x0, x0, x1  /* x0 = a + b */
    ret
