#include <cassert>
#include <format>
#include <string>

import aztro_physics;

int main()
{
    using namespace aztro::physics;

    auto distance = length::centimeters(10'000);
    auto elapsed = time::milliseconds(10'000);
    auto body_mass = mass::grams(80'000);

    auto velocity = distance / elapsed;
    auto acceleration = velocity / elapsed;
    auto force = body_mass * acceleration;

    assert(distance == length::meters(100.0));
    assert(elapsed == time::seconds(10.0));
    assert(velocity.approximately_equals(velocity::meters_per_second(10.0), 0.00001));
    assert(acceleration.approximately_equals(acceleration::meters_per_second_squared(1.0), 0.00001));
    assert(force.approximately_equals(force::newtons(80.0), 0.00001));

    auto formatted = std::format("{}", force.display_as_precision(force::ForceUnit::Kilonewtons, 3));
    assert(formatted == "0.080 kN");
}
