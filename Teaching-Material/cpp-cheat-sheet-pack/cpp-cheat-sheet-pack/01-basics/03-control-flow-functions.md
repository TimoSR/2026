# Control flow and functions

```cpp
#include <string>
#include <vector>

using namespace std;

bool is_adult(int age)
{
    return age >= 18;
}

int sum_numbers(const vector<int>& numbers)
{
    int total = 0;

    for (int number : numbers) {
        total += number;
    }

    return total;
}
```

```cpp
#include <string>

using namespace std;

string classify_score(int score)
{
    if (score >= 90) { return "excellent"; }
    if (score >= 70) { return "good"; }
    if (score >= 50) { return "pass"; }
    return "fail";
}
```
