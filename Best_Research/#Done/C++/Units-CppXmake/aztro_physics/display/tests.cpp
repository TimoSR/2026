#include "Testing/Testing.hpp"

#include <format>
#include <string>

import aztro_physics;

TEST("std::format support")
{
    auto distance = aztro::physics::length::centimeters(10'000);
    auto elapsed = aztro::physics::time::milliseconds(9'580);
    auto velocity = distance / elapsed;

    CHECK(std::format("{}", distance.display_as(aztro::physics::length::LengthUnit::Meters)) == "100 m");
    CHECK(std::format("{}", velocity.display_as_precision(aztro::physics::velocity::VelocityUnit::KilometersPerHour, 2)) == "37.58 km/h");
    CHECK(std::format("{}", velocity).find("m/s") != std::string::npos);
}
