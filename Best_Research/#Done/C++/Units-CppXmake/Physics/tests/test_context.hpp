#pragma once

#include <cmath>
#include <iostream>
#include <source_location>
#include <string_view>

namespace test
{

    bool approximately_equal(double left, double right, double epsilon);

    class TestContext
    {
        public:
            explicit TestContext(std::ostream& output);

            void check(bool condition, std::string_view expression, std::source_location location = std::source_location::current());

            int checks_run();

            int failed_checks();

        private:
            std::ostream& output_;
            int checks_run_ = 0;
            int failed_checks_ = 0;
    };

    struct TestCase
    {
            std::string_view name;
            void (*run)(TestContext&);
    };

    bool run_test(TestCase test_case, TestContext& test_context);

} // namespace test

#define EXPECT(test_context, expression) (test_context).check((expression), #expression, std::source_location::current())
