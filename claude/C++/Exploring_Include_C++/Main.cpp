// Main.cpp - The "Dependency Injection" File
// Include each implementation once in this translation unit.
#include <iostream>
#include <print>

using std::cin, std::cout;
using std::println;

#define UNITY_BUILD

#include "Math.cpp"
#include "Physics.cpp"
import Audio;

int main()
{
    int input;
    int result1 = Math::Add(5, 3);
    int result2 = Physics::CalculateForce(5, 5);

    // There are lots of issues with modules
    // No intellisense
    // just overall bad support
    int volume = Audio::CalculateVolume(25, 3);
    int pan = Audio::CalculatePan(40, 80);

    println("Hello {}", "C++23");

    cout << result1 << "\n";
    cout << result2 << "\n";
    cout << volume << "\n";
    cout << pan << "\n";

    cin >> input;

    cout << input;
}