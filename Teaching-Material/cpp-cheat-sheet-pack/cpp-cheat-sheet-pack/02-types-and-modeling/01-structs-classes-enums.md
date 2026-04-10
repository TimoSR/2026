# Structs, classes, enums

```cpp
#include <string>
#include <vector>

using namespace std;

struct UserDto
{
    int id {};
    string name;
    vector<string> roles; // plain transport shape
};
```

```cpp
#include <string>

using namespace std;

enum class OrderStatus
{
    Draft,
    Paid,
    Shipped,
    Cancelled,
};

string to_string(OrderStatus order_status)
{
    switch (order_status) {
        case OrderStatus::Draft: return "Draft";
        case OrderStatus::Paid: return "Paid";
        case OrderStatus::Shipped: return "Shipped";
        case OrderStatus::Cancelled: return "Cancelled";
    }

    return "Unknown";
}
```

```cpp
#include <string>

using namespace std;

class Product
{
public:
    Product(string name, double price)
        : m_name { move(name) }
        , m_price { price }
    {
    }

private:
    string m_name;
    double m_price {};
};
```
