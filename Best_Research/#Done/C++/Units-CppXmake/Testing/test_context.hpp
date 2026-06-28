#pragma once

#include <cmath>
#include <iostream>
#include <source_location>
#include <span>
#include <string_view>

namespace Testing
{

    class TestContext;

    using TestFunction = void (*)();

    bool approximately_equal(double left, double right, double epsilon);

    class TestContext
    {
        public:
            explicit TestContext(std::ostream& output);

            void check(bool condition, std::string_view expression, std::source_location location = std::source_location::current());

            int checks_run();

            int failed_checks();

            std::ostream& output();

        private:
            std::ostream& output_;
            int checks_run_ = 0;
            int failed_checks_ = 0;
    };

    struct TestCase
    {
            std::string_view name;
            TestFunction run;
    };

    class TestRegistration
    {
        public:
            TestRegistration(std::string_view name, TestFunction run);
    };

    void register_test(TestCase test_case);

    void check(bool condition, std::string_view expression, std::source_location location = std::source_location::current());

    bool run_test(TestCase test_case, TestContext& test_context);
    int run_tests(std::span<TestCase> test_cases, std::ostream& output);
    int run_tests(std::ostream& output);

} // namespace Testing

#define CHECK(expression) Testing::check((expression), #expression, std::source_location::current())

#define TESTING_JOIN(left, right) TESTING_JOIN_DETAIL(left, right)
#define TESTING_JOIN_DETAIL(left, right) left##right

// clang-format off
#define TEST(name) TESTING_TEST_CASE(name, __COUNTER__)
#define TEST_CASE(name) TESTING_TEST_CASE(name, __COUNTER__)
#define TESTING_TEST_CASE(name, id)                                                                                 \
    static void TESTING_JOIN(test_case_, id)();                                                                      \
    static Testing::TestRegistration TESTING_JOIN(test_registration_, id)(name, TESTING_JOIN(test_case_, id));       \
    static void TESTING_JOIN(test_case_, id)()
// clang-format on
