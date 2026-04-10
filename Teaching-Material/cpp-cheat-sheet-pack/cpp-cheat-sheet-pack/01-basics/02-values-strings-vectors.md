# Values, strings, vectors

```cpp
#include <string>
#include <vector>

using namespace std;

int main()
{
    int user_count = 3;
    double price = 19.99;
    bool enabled = true;
    string title = "modern c++";
    vector<int> scores { 10, 20, 30 };
}
```

```cpp
#include <string>
#include <string_view>

using namespace std;

string build_message(string name)
{
    return "hello " + name; // string owns memory
}

void print_label(string_view label)
{
    // string_view reads without owning
}
```

```cpp
#include <string>
#include <vector>

using namespace std;

int main()
{
    vector<string> tags;
    tags.push_back("cpp");
    tags.emplace_back("tooling"); // construct directly in vector
}
```
