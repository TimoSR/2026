#include <cassert>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

#include "conditions_and_loops.cpp"

int main() {
    int freezingTemperature = -3;
    string freezingCategory = categoryFromTemperature(freezingTemperature);
    assert(freezingCategory == "freezing");

    int coolTemperature = 10;
    string coolCategory = categoryFromTemperature(coolTemperature);
    assert(coolCategory == "cool");

    int warmTemperature = 24;
    string warmCategory = categoryFromTemperature(warmTemperature);
    assert(warmCategory == "warm");

    int hotTemperature = 35;
    string hotCategory = categoryFromTemperature(hotTemperature);
    assert(hotCategory == "hot");

    int fizzInput = 3;
    string fizzValue = fizzBuzzValue(fizzInput);
    assert(fizzValue == "Fizz");

    int buzzInput = 5;
    string buzzValue = fizzBuzzValue(buzzInput);
    assert(buzzValue == "Buzz");

    int fizzBuzzInput = 15;
    string fizzBuzzText = fizzBuzzValue(fizzBuzzInput);
    assert(fizzBuzzText == "FizzBuzz");

    int plainInput = 7;
    string plainValue = fizzBuzzValue(plainInput);
    assert(plainValue == "7");

    vector<int> firstList = {1, 9, 11, 14, 21};
    int firstEvenNumber = firstEvenNumberOrMinusOne(firstList);
    assert(firstEvenNumber == 14);

    vector<int> secondList = {1, 9, 11};
    int missingEvenNumber = firstEvenNumberOrMinusOne(secondList);
    assert(missingEvenNumber == -1);

    assert(gradeBucketFromScore(95) == "A");
    assert(gradeBucketFromScore(81) == "B");
    assert(gradeBucketFromScore(74) == "C");
    assert(gradeBucketFromScore(60) == "D");
    assert(gradeBucketFromScore(20) == "F");

    assert(weekdayNameFromNumber(1) == "Monday");
    assert(weekdayNameFromNumber(7) == "Sunday");
    assert(weekdayNameFromNumber(9) == "Invalid");

    assert(sumUntilLimitWithBreak({3, 3, 3, 3}, 8) == 9);
    assert(sumOnlyPositiveWithContinue({-5, 4, 0, 6, -1}) == 10);

    vector<int> countdown = countDownValues(4);
    assert(countdown.size() == 4);
    assert(countdown[0] == 4);
    assert(countdown[3] == 1);

    assert(attemptsNeededForPin("1234", "1234", 3) == 1);
    assert(attemptsNeededForPin("0000", "1234", 3) == 3);

    assert(indexOfNameOrMinusOne({"Nora", "Ava", "Liam"}, "Ava") == 1);
    assert(indexOfNameOrMinusOne({"Nora", "Ava", "Liam"}, "Mia") == -1);

    assert(hasAllowedRole({"viewer", "editor"}, {"admin", "editor"}) == true);
    assert(hasAllowedRole({"viewer"}, {"admin", "editor"}) == false);

    cout << "conditions_and_loops tests passed\n";
    return 0;
}
