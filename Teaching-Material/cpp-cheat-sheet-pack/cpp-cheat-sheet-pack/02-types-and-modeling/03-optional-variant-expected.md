# Optional, variant, expected

```cpp
#include <optional>
#include <string>
#include <vector>

using namespace std;

struct User
{
    int id {};
    string name;
};

optional<User> find_user(const vector<User>& users, int wanted_id)
{
    for (const User& user : users) {
        if (user.id == wanted_id) {
            return user;
        }
    }

    return nullopt;
}
```

```cpp
#include <string>
#include <variant>

using namespace std;

using ConfigValue = variant<int, double, string>;
```

```cpp
#include <expected>
#include <string_view>

using namespace std;

enum class ParseError
{
    EmptyInput,
    InvalidDigit,
};

expected<int, ParseError> parse_positive_int(string_view text)
{
    if (text.empty()) {
        return unexpected(ParseError::EmptyInput);
    }

    int value = 0;

    for (char character : text) {
        if (character < '0' || character > '9') {
            return unexpected(ParseError::InvalidDigit);
        }

        value = (value * 10) + (character - '0');
    }

    return value;
}
```
