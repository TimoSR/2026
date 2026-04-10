# Result types

```cpp
#include <expected>
#include <string>

using namespace std;

enum class LoginError
{
    EmptyUsername,
    WrongPassword,
};

expected<string, LoginError> login(const string& user_name, const string& password)
{
    if (user_name.empty()) {
        return unexpected(LoginError::EmptyUsername);
    }

    if (password != "secret") {
        return unexpected(LoginError::WrongPassword);
    }

    return "session-token";
}
```
