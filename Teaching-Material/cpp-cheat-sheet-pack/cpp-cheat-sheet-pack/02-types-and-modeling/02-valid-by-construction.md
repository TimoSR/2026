# Valid by construction

```cpp
#include <stdexcept>
#include <string>

using namespace std;

class Username
{
public:
    explicit Username(string value)
        : m_value { move(value) }
    {
        if (m_value.size() < 3) {
            throw invalid_argument("username too short"); // reject bad state early
        }
    }

private:
    string m_value;
};
```

```cpp
#include <stdexcept>
#include <string>

using namespace std;

class Money
{
public:
    Money(long long cents, string currency)
        : m_cents { cents }
        , m_currency { move(currency) }
    {
        if (m_currency.empty()) {
            throw invalid_argument("currency must not be empty");
        }
    }

private:
    long long m_cents {};
    string m_currency;
};
```
