#include "Physics/tests/test_context.hpp"

#include "Physics/Physics.hpp"

namespace test
{

    template <typename Left, typename Right>
    concept CanDivide = requires(Left left, Right right) { left / right; };

    void test_equation_rules_ignore_input_scale(TestContext& test_context)
    {
        using Physics::acceleration::Acceleration;
        using Physics::force::Force;
        using Physics::mass::Mass;
        using Physics::velocity::Velocity;

        auto distance = Physics::length::centimeters(10'000);
        auto time = Physics::time::milliseconds(9'580);
        Mass mass = Physics::mass::grams(80'000);

        Velocity velocity = distance / time;
        Acceleration acceleration = velocity / time;
        Force force = mass * acceleration;

        EXPECT(test_context, approximately_equal(velocity.to_meters_per_second(), 100.0 / 9.58, 1e-12));
        EXPECT(test_context, approximately_equal(acceleration.to_meters_per_second_squared(), (100.0 / 9.58) / 9.58, 1e-12));
        EXPECT(test_context, approximately_equal(force.to_newtons(), 80.0 * ((100.0 / 9.58) / 9.58), 1e-12));
    }

    void test_unsupported_operations_are_omitted(TestContext&)
    {
        static_assert(!CanDivide<Physics::velocity::Velocity, Physics::mass::Mass>);
    }

} // namespace test
