# Algorithms

```cpp
#include <algorithm>
#include <numeric>
#include <vector>

using namespace std;

int main()
{
    vector<int> values { 5, 1, 9, 2, 7 };

    sort(values.begin(), values.end());
    int total = accumulate(values.begin(), values.end(), 0);

    bool has_large_value = any_of(values.begin(), values.end(), [](int value) {
        return value > 8;
    });
}
```
