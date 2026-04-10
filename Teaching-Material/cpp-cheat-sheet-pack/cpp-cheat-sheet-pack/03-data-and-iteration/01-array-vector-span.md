# Array, vector, span

```cpp
#include <array>
#include <span>
#include <vector>

using namespace std;

int sum(span<const int> values)
{
    int total = 0;

    for (int value : values) {
        total += value; // same function works with array and vector
    }

    return total;
}
```
