#include "Testing/Testing.hpp"

import aztro_physics;

TEST("acceleration quantity")
{
    auto velocity = aztro::physics::velocity::meters_per_second(10);
    auto elapsed = aztro::physics::time::seconds(10);
    auto acceleration = aztro::physics::acceleration::calculate(velocity, elapsed);

    CHECK(acceleration == aztro::physics::acceleration::meters_per_second_squared(1));
    CHECK(!aztro::physics::acceleration::checked_calculate(velocity, aztro::physics::time::seconds(0)).has_value());
}
