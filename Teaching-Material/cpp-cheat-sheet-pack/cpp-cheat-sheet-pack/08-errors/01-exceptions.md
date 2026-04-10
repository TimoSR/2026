# Exceptions

```cpp
#include <iostream>
#include <stdexcept>
#include <string>

using namespace std;

string load_config_text(bool file_missing)
{
    if (file_missing) {
        throw runtime_error("config file missing");
    }

    return "port=8080";
}

int main()
{
    try {
        string config_text = load_config_text(true);
        cout << config_text << '\n';
    } catch (const exception& error) {
        cout << "error: " << error.what() << '\n';
    }
}
```
