module;
#include <cmath>
#include <optional>
#include <ostream>

export module aztro_physics:velocity;
import :detail_math;
import :quantity_display;
import :time;
import :acceleration;

export {
    namespace aztro::physics::velocity
    {

        enum class VelocityUnit
        {
            MetersPerSecond,
            KilometersPerHour,
        };

        class Velocity
        {
            private:
                double _meters_per_second;

            private:
                explicit Velocity(double meters_per_second)
                {
                    _meters_per_second = meters_per_second;
                }

            public:
                static Velocity from_raw_si(double meters_per_second);

                static Velocity meters_per_second(double value);

                static std::optional<Velocity> try_meters_per_second(double value);

                static Velocity kilometers_per_hour(double value);

                static std::optional<Velocity> try_kilometers_per_hour(double value);

                double raw_si();

                double to_meters_per_second();

                double to_kilometers_per_hour();

                bool approximately_equals(Velocity other, double epsilon);

                QuantityDisplay<Velocity, VelocityUnit> display_as(VelocityUnit unit);

                QuantityDisplay<Velocity, VelocityUnit> display_as_precision(VelocityUnit unit, int precision);

                QuantityDisplay<Velocity, VelocityUnit> display_meters_per_second();
                QuantityDisplay<Velocity, VelocityUnit> display_kilometers_per_hour();

                QuantityDisplay<Velocity, VelocityUnit> display_meters_per_second_precision(int precision);
                QuantityDisplay<Velocity, VelocityUnit> display_kilometers_per_hour_precision(int precision);

                std::optional<acceleration::Acceleration> checked_div_time(time::Time time);
                std::optional<time::Time> checked_div_acceleration(acceleration::Acceleration acceleration);

                friend bool operator==(Velocity left, Velocity right);
        };

        Velocity meters_per_second(double value);
        std::optional<Velocity> try_meters_per_second(double value);
        Velocity meters_pr_second(double value);
        Velocity kilometers_per_hour(double value);
        std::optional<Velocity> try_kilometers_per_hour(double value);

    } // namespace aztro::physics::velocity

    namespace aztro::physics::velocity
    {

        Velocity Velocity::from_raw_si(double meters_per_second)
        {
            return Velocity(meters_per_second);
        }

        Velocity Velocity::meters_per_second(double value)
        {
            return Velocity(value);
        }

        Velocity Velocity::kilometers_per_hour(double value)
        {
            return Velocity(value / 3.6);
        }

        double Velocity::raw_si()
        {
            return _meters_per_second;
        }

        double Velocity::to_meters_per_second()
        {
            return _meters_per_second;
        }

        double Velocity::to_kilometers_per_hour()
        {
            return _meters_per_second * 3.6;
        }

        bool Velocity::approximately_equals(Velocity other, double epsilon)
        {
            return detail::absolute(_meters_per_second - other._meters_per_second) <= epsilon;
        }

        QuantityDisplay<Velocity, VelocityUnit> Velocity::display_as(VelocityUnit unit)
        {
            return QuantityDisplay<Velocity, VelocityUnit>(*this, unit);
        }

        QuantityDisplay<Velocity, VelocityUnit> Velocity::display_as_precision(VelocityUnit unit, int precision)
        {
            return QuantityDisplay<Velocity, VelocityUnit>(*this, unit, precision);
        }

        bool operator==(Velocity left, Velocity right)
        {
            return left._meters_per_second == right._meters_per_second;
        }

    } // namespace aztro::physics::velocity

    namespace aztro::physics::velocity
    {

        std::optional<Velocity> Velocity::try_meters_per_second(double value)
        {
            if (!std::isfinite(value))
            {
                return std::nullopt;
            }

            return Velocity::meters_per_second(value);
        }

        std::optional<Velocity> Velocity::try_kilometers_per_hour(double value)
        {
            if (!std::isfinite(value))
            {
                return std::nullopt;
            }

            return Velocity::kilometers_per_hour(value);
        }

        Velocity meters_per_second(double value)
        {
            return Velocity::meters_per_second(value);
        }

        std::optional<Velocity> try_meters_per_second(double value)
        {
            return Velocity::try_meters_per_second(value);
        }

        Velocity meters_pr_second(double value)
        {
            return Velocity::meters_per_second(value);
        }

        Velocity kilometers_per_hour(double value)
        {
            return Velocity::kilometers_per_hour(value);
        }

        std::optional<Velocity> try_kilometers_per_hour(double value)
        {
            return Velocity::try_kilometers_per_hour(value);
        }

    } // namespace aztro::physics::velocity

    namespace aztro::physics::velocity
    {

        Velocity operator+(Velocity left, Velocity right)
        {
            return Velocity::from_raw_si(left.raw_si() + right.raw_si());
        }

        Velocity operator-(Velocity left, Velocity right)
        {
            return Velocity::from_raw_si(left.raw_si() - right.raw_si());
        }

        Velocity operator-(Velocity value)
        {
            return Velocity::from_raw_si(-value.raw_si());
        }

        Velocity operator*(Velocity value, double scalar)
        {
            return Velocity::from_raw_si(value.raw_si() * scalar);
        }

        Velocity operator*(double scalar, Velocity value)
        {
            return Velocity::from_raw_si(scalar * value.raw_si());
        }

        Velocity operator/(Velocity value, double scalar)
        {
            return Velocity::from_raw_si(value.raw_si() / scalar);
        }

        double operator/(Velocity left, Velocity right)
        {
            return left.raw_si() / right.raw_si();
        }

    } // namespace aztro::physics::velocity

    namespace aztro::physics::velocity
    {

        const char* symbol(VelocityUnit unit)
        {
            switch (unit)
            {
            case VelocityUnit::MetersPerSecond:
                return "m/s";
            case VelocityUnit::KilometersPerHour:
                return "km/h";
            }

            return "";
        }

        double value_in_unit(Velocity value, VelocityUnit unit)
        {
            switch (unit)
            {
            case VelocityUnit::MetersPerSecond:
                return value.to_meters_per_second();
            case VelocityUnit::KilometersPerHour:
                return value.to_kilometers_per_hour();
            }

            return value.to_meters_per_second();
        }

        QuantityDisplay<Velocity, VelocityUnit> Velocity::display_meters_per_second()
        {
            return display_as(VelocityUnit::MetersPerSecond);
        }

        QuantityDisplay<Velocity, VelocityUnit> Velocity::display_kilometers_per_hour()
        {
            return display_as(VelocityUnit::KilometersPerHour);
        }

        QuantityDisplay<Velocity, VelocityUnit> Velocity::display_meters_per_second_precision(int precision)
        {
            return display_as_precision(VelocityUnit::MetersPerSecond, precision);
        }

        QuantityDisplay<Velocity, VelocityUnit> Velocity::display_kilometers_per_hour_precision(int precision)
        {
            return display_as_precision(VelocityUnit::KilometersPerHour, precision);
        }

        std::ostream& operator<<(std::ostream& stream, Velocity value)
        {
            return stream << value.to_meters_per_second() << " m/s";
        }

    } // namespace aztro::physics::velocity
}
