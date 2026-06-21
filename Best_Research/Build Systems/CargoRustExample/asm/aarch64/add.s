/* AArch64 GAS — AAPCS64 calling convention
   First arg: x0, second arg: x1, return value: x0  */

.global _asm_add

_asm_add:
    add x0, x0, x1
    ret