#include "Physics/tests/test_context.hpp"

#include <format>
#include <string>

#include "Physics/Physics.hpp"

namespace test
{

    void test_std_format_support(TestContext& test_context)
    {
        auto distance = Physics::length::centimeters(10'000);
        auto elapsed = Physics::time::milliseconds(9'580);
        auto velocity = distance / elapsed;

        EXPECT(test_context, std::format("{}", distance.display_as(Physics::length::LengthUnit::Meters)) == "100 m");
        EXPECT(test_context, std::format("{}", velocity.display_as_precision(Physics::velocity::VelocityUnit::KilometersPerHour, 2)) == "37.58 km/h");
        EXPECT(test_context, std::format("{}", velocity).find("m/s") != std::string::npos);
    }

} // namespace test
