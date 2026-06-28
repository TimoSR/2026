#include "Physics/tests/test_context.hpp"

#include <format>
#include <limits>

#include "Physics/Physics.hpp"

namespace test
{

    void test_length_quantity(TestContext& test_context)
    {
        auto infinity = std::numeric_limits<double>::infinity();
        auto distance = Physics::length::meters(100);
        auto zero_time = Physics::time::seconds(0);

        EXPECT(test_context, Physics::length::meters(1) == Physics::length::centimeters(100));
        EXPECT(test_context, Physics::length::centimeters(100) == Physics::length::millimeters(1'000));
        EXPECT(test_context, Physics::length::try_meters(infinity) == std::nullopt);
        EXPECT(test_context, Physics::length::try_centimeters(100).value() == Physics::length::meters(1));
        EXPECT(test_context, std::format("{}", distance.display_centimeters()) == "10000 cm");
        EXPECT(test_context, !distance.checked_div_time(zero_time).has_value());
        EXPECT(test_context, distance.checked_div_time(Physics::time::seconds(2)).has_value());
    }

} // namespace test
