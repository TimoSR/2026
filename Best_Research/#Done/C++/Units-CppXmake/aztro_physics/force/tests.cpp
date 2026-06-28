#include "Testing/Testing.hpp"

#include <format>

import aztro_physics;

TEST("force quantity")
{
    auto force = aztro::physics::force::calculate(aztro::physics::mass::kilograms(80), aztro::physics::acceleration::meters_per_second_squared(1));
    auto zero_mass = aztro::physics::mass::kilograms(0);

    CHECK(force == aztro::physics::force::newtons(80));
    CHECK(!force.checked_div_mass(zero_mass).has_value());
    CHECK(std::format("{}", force.display_kilonewtons_precision(3)) == "0.080 kN");
}
