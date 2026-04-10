# Maps and lookups

```cpp
#include <optional>
#include <string>
#include <unordered_map>

using namespace std;

optional<int> try_get_score(const unordered_map<string, int>& scores, const string& name)
{
    unordered_map<string, int>::const_iterator found = scores.find(name);

    if (found == scores.end()) {
        return nullopt;
    }

    return found->second;
}
```
