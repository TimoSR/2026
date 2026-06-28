#include "Testing/Testing.hpp"

#include <limits>

#include "Physics/Physics.hpp"

TEST("mass quantity")
{
    auto infinity = std::numeric_limits<double>::infinity();

    CHECK(Physics::mass::kilogram(1) == Physics::mass::grams(1'000));
    CHECK(Physics::mass::kilogram(1) == Physics::mass::milligrams(1'000'000));
    CHECK(Physics::mass::try_kilograms(infinity) == std::nullopt);
}
