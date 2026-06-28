module;

#include <format>
#include <iomanip>
#include <limits>
#include <optional>
#include <ostream>

module aztro.physics;

namespace aztro::physics::time
{

    Time::Time(double seconds) : _seconds(seconds)
    {
    }

    Time Time::from_raw_si(double seconds)
    {
        return Time(seconds);
    }

    Time Time::seconds(double value)
    {
        return Time(value);
    }

    Time Time::milliseconds(double value)
    {
        return Time(value / 1'000.0);
    }

    Time Time::microseconds(double value)
    {
        return Time(value / 1'000'000.0);
    }

    Time Time::nanoseconds(double value)
    {
        return Time(value / 1'000'000'000.0);
    }

    Time Time::minutes(double value)
    {
        return Time(value * 60.0);
    }

    Time Time::hours(double value)
    {
        return Time(value * 3'600.0);
    }

    double Time::raw_si()
    {
        return _seconds;
    }

    double Time::to_seconds()
    {
        return _seconds;
    }

    double Time::to_milliseconds()
    {
        return _seconds * 1'000.0;
    }

    double Time::to_microseconds()
    {
        return _seconds * 1'000'000.0;
    }

    double Time::to_nanoseconds()
    {
        return _seconds * 1'000'000'000.0;
    }

    double Time::to_minutes()
    {
        return _seconds / 60.0;
    }

    double Time::to_hours()
    {
        return _seconds / 3'600.0;
    }

    bool Time::approximately_equals(Time other, double epsilon)
    {
        return detail::absolute(_seconds - other._seconds) <= epsilon;
    }

    QuantityDisplay<Time, TimeUnit> Time::display_as(TimeUnit unit)
    {
        return QuantityDisplay<Time, TimeUnit>(*this, unit);
    }

    QuantityDisplay<Time, TimeUnit> Time::display_as_precision(TimeUnit unit, int precision)
    {
        return QuantityDisplay<Time, TimeUnit>(*this, unit, precision);
    }

    bool operator==(Time left, Time right)
    {
        return left._seconds == right._seconds;
    }

} // namespace aztro::physics::time
