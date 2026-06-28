#include "Testing/Testing.hpp"

#include <format>
#include <limits>

#include "Physics/Physics.hpp"

TEST("time quantity")
{
    auto infinity = std::numeric_limits<double>::infinity();
    auto elapsed = Physics::time::seconds(10);

    CHECK(Physics::time::seconds(1) == Physics::time::milliseconds(1'000));
    CHECK(Physics::time::seconds(1) == Physics::time::microseconds(1'000'000));
    CHECK(Physics::time::try_seconds(infinity) == std::nullopt);
    CHECK(std::format("{}", elapsed.display_minutes_precision(2)) == "0.17 min");
}
