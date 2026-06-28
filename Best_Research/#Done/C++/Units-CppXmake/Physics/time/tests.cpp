#include "Physics/tests/test_context.hpp"

#include <format>
#include <limits>

#include "Physics/Physics.hpp"

namespace test
{

    void test_time_quantity(TestContext& test_context)
    {
        auto infinity = std::numeric_limits<double>::infinity();
        auto elapsed = Physics::time::seconds(10);

        EXPECT(test_context, Physics::time::seconds(1) == Physics::time::milliseconds(1'000));
        EXPECT(test_context, Physics::time::seconds(1) == Physics::time::microseconds(1'000'000));
        EXPECT(test_context, Physics::time::try_seconds(infinity) == std::nullopt);
        EXPECT(test_context, std::format("{}", elapsed.display_minutes_precision(2)) == "0.17 min");
    }

} // namespace test
