#include <tuple>
#include <vector>

using namespace std;

int cappedAge(int age) {
    if (age < 0) {
        return 0;
    }
    if (age > 130) {
        return 130;
    }
    return age;
}

long long sumValues(vector<int> values) {
    long long total = 0;
    for (int value : values) {
        total += value;
    }
    return total;
}

double averageValues(vector<int> values) {
    if (values.empty()) {
        return 0.0;
    }
    return static_cast<double>(sumValues(values)) / static_cast<double>(values.size());
}

pair<int, int> minMaxValues(vector<int> values) {
    if (values.empty()) {
        return make_pair(0, 0);
    }

    int minimum = values[0];
    int maximum = values[0];
    for (size_t index = 1; index < values.size(); index += 1) {
        if (values[index] < minimum) {
            minimum = values[index];
        }
        if (values[index] > maximum) {
            maximum = values[index];
        }
    }
    return make_pair(minimum, maximum);
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    vector<int> scores = {95, 88, 77, 100};
    pair<int, int> minMax = minMaxValues(scores);

    cout << "Capped age: " << cappedAge(150) << "\n";
    cout << "Sum: " << sumValues(scores) << "\n";
    cout << "Average: " << averageValues(scores) << "\n";
    cout << "Min/Max: " << minMax.first << "/" << minMax.second << "\n";
    return 0;
}
#endif
