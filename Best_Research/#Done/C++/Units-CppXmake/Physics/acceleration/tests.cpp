#include "Testing/Testing.hpp"

#include "Physics/Physics.hpp"

TEST("acceleration quantity")
{
    auto velocity = Physics::velocity::meters_per_second(10);
    auto elapsed = Physics::time::seconds(10);
    auto acceleration = Physics::acceleration::calculate(velocity, elapsed);

    CHECK(acceleration == Physics::acceleration::meters_per_second_squared(1));
    CHECK(!Physics::acceleration::checked_calculate(velocity, Physics::time::seconds(0)).has_value());
}
