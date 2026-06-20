#include <map>
#include <string>
#include <vector>

using namespace std;

double averageScore(vector<int> scores) {
    if (scores.empty()) {
        return 0.0;
    }

    int totalScore = 0;
    for (int score : scores) {
        totalScore += score;
    }

    return static_cast<double>(totalScore) / scores.size();
}

map<string, int> countWords(vector<string> words) {
    map<string, int> counts;

    for (string word : words) {
        // map[key] creates key with 0 first time, then increments.
        counts[word] += 1;
    }

    return counts;
}

string topStudentName(map<string, int> studentScores) {
    string bestName = "";
    int bestScore = -1;

    for (pair<string, int> entry : studentScores) {
        if (entry.second > bestScore) {
            bestScore = entry.second;
            bestName = entry.first;
        }
    }

    return bestName;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    using namespace std;

    cout << averageScore({75, 90, 100}) << "\n";

    map<string, int> wordCounts = countWords({"cat", "dog", "cat"});
    cout << "cat count: " << wordCounts["cat"] << "\n";

    cout << topStudentName({{"Ava", 91}, {"Noah", 97}, {"Liam", 88}}) << "\n";
    return 0;
}
#endif
