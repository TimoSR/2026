#include <cstdint>
#include <iostream>

using std::int64_t;

extern "C" int64_t asm_add(int64_t a, int64_t b);

int main() {
    int64_t a = 7;
    int64_t b = 35;
    int64_t result = asm_add(a, b);

    std::cout << a << " + " << b << " = " << result << '\n';
    return 0;
}
