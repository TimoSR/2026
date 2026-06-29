#include "Testing/test_context.hpp"

#include <limits>
#include <optional>

import aztro_physics;

TEST("mass quantity")
{
    auto infinity = std::numeric_limits<double>::infinity();

    CHECK(aztro::physics::mass::kilogram(1) == aztro::physics::mass::grams(1'000));
    CHECK(aztro::physics::mass::kilogram(1) == aztro::physics::mass::milligrams(1'000'000));
    CHECK(aztro::physics::mass::try_kilograms(infinity) == std::nullopt);
}
