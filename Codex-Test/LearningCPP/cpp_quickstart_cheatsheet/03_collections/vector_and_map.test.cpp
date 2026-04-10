#include <cassert>
#include <cmath>
#include <iostream>
#include <map>
#include <string>
#include <vector>

using namespace std;

#include "vector_and_map.cpp"

int main() {
    vector<int> scoreList = {60, 70, 80, 90};
    double average = averageScore(scoreList);
    assert(abs(average - 75.0) < 0.000001);

    vector<int> emptyScoreList = {};
    double emptyAverage = averageScore(emptyScoreList);
    assert(emptyAverage == 0.0);

    vector<string> words = {"apple", "banana", "apple", "apple", "banana"};
    map<string, int> wordCounts = countWords(words);
    int appleCount = wordCounts["apple"];
    int bananaCount = wordCounts["banana"];
    assert(appleCount == 3);
    assert(bananaCount == 2);

    map<string, int> studentScores = {{"Mila", 84}, {"Iris", 99}, {"Omar", 92}};
    string topStudent = topStudentName(studentScores);
    assert(topStudent == "Iris");

    cout << "vector_and_map tests passed\n";
    return 0;
}
