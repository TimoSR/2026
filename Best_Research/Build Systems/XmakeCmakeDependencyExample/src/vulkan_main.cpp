#include <iostream>

#include "vulkan_demo/vulkan_demo.hpp"

int main() {
    const auto loader = vulkan_demo::inspect_loader();
    std::cout << loader.message << '\n';
    return loader.available ? 0 : 1;
}
