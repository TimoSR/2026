#include "units/display/display.hpp"

#include <ostream>

namespace units::time
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

    double valueInUnit(Time value, TimeUnit unit)
    {
        switch (unit)
        {
        case TimeUnit::Seconds:
            return value.asSeconds();
        case TimeUnit::Milliseconds:
            return value.asMilliseconds();
        case TimeUnit::Microseconds:
            return value.asMicroseconds();
        case TimeUnit::Nanoseconds:
            return value.asNanoseconds();
        case TimeUnit::Minutes:
            return value.asMinutes();
        case TimeUnit::Hours:
            return value.asHours();
        }

        return value.asSeconds();
    }

    std::ostream& operator<<(std::ostream& stream, Time value)
    {
        return stream << value.displayAs(TimeUnit::Seconds);
    }

} // namespace units::time
