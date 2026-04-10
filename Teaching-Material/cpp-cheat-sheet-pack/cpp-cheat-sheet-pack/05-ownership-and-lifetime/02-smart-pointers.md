# Smart pointers

```cpp
#include <memory>
#include <string>

using namespace std;

class Connection
{
public:
    explicit Connection(string name)
        : m_name { move(name) }
    {
    }

private:
    string m_name;
};

int main()
{
    unique_ptr<Connection> connection = make_unique<Connection>("primary"); // single owner
}
```

```cpp
#include <memory>
#include <string>

using namespace std;

int main()
{
    shared_ptr<string> environment = make_shared<string>("production"); // shared lifetime when needed
}
```
