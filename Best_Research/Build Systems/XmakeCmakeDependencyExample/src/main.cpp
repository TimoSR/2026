#include <iostream>

#include "cmake_math.h"

int main() {
    constexpr int left = 7;
    constexpr int right = 35;

    std::cout << "CMake dependency: " << left << " + " << right << " = "
              << cmake_math::add(left, right) << '\n';
    return 0;
}
