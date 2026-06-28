#include "Testing/Testing.hpp"

#include <format>
#include <string>

#include "Physics/Physics.hpp"

TEST("std::format support")
{
    auto distance = Physics::length::centimeters(10'000);
    auto elapsed = Physics::time::milliseconds(9'580);
    auto velocity = distance / elapsed;

    CHECK(std::format("{}", distance.display_as(Physics::length::LengthUnit::Meters)) == "100 m");
    CHECK(std::format("{}", velocity.display_as_precision(Physics::velocity::VelocityUnit::KilometersPerHour, 2)) == "37.58 km/h");
    CHECK(std::format("{}", velocity).find("m/s") != std::string::npos);
}
