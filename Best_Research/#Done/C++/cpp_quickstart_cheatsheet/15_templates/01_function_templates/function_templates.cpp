#include <string>
#include <vector>

using namespace std;

template <typename T>
T getMaxValue(T firstValue, T secondValue) {
    if (firstValue > secondValue) {
        return firstValue;
    }
    return secondValue;
}

template <typename T>
vector<T> repeatValue(T value, int repeatCount) {
    vector<T> result;
    for (int index = 0; index < repeatCount; index++) {
        result.push_back(value);
    }
    return result;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    cout << getMaxValue<int>(10, 30) << "\n";
    cout << getMaxValue<string>("A", "B") << "\n";

    vector<int> values = repeatValue<int>(7, 3);
    cout << values[0] << "," << values[1] << "," << values[2] << "\n";
    return 0;
}
#endif
