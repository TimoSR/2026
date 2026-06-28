#include "Testing/Testing.hpp"

#include <format>
#include <limits>
#include <optional>

import aztro_physics;

TEST("length quantity")
{
    auto infinity = std::numeric_limits<double>::infinity();
    auto distance = aztro::physics::length::meters(100);
    auto zero_time = aztro::physics::time::seconds(0);

    CHECK(aztro::physics::length::meters(1) == aztro::physics::length::centimeters(100));
    CHECK(aztro::physics::length::centimeters(100) == aztro::physics::length::millimeters(1'000));
    CHECK(aztro::physics::length::try_meters(infinity) == std::nullopt);
    CHECK(aztro::physics::length::try_centimeters(100).value() == aztro::physics::length::meters(1));
    CHECK(std::format("{}", distance.display_centimeters()) == "10000 cm");
    CHECK(!distance.checked_div_time(zero_time).has_value());
    CHECK(distance.checked_div_time(aztro::physics::time::seconds(2)).has_value());
}
