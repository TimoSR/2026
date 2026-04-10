# Parameter patterns

```cpp
#include <span>
#include <string>
#include <string_view>
#include <vector>

using namespace std;

void set_age(int age) // cheap value -> pass by value
{
}

void rename_user(string name) // ownership -> pass by value
{
}

void print_label(string_view label) // read-only text -> view
{
}

void process_scores(span<const int> scores) // contiguous read-only values
{
}

void sort_names(vector<string>& names) // mutate caller-owned object
{
}
```
