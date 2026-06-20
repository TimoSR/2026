#include <cassert>

#include "battery_and_active_displays.cpp"

int main() {
    int battery = batteryLifePercentWin32();
    int displays = activeDisplayCountWin32();

    assert(battery >= -1 && battery <= 100);
    assert(displays >= 0);
    return 0;
}
