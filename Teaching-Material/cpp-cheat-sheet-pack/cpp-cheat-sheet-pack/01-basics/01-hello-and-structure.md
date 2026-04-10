# Hello and structure

```cpp
#include <iostream>
#include <string>

using namespace std;

int main()
{
    string name = "world";
    cout << "hello, " << name << '\n'; // simplest shape
}
```

```cpp
#include <iostream>
#include <string>
#include <vector>

using namespace std;

int main()
{
    vector<string> names { "anna", "bob", "chris" };

    for (const string& name : names) { // avoid copying each string
        cout << name << '\n';
    }
}
```
