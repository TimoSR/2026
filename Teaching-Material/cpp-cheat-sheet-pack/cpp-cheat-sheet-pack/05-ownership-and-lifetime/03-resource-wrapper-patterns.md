# Resource wrapper patterns

```cpp
#include <cstdio>
#include <memory>
#include <stdexcept>
#include <string>

using namespace std;

class File
{
public:
    explicit File(const char* path)
        : m_handle { fopen(path, "rb"), &fclose } // wrap resource immediately
    {
        if (!m_handle) {
            throw runtime_error("failed to open file");
        }
    }

private:
    unique_ptr<FILE, int(*)(FILE*)> m_handle;
};
```
