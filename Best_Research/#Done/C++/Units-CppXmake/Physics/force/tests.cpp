#include "Testing/Testing.hpp"

#include <format>

#include "Physics/Physics.hpp"

TEST("force quantity")
{
    auto force = Physics::force::calculate(Physics::mass::kilograms(80), Physics::acceleration::meters_per_second_squared(1));
    auto zero_mass = Physics::mass::kilograms(0);

    CHECK(force == Physics::force::newtons(80));
    CHECK(!force.checked_div_mass(zero_mass).has_value());
    CHECK(std::format("{}", force.display_kilonewtons_precision(3)) == "0.080 kN");
}
