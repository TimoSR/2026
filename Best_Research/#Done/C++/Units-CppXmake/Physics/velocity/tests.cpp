#include "Physics/tests/test_context.hpp"

#include "Physics/Physics.hpp"

namespace test
{

    void test_velocity_quantity(TestContext& test_context)
    {
        auto distance = Physics::length::meters(100);
        auto elapsed = Physics::time::seconds(10);
        auto velocity = Physics::velocity::calculate(distance, elapsed);

        EXPECT(test_context, velocity == Physics::velocity::meters_per_second(10));
        EXPECT(test_context, !Physics::velocity::checked_calculate(distance, Physics::time::seconds(0)).has_value());
    }

} // namespace test
