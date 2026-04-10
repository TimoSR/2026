# format, filesystem, chrono

```cpp
#include <chrono>
#include <filesystem>
#include <format>
#include <string>

using namespace std;
using namespace chrono;

string build_message(const string& user_name, int score)
{
    return format("user={} score={}", user_name, score);
}

milliseconds default_timeout()
{
    return 2500ms;
}
```
