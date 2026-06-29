module;
#include <cmath>
#include <optional>
#include <ostream>

export module aztro_physics:time;
import :detail_math;
import :quantity_display;

export {
    namespace aztro::physics::time
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

    } // namespace aztro::physics::time

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

    namespace aztro::physics::time
    {

        std::optional<Time> Time::try_seconds(double value)
        {
            if (!std::isfinite(value))
            {
                return std::nullopt;
            }

            return Time::seconds(value);
        }

        std::optional<Time> Time::try_milliseconds(double value)
        {
            if (!std::isfinite(value))
            {
                return std::nullopt;
            }

            return Time::milliseconds(value);
        }

        std::optional<Time> Time::try_microseconds(double value)
        {
            if (!std::isfinite(value))
            {
                return std::nullopt;
            }

            return Time::microseconds(value);
        }

        std::optional<Time> Time::try_nanoseconds(double value)
        {
            if (!std::isfinite(value))
            {
                return std::nullopt;
            }

            return Time::nanoseconds(value);
        }

        std::optional<Time> Time::try_minutes(double value)
        {
            if (!std::isfinite(value))
            {
                return std::nullopt;
            }

            return Time::minutes(value);
        }

        std::optional<Time> Time::try_hours(double value)
        {
            if (!std::isfinite(value))
            {
                return std::nullopt;
            }

            return Time::hours(value);
        }

        Time seconds(double value)
        {
            return Time::seconds(value);
        }

        std::optional<Time> try_seconds(double value)
        {
            return Time::try_seconds(value);
        }

        Time milliseconds(double value)
        {
            return Time::milliseconds(value);
        }

        std::optional<Time> try_milliseconds(double value)
        {
            return Time::try_milliseconds(value);
        }

        Time microseconds(double value)
        {
            return Time::microseconds(value);
        }

        std::optional<Time> try_microseconds(double value)
        {
            return Time::try_microseconds(value);
        }

        Time nanoseconds(double value)
        {
            return Time::nanoseconds(value);
        }

        std::optional<Time> try_nanoseconds(double value)
        {
            return Time::try_nanoseconds(value);
        }

        Time minutes(double value)
        {
            return Time::minutes(value);
        }

        std::optional<Time> try_minutes(double value)
        {
            return Time::try_minutes(value);
        }

        Time hours(double value)
        {
            return Time::hours(value);
        }

        std::optional<Time> try_hours(double value)
        {
            return Time::try_hours(value);
        }

    } // namespace aztro::physics::time

    namespace aztro::physics::time
    {

        Time operator+(Time left, Time right)
        {
            return Time::from_raw_si(left.raw_si() + right.raw_si());
        }

        Time operator-(Time left, Time right)
        {
            return Time::from_raw_si(left.raw_si() - right.raw_si());
        }

        Time operator-(Time value)
        {
            return Time::from_raw_si(-value.raw_si());
        }

        Time operator*(Time value, double scalar)
        {
            return Time::from_raw_si(value.raw_si() * scalar);
        }

        Time operator*(double scalar, Time value)
        {
            return Time::from_raw_si(scalar * value.raw_si());
        }

        Time operator/(Time value, double scalar)
        {
            return Time::from_raw_si(value.raw_si() / scalar);
        }

        double operator/(Time left, Time right)
        {
            return left.raw_si() / right.raw_si();
        }

    } // namespace aztro::physics::time

    namespace aztro::physics::time
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
            return stream << value.to_seconds() << " s";
        }

    } // namespace aztro::physics::time
}
