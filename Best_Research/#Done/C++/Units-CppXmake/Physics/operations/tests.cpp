#include "Testing/Testing.hpp"

#include "Physics/Physics.hpp"

template <typename Left, typename Right>
concept CanDivide = requires(Left left, Right right) { left / right; };

TEST("equation rules ignore input scale")
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

    CHECK(Testing::approximately_equal(velocity.to_meters_per_second(), 100.0 / 9.58, 1e-12));
    CHECK(Testing::approximately_equal(acceleration.to_meters_per_second_squared(), (100.0 / 9.58) / 9.58, 1e-12));
    CHECK(Testing::approximately_equal(force.to_newtons(), 80.0 * ((100.0 / 9.58) / 9.58), 1e-12));
}

TEST("unsupported operations are omitted")
{
    static_assert(!CanDivide<Physics::velocity::Velocity, Physics::mass::Mass>);
}
