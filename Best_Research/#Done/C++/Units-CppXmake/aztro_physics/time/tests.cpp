#include "Testing/test_context.hpp"

#include <format>
#include <limits>
#include <optional>

import aztro_physics;

TEST("time quantity")
{
    auto infinity = std::numeric_limits<double>::infinity();
    auto elapsed = aztro::physics::time::seconds(10);

    CHECK(aztro::physics::time::seconds(1) == aztro::physics::time::milliseconds(1'000));
    CHECK(aztro::physics::time::seconds(1) == aztro::physics::time::microseconds(1'000'000));
    CHECK(aztro::physics::time::try_seconds(infinity) == std::nullopt);
    CHECK(std::format("{}", elapsed.display_minutes_precision(2)) == "0.17 min");
}
