#include "Testing/test_context.hpp"

#include <cstdlib>
#include <vector>

namespace Testing
{

    std::vector<TestCase>& registered_tests()
    {
        static std::vector<TestCase> test_cases;
        return test_cases;
    }

    TestContext*& current_test_context()
    {
        static TestContext* test_context = nullptr;
        return test_context;
    }

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

    std::ostream& TestContext::output()
    {
        return output_;
    }

    TestRegistration::TestRegistration(std::string_view name, TestFunction run)
    {
        register_test({name, run});
    }

    void register_test(TestCase test_case)
    {
        registered_tests().push_back(test_case);
    }

    void check(bool condition, std::string_view expression, std::source_location location)
    {
        TestContext* test_context = current_test_context();

        if (test_context == nullptr)
        {
            std::cerr << "CHECK used outside a running test\n";
            std::abort();
        }

        test_context->check(condition, expression, location);
    }

    bool run_test(TestCase test_case, TestContext& test_context)
    {
        int failed_checks_before_test = test_context.failed_checks();
        int checks_run_before_test = test_context.checks_run();

        current_test_context() = &test_context;
        test_case.run();
        current_test_context() = nullptr;

        int checks_run_in_test = test_context.checks_run() - checks_run_before_test;
        bool passed = test_context.failed_checks() == failed_checks_before_test;

        test_context.output() << (passed ? "PASS " : "FAIL ") << test_case.name << " (" << checks_run_in_test << " checks)\n";

        return passed;
    }

    int run_tests(std::span<TestCase> test_cases, std::ostream& output)
    {
        TestContext test_context(output);
        int passed_tests = 0;
        int failed_tests = 0;

        for (TestCase test_case : test_cases)
        {
            if (run_test(test_case, test_context))
            {
                passed_tests += 1;
            }
            else
            {
                failed_tests += 1;
            }
        }

        output << "\n";
        output << "Tests: " << passed_tests << " passed, " << failed_tests << " failed\n";
        output << "Checks: " << test_context.checks_run() - test_context.failed_checks() << " passed, " << test_context.failed_checks() << " failed\n";

        return failed_tests == 0 ? 0 : 1;
    }

    int run_tests(std::ostream& output)
    {
        return run_tests(registered_tests(), output);
    }

} // namespace Testing
