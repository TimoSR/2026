#include "Testing/Testing.hpp"

#include <format>
#include <limits>

#include "Physics/Physics.hpp"

TEST("length quantity")
{
    auto infinity = std::numeric_limits<double>::infinity();
    auto distance = Physics::length::meters(100);
    auto zero_time = Physics::time::seconds(0);

    CHECK(Physics::length::meters(1) == Physics::length::centimeters(100));
    CHECK(Physics::length::centimeters(100) == Physics::length::millimeters(1'000));
    CHECK(Physics::length::try_meters(infinity) == std::nullopt);
    CHECK(Physics::length::try_centimeters(100).value() == Physics::length::meters(1));
    CHECK(std::format("{}", distance.display_centimeters()) == "10000 cm");
    CHECK(!distance.checked_div_time(zero_time).has_value());
    CHECK(distance.checked_div_time(Physics::time::seconds(2)).has_value());
}
