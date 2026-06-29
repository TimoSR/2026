#include "Testing/test_context.hpp"

#include <format>
#include <limits>

#include "physics/physics.hpp"

TEST("time quantity")
{
    auto infinity = std::numeric_limits<double>::infinity();
    auto elapsed = physics::time::seconds(10);

    CHECK(physics::time::seconds(1) == physics::time::milliseconds(1'000));
    CHECK(physics::time::seconds(1) == physics::time::microseconds(1'000'000));
    CHECK(physics::time::try_seconds(infinity) == std::nullopt);
    CHECK(std::format("{}", elapsed.display_minutes_precision(2)) == "0.17 min");
}
