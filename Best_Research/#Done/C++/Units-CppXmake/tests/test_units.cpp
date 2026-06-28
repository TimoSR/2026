#include <cmath>
#include <format>
#include <iostream>
#include <source_location>
#include <string>
#include <string_view>

#include "units/units.hpp"

namespace test
{

    template <typename Left, typename Right>
    concept CanDivide = requires(Left left, Right right) { left / right; };

    bool approximatelyEqual(double left, double right, double epsilon)
    {
        return std::fabs(left - right) <= epsilon;
    }

    class TestContext
    {
        public:
            explicit TestContext(std::ostream& output) : output_(output)
            {
            }

            void check(bool condition, std::string_view expression, std::source_location location = std::source_location::current())
            {
                checksRun_ += 1;

                if (condition)
                {
                    return;
                }

                failedChecks_ += 1;
                output_ << "    FAIL " << location.file_name() << ':' << location.line() << " `" << expression << "`\n";
            }

            int checksRun()
            {
                return checksRun_;
            }

            int failedChecks()
            {
                return failedChecks_;
            }

        private:
            std::ostream& output_;
            int checksRun_ = 0;
            int failedChecks_ = 0;
    };

#define EXPECT(testContext, expression) (testContext).check((expression), #expression, std::source_location::current())

    void testUnitNormalization(TestContext& testContext)
    {
        EXPECT(testContext, units::length::meters(1) == units::length::centimeters(100));
        EXPECT(testContext, units::length::centimeters(100) == units::length::millimeters(1'000));
        EXPECT(testContext, units::time::seconds(1) == units::time::milliseconds(1'000));
        EXPECT(testContext, units::time::seconds(1) == units::time::microseconds(1'000'000));
        EXPECT(testContext, units::mass::kilogram(1) == units::mass::grams(1'000));
        EXPECT(testContext, units::mass::kilogram(1) == units::mass::milligrams(1'000'000));
    }

    void testEquationRulesIgnoreInputScale(TestContext& testContext)
    {
        using units::Acceleration;
        using units::Force;
        using units::Mass;
        using units::Velocity;

        auto distance = units::length::centimeters(10'000);
        auto time = units::time::milliseconds(9'580);
        Mass mass = units::mass::grams(80'000);

        Velocity velocity = distance / time;
        Acceleration acceleration = velocity / time;
        Force force = mass * acceleration;

        EXPECT(testContext, approximatelyEqual(velocity.asMetersPerSecond(), 100.0 / 9.58, 1e-12));
        EXPECT(testContext, approximatelyEqual(acceleration.asMetersPerSecondSquared(), (100.0 / 9.58) / 9.58, 1e-12));
        EXPECT(testContext, approximatelyEqual(force.asNewtons(), 80.0 * ((100.0 / 9.58) / 9.58), 1e-12));
    }

    void testUnsupportedOperationsAreOmitted(TestContext&)
    {
        static_assert(!CanDivide<units::Velocity, units::Mass>);
    }

    void testCheckedDivision(TestContext& testContext)
    {
        auto zeroTime = units::time::seconds(0);
        auto distance = units::length::meters(10);
        auto force = units::force::newtons(10);
        auto zeroMass = units::mass::kilograms(0);

        EXPECT(testContext, !distance.checkedDivTime(zeroTime).has_value());
        EXPECT(testContext, !force.checkedDivMass(zeroMass).has_value());
        EXPECT(testContext, distance.checkedDivTime(units::time::seconds(2)).has_value());
    }

    void testStdFormatSupport(TestContext& testContext)
    {
        auto distance = units::length::centimeters(10'000);
        auto elapsed = units::time::milliseconds(9'580);
        auto velocity = distance / elapsed;

        EXPECT(testContext, std::format("{}", distance.displayAs(units::LengthUnit::Meters)) == "100 m");
        EXPECT(testContext, std::format("{}", velocity.displayAsPrecision(units::VelocityUnit::KilometersPerHour, 2)) == "37.58 km/h");
        EXPECT(testContext, std::format("{}", velocity).find("m/s") != std::string::npos);
    }

    struct TestCase
    {
            std::string_view name;
            void (*run)(TestContext&);
    };

    bool runTest(TestCase testCase, TestContext& testContext)
    {
        int failedChecksBeforeTest = testContext.failedChecks();
        int checksRunBeforeTest = testContext.checksRun();

        testCase.run(testContext);

        int checksRunInTest = testContext.checksRun() - checksRunBeforeTest;
        bool passed = testContext.failedChecks() == failedChecksBeforeTest;

        std::cout << (passed ? "PASS " : "FAIL ") << testCase.name << " (" << checksRunInTest << " checks)\n";

        return passed;
    }

} // namespace test

int main()
{
    test::TestContext testContext(std::cout);
    int passedTests = 0;
    int failedTests = 0;

    test::TestCase testCases[] = {
        {"unit normalization", test::testUnitNormalization},
        {"equation rules ignore input scale", test::testEquationRulesIgnoreInputScale},
        {"unsupported operations are omitted", test::testUnsupportedOperationsAreOmitted},
        {"checked division", test::testCheckedDivision},
        {"std::format support", test::testStdFormatSupport},
    };

    for (test::TestCase testCase : testCases)
    {
        if (test::runTest(testCase, testContext))
        {
            passedTests += 1;
        }
        else
        {
            failedTests += 1;
        }
    }

    std::cout << "\n";
    std::cout << "Tests: " << passedTests << " passed, " << failedTests << " failed\n";
    std::cout << "Checks: " << testContext.checksRun() - testContext.failedChecks() << " passed, " << testContext.failedChecks() << " failed\n";

    return failedTests == 0 ? 0 : 1;
}
