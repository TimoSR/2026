# Stack and RAII

```cpp
#include <fstream>
#include <mutex>
#include <string>
#include <vector>

using namespace std;

void write_log_line(const string& path, const string& line)
{
    ofstream file { path }; // opens here
    file << line << '\n';  // closes automatically
}
```

```cpp
class ThreadSafeMessages
{
public:
    void add(string message)
    {
        lock_guard<mutex> lock { m_mutex };
        m_messages.push_back(move(message));
    }

private:
    mutex m_mutex;
    vector<string> m_messages;
};
```
