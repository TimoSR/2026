# Lambdas and callables

```cpp
#include <algorithm>
#include <functional>
#include <string>
#include <vector>

using namespace std;

int main()
{
    vector<string> names { "anna", "bob", "chris" };

    int count = static_cast<int>(count_if(
        names.begin(),
        names.end(),
        [](const string& name) {
            return name.size() >= 4; // local predicate
        }
    ));
}
```
