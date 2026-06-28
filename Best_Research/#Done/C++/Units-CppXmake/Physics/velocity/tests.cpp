#include "Testing/Testing.hpp"

#include "Physics/Physics.hpp"

TEST("velocity quantity")
{
    auto distance = Physics::length::meters(100);
    auto elapsed = Physics::time::seconds(10);
    auto velocity = Physics::velocity::calculate(distance, elapsed);

    CHECK(velocity == Physics::velocity::meters_per_second(10));
    CHECK(!Physics::velocity::checked_calculate(distance, Physics::time::seconds(0)).has_value());
}
