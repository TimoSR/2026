# Thread-safe patterns

```cpp
#include <mutex>
#include <optional>
#include <string>
#include <vector>

using namespace std;

class ThreadSafeBuffer
{
public:
    void add(string value)
    {
        lock_guard<mutex> lock { m_mutex };
        m_values.push_back(move(value));
    }

    optional<string> pop_last()
    {
        lock_guard<mutex> lock { m_mutex };

        if (m_values.empty()) {
            return nullopt;
        }

        string value = move(m_values.back());
        m_values.pop_back();
        return value;
    }

private:
    mutex m_mutex;
    vector<string> m_values;
};
```
