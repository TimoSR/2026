# Practical STL patterns

```cpp
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

struct User
{
    int id {};
    string name;
};

unordered_map<int, User> index_users_by_id(const vector<User>& users)
{
    unordered_map<int, User> result;

    for (const User& user : users) {
        result[user.id] = user;
    }

    return result;
}
```
