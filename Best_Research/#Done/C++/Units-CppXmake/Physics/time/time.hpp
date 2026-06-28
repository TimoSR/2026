#pragma once

#include <optional>

#include "Physics/detail/quantity_display.hpp"

namespace Physics::time
{

    enum class TimeUnit
    {
        Seconds,
        Milliseconds,
        Microseconds,
        Nanoseconds,
        Minutes,
        Hours,
    };

    class Time
    {
            double _seconds;

        public:
            static Time from_raw_si(double seconds);

            static Time seconds(double value);

            static std::optional<Time> try_seconds(double value);

            static Time milliseconds(double value);

            static std::optional<Time> try_milliseconds(double value);

            static Time microseconds(double value);

            static std::optional<Time> try_microseconds(double value);

            static Time nanoseconds(double value);

            static std::optional<Time> try_nanoseconds(double value);

            static Time minutes(double value);

            static std::optional<Time> try_minutes(double value);

            static Time hours(double value);

            static std::optional<Time> try_hours(double value);

            double raw_si();

            double to_seconds();

            double to_milliseconds();

            double to_microseconds();

            double to_nanoseconds();

            double to_minutes();

            double to_hours();

            bool approximately_equals(Time other, double epsilon);

            QuantityDisplay<Time, TimeUnit> display_as(TimeUnit unit);

            QuantityDisplay<Time, TimeUnit> display_as_precision(TimeUnit unit, int precision);

            QuantityDisplay<Time, TimeUnit> display_seconds();
            QuantityDisplay<Time, TimeUnit> display_milliseconds();
            QuantityDisplay<Time, TimeUnit> display_microseconds();
            QuantityDisplay<Time, TimeUnit> display_nanoseconds();
            QuantityDisplay<Time, TimeUnit> display_minutes();
            QuantityDisplay<Time, TimeUnit> display_hours();

            QuantityDisplay<Time, TimeUnit> display_seconds_precision(int precision);
            QuantityDisplay<Time, TimeUnit> display_milliseconds_precision(int precision);
            QuantityDisplay<Time, TimeUnit> display_microseconds_precision(int precision);
            QuantityDisplay<Time, TimeUnit> display_nanoseconds_precision(int precision);
            QuantityDisplay<Time, TimeUnit> display_minutes_precision(int precision);
            QuantityDisplay<Time, TimeUnit> display_hours_precision(int precision);

            friend bool operator==(Time left, Time right);

        private:
            explicit Time(double seconds);
    };

    Time seconds(double value);
    std::optional<Time> try_seconds(double value);
    Time milliseconds(double value);
    std::optional<Time> try_milliseconds(double value);
    Time microseconds(double value);
    std::optional<Time> try_microseconds(double value);
    Time nanoseconds(double value);
    std::optional<Time> try_nanoseconds(double value);
    Time minutes(double value);
    std::optional<Time> try_minutes(double value);
    Time hours(double value);
    std::optional<Time> try_hours(double value);

} // namespace Physics::time
