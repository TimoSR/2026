#include "Testing/test_context.hpp"

#include <format>
#include <string>

#include "physics/physics.hpp"

TEST("std::format support")
{
    auto distance = physics::length::centimeters(10'000);
    auto elapsed = physics::time::milliseconds(9'580);
    auto velocity = distance / elapsed;

    CHECK(std::format("{}", distance.display_as(physics::length::LengthUnit::Meters)) == "100 m");
    CHECK(std::format("{}", velocity.display_as_precision(physics::velocity::VelocityUnit::KilometersPerHour, 2)) == "37.58 km/h");
    CHECK(std::format("{}", velocity).find("m/s") != std::string::npos);
}
