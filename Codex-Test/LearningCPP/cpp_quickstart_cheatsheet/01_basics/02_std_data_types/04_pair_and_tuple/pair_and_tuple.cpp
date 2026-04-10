#include <string>
#include <tuple>

using namespace std;

pair<string, int> userWithScore(string userName, int score) {
    return make_pair(userName, score);
}

tuple<string, int, bool> accountSummary(string userName, int points, bool isActive) {
    return make_tuple(userName, points, isActive);
}

string formatUserScore(pair<string, int> userScore) {
    return userScore.first + " => " + to_string(userScore.second);
}

bool isAccountActive(tuple<string, int, bool> summary) {
    return get<2>(summary);
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    pair<string, int> userScore = userWithScore("Nora", 95);
    tuple<string, int, bool> summary = accountSummary("Nora", 1200, true);

    cout << formatUserScore(userScore) << "\n";
    cout << "Active: " << isAccountActive(summary) << "\n";
    return 0;
}
#endif
