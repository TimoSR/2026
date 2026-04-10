# jthread and mutex

```cpp
#include <chrono>
#include <stop_token>
#include <thread>

using namespace std;
using namespace chrono;

class Worker
{
public:
    Worker()
        : m_thread { [this](stop_token stop_token_value) { run(stop_token_value); } }
    {
    }

private:
    void run(stop_token stop_token_value)
    {
        while (!stop_token_value.stop_requested()) {
            this_thread::sleep_for(100ms);
        }
    }

    jthread m_thread; // joins automatically
};
```
