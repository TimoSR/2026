# Return patterns

```cpp
#include <expected>
#include <optional>
#include <string>
#include <vector>

using namespace std;

vector<string> build_tags()
{
    return { "cpp", "systems", "tooling" }; // return values directly
}

optional<string> maybe_nickname(bool enabled)
{
    if (!enabled) {
        return nullopt;
    }

    return "wizard";
}
```
