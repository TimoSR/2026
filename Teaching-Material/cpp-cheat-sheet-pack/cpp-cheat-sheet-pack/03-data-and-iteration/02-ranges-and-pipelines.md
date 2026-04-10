# Ranges and pipelines

```cpp
#include <ranges>
#include <string>
#include <vector>

using namespace std;

struct User
{
    string name;
    bool active {};
};

vector<string> active_names(const vector<User>& users)
{
    auto filtered_names = users
        | views::filter([](const User& user) { return user.active; })
        | views::transform([](const User& user) { return user.name; });

    return vector<string>(filtered_names.begin(), filtered_names.end()); // materialize the view
}
```
