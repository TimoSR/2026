# Rule of zero

```cpp
#include <string>
#include <vector>

using namespace std;

class Project
{
public:
    Project(string name, vector<string> files)
        : m_name { move(name) }
        , m_files { move(files) }
    {
    }

private:
    string m_name;
    vector<string> m_files; // standard members manage themselves
};
```
