#include "Testing/Testing.hpp"

#include <format>

#include "physics/physics.hpp"

TEST("force quantity")
{
    auto force = physics::force::calculate(physics::mass::kilograms(80), physics::acceleration::meters_per_second_squared(1));
    auto zero_mass = physics::mass::kilograms(0);

    CHECK(force == physics::force::newtons(80));
    CHECK(!force.checked_div_mass(zero_mass).has_value());
    CHECK(std::format("{}", force.display_kilonewtons_precision(3)) == "0.080 kN");
}
