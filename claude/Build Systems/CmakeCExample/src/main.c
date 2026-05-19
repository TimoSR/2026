#include <stdio.h>
#include <stdint.h>

extern int64_t asm_add(int64_t a, int64_t b);

int main(void) {
    int64_t a = 7;
    int64_t b = 35;
    int64_t result = asm_add(a, b);

    printf("%lld + %lld = %lld\n",
           (long long)a,
           (long long)b,
           (long long)result);
    return 0;
}
