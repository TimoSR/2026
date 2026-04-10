#include <cassert>
#include <string>
#include <tuple>

using namespace std;

#include "pair_and_tuple.cpp"

int main() {
    pair<string, int> userScore = userWithScore("Ava", 88);
    assert(userScore.first == "Ava");
    assert(userScore.second == 88);
    assert(formatUserScore(userScore) == "Ava => 88");

    tuple<string, int, bool> summary = accountSummary("Ava", 500, true);
    assert(get<0>(summary) == "Ava");
    assert(get<1>(summary) == 500);
    assert(get<2>(summary) == true);
    assert(isAccountActive(summary) == true);

    tuple<string, int, bool> inactiveSummary = accountSummary("Nora", 10, false);
    assert(isAccountActive(inactiveSummary) == false);
    return 0;
}
