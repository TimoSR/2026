#include "Physics/tests/test_context.hpp"

#include <format>

#include "Physics/Physics.hpp"

namespace test
{

    void test_force_quantity(TestContext& test_context)
    {
        auto force = Physics::force::calculate(Physics::mass::kilograms(80), Physics::acceleration::meters_per_second_squared(1));
        auto zero_mass = Physics::mass::kilograms(0);

        EXPECT(test_context, force == Physics::force::newtons(80));
        EXPECT(test_context, !force.checked_div_mass(zero_mass).has_value());
        EXPECT(test_context, std::format("{}", force.display_kilonewtons_precision(3)) == "0.080 kN");
    }

} // namespace test
