#include "Testing/Testing.hpp"

#include "physics/physics.hpp"

TEST("acceleration quantity")
{
    auto velocity = physics::velocity::meters_per_second(10);
    auto elapsed = physics::time::seconds(10);
    auto acceleration = physics::acceleration::calculate(velocity, elapsed);

    CHECK(acceleration == physics::acceleration::meters_per_second_squared(1));
    CHECK(!physics::acceleration::checked_calculate(velocity, physics::time::seconds(0)).has_value());
}
