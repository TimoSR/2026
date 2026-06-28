#include "physics/display/display.hpp"

#include <ostream>

namespace physics::time
{

    const char* symbol(TimeUnit unit)
    {
        switch (unit)
        {
        case TimeUnit::Seconds:
            return "s";
        case TimeUnit::Milliseconds:
            return "ms";
        case TimeUnit::Microseconds:
            return "us";
        case TimeUnit::Nanoseconds:
            return "ns";
        case TimeUnit::Minutes:
            return "min";
        case TimeUnit::Hours:
            return "h";
        }

        return "";
    }

    double value_in_unit(Time value, TimeUnit unit)
    {
        switch (unit)
        {
        case TimeUnit::Seconds:
            return value.to_seconds();
        case TimeUnit::Milliseconds:
            return value.to_milliseconds();
        case TimeUnit::Microseconds:
            return value.to_microseconds();
        case TimeUnit::Nanoseconds:
            return value.to_nanoseconds();
        case TimeUnit::Minutes:
            return value.to_minutes();
        case TimeUnit::Hours:
            return value.to_hours();
        }

        return value.to_seconds();
    }

    QuantityDisplay<Time, TimeUnit> Time::display_seconds()
    {
        return display_as(TimeUnit::Seconds);
    }

    QuantityDisplay<Time, TimeUnit> Time::display_milliseconds()
    {
        return display_as(TimeUnit::Milliseconds);
    }

    QuantityDisplay<Time, TimeUnit> Time::display_microseconds()
    {
        return display_as(TimeUnit::Microseconds);
    }

    QuantityDisplay<Time, TimeUnit> Time::display_nanoseconds()
    {
        return display_as(TimeUnit::Nanoseconds);
    }

    QuantityDisplay<Time, TimeUnit> Time::display_minutes()
    {
        return display_as(TimeUnit::Minutes);
    }

    QuantityDisplay<Time, TimeUnit> Time::display_hours()
    {
        return display_as(TimeUnit::Hours);
    }

    QuantityDisplay<Time, TimeUnit> Time::display_seconds_precision(int precision)
    {
        return display_as_precision(TimeUnit::Seconds, precision);
    }

    QuantityDisplay<Time, TimeUnit> Time::display_milliseconds_precision(int precision)
    {
        return display_as_precision(TimeUnit::Milliseconds, precision);
    }

    QuantityDisplay<Time, TimeUnit> Time::display_microseconds_precision(int precision)
    {
        return display_as_precision(TimeUnit::Microseconds, precision);
    }

    QuantityDisplay<Time, TimeUnit> Time::display_nanoseconds_precision(int precision)
    {
        return display_as_precision(TimeUnit::Nanoseconds, precision);
    }

    QuantityDisplay<Time, TimeUnit> Time::display_minutes_precision(int precision)
    {
        return display_as_precision(TimeUnit::Minutes, precision);
    }

    QuantityDisplay<Time, TimeUnit> Time::display_hours_precision(int precision)
    {
        return display_as_precision(TimeUnit::Hours, precision);
    }

    std::ostream& operator<<(std::ostream& stream, Time value)
    {
        return stream << value.display_as(TimeUnit::Seconds);
    }

} // namespace physics::time
