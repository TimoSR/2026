#pragma once

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
            static Time fromRawSi(double seconds);

            static Time seconds(double value);

            static Time milliseconds(double value);

            static Time microseconds(double value);

            static Time nanoseconds(double value);

            static Time minutes(double value);

            static Time hours(double value);

            double rawSi();

            double asSeconds();

            double asMilliseconds();

            double asMicroseconds();

            double asNanoseconds();

            double asMinutes();

            double asHours();

            bool approximatelyEquals(Time other, double epsilon);

            QuantityDisplay<Time, TimeUnit> displayAs(TimeUnit unit);

            QuantityDisplay<Time, TimeUnit> displayAsPrecision(TimeUnit unit, int precision);

            friend bool operator==(Time left, Time right);

        private:
            explicit Time(double seconds);
    };

    Time seconds(double value);
    Time milliseconds(double value);
    Time microseconds(double value);
    Time nanoseconds(double value);
    Time minutes(double value);
    Time hours(double value);

} // namespace Physics::time
