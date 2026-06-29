#include "Testing/test_context.hpp"

#include <format>
#include <limits>

#include "physics/physics.hpp"

TEST("length quantity")
{
    auto infinity = std::numeric_limits<double>::infinity();
    auto distance = physics::length::meters(100);
    auto zero_time = physics::time::seconds(0);

    CHECK(physics::length::meters(1) == physics::length::centimeters(100));
    CHECK(physics::length::centimeters(100) == physics::length::millimeters(1'000));
    CHECK(physics::length::try_meters(infinity) == std::nullopt);
    CHECK(physics::length::try_centimeters(100).value() == physics::length::meters(1));
    CHECK(std::format("{}", distance.display_centimeters()) == "10000 cm");
    CHECK(!distance.checked_div_time(zero_time).has_value());
    CHECK(distance.checked_div_time(physics::time::seconds(2)).has_value());
}
