#include "Physics/tests/test_context.hpp"

#include "Physics/Physics.hpp"

namespace test
{

    void test_acceleration_quantity(TestContext& test_context)
    {
        auto velocity = Physics::velocity::meters_per_second(10);
        auto elapsed = Physics::time::seconds(10);
        auto acceleration = Physics::acceleration::calculate(velocity, elapsed);

        EXPECT(test_context, acceleration == Physics::acceleration::meters_per_second_squared(1));
        EXPECT(test_context, !Physics::acceleration::checked_calculate(velocity, Physics::time::seconds(0)).has_value());
    }

} // namespace test
