#include "Physics/time/time.hpp"

#include "Physics/detail/math.hpp"

namespace Physics::time
{

    Time::Time(double seconds) : _seconds(seconds)
    {
    }

    Time Time::fromRawSi(double seconds)
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

    double Time::rawSi()
    {
        return _seconds;
    }

    double Time::asSeconds()
    {
        return _seconds;
    }

    double Time::asMilliseconds()
    {
        return _seconds * 1'000.0;
    }

    double Time::asMicroseconds()
    {
        return _seconds * 1'000'000.0;
    }

    double Time::asNanoseconds()
    {
        return _seconds * 1'000'000'000.0;
    }

    double Time::asMinutes()
    {
        return _seconds / 60.0;
    }

    double Time::asHours()
    {
        return _seconds / 3'600.0;
    }

    bool Time::approximatelyEquals(Time other, double epsilon)
    {
        return detail::absolute(_seconds - other._seconds) <= epsilon;
    }

    QuantityDisplay<Time, TimeUnit> Time::displayAs(TimeUnit unit)
    {
        return QuantityDisplay<Time, TimeUnit>(*this, unit);
    }

    QuantityDisplay<Time, TimeUnit> Time::displayAsPrecision(TimeUnit unit, int precision)
    {
        return QuantityDisplay<Time, TimeUnit>(*this, unit, precision);
    }

    bool operator==(Time left, Time right)
    {
        return left._seconds == right._seconds;
    }

} // namespace Physics::time
