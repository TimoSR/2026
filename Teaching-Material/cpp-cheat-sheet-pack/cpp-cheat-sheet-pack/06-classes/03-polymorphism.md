# Polymorphism

```cpp
#include <memory>
#include <string>
#include <vector>

using namespace std;

class LoggerSink
{
public:
    virtual ~LoggerSink() = default;
    virtual void write(const string& message) = 0;
};

class MemorySink : public LoggerSink
{
public:
    void write(const string& message) override
    {
        m_messages.push_back(message);
    }

private:
    vector<string> m_messages;
};
```
