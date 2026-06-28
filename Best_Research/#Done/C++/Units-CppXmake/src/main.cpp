#include "Physics/Physics.hpp"
#include <iostream>
#include <print>

import aztro_physics;

void run_physics_example()
{
    auto distance = ::Physics::length::centimeters(10'000);
    auto elapsed = ::Physics::time::milliseconds(9'580);
    auto body_mass = ::Physics::mass::grams(80'000);

    auto velocity = distance / elapsed;
    auto acceleration = velocity / elapsed;
    auto force = body_mass * acceleration;

    std::println("[Physics]");
    std::println("distance = {}", distance);
    std::println("time = {}", elapsed);
    std::println("mass = {}", body_mass);
    std::println("velocity = {}", velocity.display_as_precision(::Physics::velocity::VelocityUnit::KilometersPerHour, 2));
    std::println("acceleration = {}", acceleration);
    std::println("force = {}", force.display_as_precision(::Physics::force::ForceUnit::Kilonewtons, 4));
}

void run_aztro_physics_example()
{
    auto distance = ::aztro::physics::length::centimeters(10'000);
    auto elapsed = ::aztro::physics::time::milliseconds(9'580);
    auto body_mass = ::aztro::physics::mass::grams(80'000);

    auto velocity = distance / elapsed;
    auto acceleration = velocity / elapsed;
    auto force = body_mass * acceleration;

    std::println("[aztro::physics]");
    std::println("distance = {}", distance);
    std::println("time = {}", elapsed);
    std::println("mass = {}", body_mass);
    std::println("velocity = {}", velocity.display_as_precision(::aztro::physics::velocity::VelocityUnit::KilometersPerHour, 2));
    std::println("acceleration = {}", acceleration);
    std::println("force = {}", force.display_as_precision(::aztro::physics::force::ForceUnit::Kilonewtons, 4));
}

int main()
{

    // std::int32_t helo = 1;
    // std::double_t he = 2.0;

    // int dasd = 1;
    // double sqweqe = 2;

    // auto qweiqweoiqn = 'A';

    run_physics_example();
    std::println("");
    run_aztro_physics_example();

    // char greeting;

    std::cout << "Hello World!" << "\n";
    // std::cin >>  greeting;
    // std::cout << greeting << "\n";

    std::string text = "This is the first part of a very long string. "
                       "This is the second part. "
                       "This is the third part. "
                       "C++ joins these into one string at compile time.";

    std::cout << text << '\n';

    std::println("{}", text);

    std::println();

    std::cout << "This is a very long output printed with std::cout.\n"
              << "You can keep chaining many strings together using the << operator.\n"
              << "This is useful when you want to print a large block of text.\n"
              << "Each string here is separate in the source code, but the output appears continuous.\n"
              << "Line 1: C++ lets you split long output across multiple lines.\n"
              << "Line 2: This keeps your source code readable.\n"
              << "Line 3: You do not need one giant unreadable line.\n"
              << "Line 4: You can print text, numbers, variables, and expressions.\n"
              << "Line 5: std::cout is available in every modern C++ version.\n"
              << "Line 6: For very large generated output, loops are usually better.\n"
              << "Line 7: For fixed text, chained string literals work well.\n"
              << "Line 8: This is still one long cout statement.\n"
              << "Line 9: You can add newline characters with \\n.\n"
              << "Line 10: You can also use std::endl, but \\n is usually faster.\n"
              << "Line 11: This example intentionally prints a lot of text.\n"
              << "Line 12: The output continues here.\n"
              << "Line 13: And here.\n"
              << "Line 14: And here.\n"
              << "Line 15: And here.\n"
              << "Line 16: This is a long cout chain.\n"
              << "Line 17: It is valid C++.\n"
              << "Line 18: The semicolon ends the whole output statement.\n"
              << "Line 19: You only need std::cout once at the beginning.\n"
              << "Line 20: Done printing a very long cout.\n";

    std::println();

    return 0;
}
