#include "Testing/Testing.hpp"

#include <limits>

#include "physics/physics.hpp"

TEST("mass quantity")
{
    auto infinity = std::numeric_limits<double>::infinity();

    CHECK(physics::mass::kilogram(1) == physics::mass::grams(1'000));
    CHECK(physics::mass::kilogram(1) == physics::mass::milligrams(1'000'000));
    CHECK(physics::mass::try_kilograms(infinity) == std::nullopt);
}
