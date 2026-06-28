#include "Physics/tests/test_context.hpp"

#include <iostream>

namespace test
{

    bool approximately_equal(double left, double right, double epsilon)
    {
        return std::fabs(left - right) <= epsilon;
    }

    TestContext::TestContext(std::ostream& output) : output_(output)
    {
    }

    void TestContext::check(bool condition, std::string_view expression, std::source_location location)
    {
        checks_run_ += 1;

        if (condition)
        {
            return;
        }

        failed_checks_ += 1;
        output_ << "    FAIL " << location.file_name() << ':' << location.line() << " `" << expression << "`\n";
    }

    int TestContext::checks_run()
    {
        return checks_run_;
    }

    int TestContext::failed_checks()
    {
        return failed_checks_;
    }

    bool run_test(TestCase test_case, TestContext& test_context)
    {
        int failed_checks_before_test = test_context.failed_checks();
        int checks_run_before_test = test_context.checks_run();

        test_case.run(test_context);

        int checks_run_in_test = test_context.checks_run() - checks_run_before_test;
        bool passed = test_context.failed_checks() == failed_checks_before_test;

        std::cout << (passed ? "PASS " : "FAIL ") << test_case.name << " (" << checks_run_in_test << " checks)\n";

        return passed;
    }

    void test_length_quantity(TestContext& test_context);
    void test_time_quantity(TestContext& test_context);
    void test_mass_quantity(TestContext& test_context);
    void test_velocity_quantity(TestContext& test_context);
    void test_acceleration_quantity(TestContext& test_context);
    void test_force_quantity(TestContext& test_context);
    void test_equation_rules_ignore_input_scale(TestContext& test_context);
    void test_unsupported_operations_are_omitted(TestContext& test_context);
    void test_std_format_support(TestContext& test_context);

} // namespace test

int main()
{
    test::TestContext test_context(std::cout);
    int passed_tests = 0;
    int failed_tests = 0;

    test::TestCase test_cases[] = {
        {"length quantity", test::test_length_quantity},
        {"time quantity", test::test_time_quantity},
        {"mass quantity", test::test_mass_quantity},
        {"velocity quantity", test::test_velocity_quantity},
        {"acceleration quantity", test::test_acceleration_quantity},
        {"force quantity", test::test_force_quantity},
        {"equation rules ignore input scale", test::test_equation_rules_ignore_input_scale},
        {"unsupported operations are omitted", test::test_unsupported_operations_are_omitted},
        {"std::format support", test::test_std_format_support},
    };

    for (test::TestCase test_case : test_cases)
    {
        if (test::run_test(test_case, test_context))
        {
            passed_tests += 1;
        }
        else
        {
            failed_tests += 1;
        }
    }

    std::cout << "\n";
    std::cout << "Tests: " << passed_tests << " passed, " << failed_tests << " failed\n";
    std::cout << "Checks: " << test_context.checks_run() - test_context.failed_checks() << " passed, " << test_context.failed_checks() << " failed\n";

    return failed_tests == 0 ? 0 : 1;
}
