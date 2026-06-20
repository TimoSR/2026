#include <cassert>

#include "cpu_memory_and_uptime.cpp"

int main() {
    assert(logicalCpuCountWin32() > 0);
    assert(totalPhysicalMemoryMbWin32() > 0);
    assert(uptimeMillisecondsWin32() > 0);
    return 0;
}
