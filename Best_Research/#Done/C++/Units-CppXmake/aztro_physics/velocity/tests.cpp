#include "Testing/Testing.hpp"

import aztro_physics;

TEST("velocity quantity")
{
    auto distance = aztro::physics::length::meters(100);
    auto elapsed = aztro::physics::time::seconds(10);
    auto velocity = aztro::physics::velocity::calculate(distance, elapsed);

    CHECK(velocity == aztro::physics::velocity::meters_per_second(10));
    CHECK(!aztro::physics::velocity::checked_calculate(distance, aztro::physics::time::seconds(0)).has_value());
}
