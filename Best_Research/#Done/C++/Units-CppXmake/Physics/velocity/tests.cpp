#include "Testing/test_context.hpp"

#include "physics/physics.hpp"

TEST("velocity quantity")
{
    auto distance = physics::length::meters(100);
    auto elapsed = physics::time::seconds(10);
    auto velocity = physics::velocity::calculate(distance, elapsed);

    CHECK(velocity == physics::velocity::meters_per_second(10));
    CHECK(!physics::velocity::checked_calculate(distance, physics::time::seconds(0)).has_value());
}
