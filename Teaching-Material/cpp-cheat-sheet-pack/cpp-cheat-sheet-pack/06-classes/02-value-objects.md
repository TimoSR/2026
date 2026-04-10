# Value objects

```cpp
#include <stdexcept>
#include <string>

using namespace std;

class EmailAddress
{
public:
    explicit EmailAddress(string value)
        : m_value { move(value) }
    {
        if (m_value.find('@') == string::npos) {
            throw invalid_argument("invalid email");
        }
    }

    friend bool operator==(const EmailAddress&, const EmailAddress&) = default;

private:
    string m_value;
};
```
