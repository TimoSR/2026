#include <cassert>

#include "open_window_and_message_loop.cpp"

int main() {
#ifdef _WIN32
    bool didRun = runWindowForMilliseconds(150);
    assert(didRun == true);
#endif
    return 0;
}
