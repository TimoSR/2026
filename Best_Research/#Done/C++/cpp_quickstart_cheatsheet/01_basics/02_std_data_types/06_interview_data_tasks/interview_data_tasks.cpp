#include <map>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

string trimSpaces(string text) {
    size_t start = text.find_first_not_of(" \t");
    if (start == string::npos) {
        return "";
    }
    size_t end = text.find_last_not_of(" \t");
    return text.substr(start, end - start + 1);
}

map<string, int> frequencyByWord(vector<string> words) {
    map<string, int> frequency;
    for (string word : words) {
        string normalized = trimSpaces(word);
        if (!normalized.empty()) {
            frequency[normalized] += 1;
        }
    }
    return frequency;
}

vector<int> valuesAbove(vector<int> values, int threshold) {
    vector<int> filtered;
    for (int value : values) {
        if (value > threshold) {
            filtered.push_back(value);
        }
    }
    return filtered;
}

pair<int, int> evenOddCount(vector<int> values) {
    int evenCount = 0;
    int oddCount = 0;

    for (int value : values) {
        if (value % 2 == 0) {
            evenCount += 1;
        } else {
            oddCount += 1;
        }
    }
    return make_pair(evenCount, oddCount);
}

tuple<int, int, int> top3OrZero(vector<int> values) {
    int first = 0;
    int second = 0;
    int third = 0;

    for (int value : values) {
        if (value > first) {
            third = second;
            second = first;
            first = value;
        } else if (value > second) {
            third = second;
            second = value;
        } else if (value > third) {
            third = value;
        }
    }
    return make_tuple(first, second, third);
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    map<string, int> frequency = frequencyByWord({"cpp", "docs", "cpp", " interview "});
    cout << "cpp count: " << frequency["cpp"] << "\n";

    vector<int> filtered = valuesAbove({10, 30, 5, 42}, 20);
    cout << "valuesAbove count: " << filtered.size() << "\n";

    pair<int, int> evenOdd = evenOddCount({1, 2, 3, 4, 5, 6});
    cout << "even/odd: " << evenOdd.first << "/" << evenOdd.second << "\n";

    tuple<int, int, int> top3 = top3OrZero({42, 7, 99, 18, 56});
    cout << "top3: " << get<0>(top3) << ", " << get<1>(top3) << ", " << get<2>(top3) << "\n";
    return 0;
}
#endif
