#include "Physics/tests/test_context.hpp"

#include <limits>

#include "Physics/Physics.hpp"

namespace test
{

    void test_mass_quantity(TestContext& test_context)
    {
        auto infinity = std::numeric_limits<double>::infinity();

        EXPECT(test_context, Physics::mass::kilogram(1) == Physics::mass::grams(1'000));
        EXPECT(test_context, Physics::mass::kilogram(1) == Physics::mass::milligrams(1'000'000));
        EXPECT(test_context, Physics::mass::try_kilograms(infinity) == std::nullopt);
    }

} // namespace test
