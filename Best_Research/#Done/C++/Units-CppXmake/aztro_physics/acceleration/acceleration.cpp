module;
#include <cmath>
#include <optional>
#include <ostream>

export module aztro_physics:acceleration;
import :detail_math;
import :quantity_display;

export {
    namespace aztro::physics::acceleration
    {

        enum class AccelerationUnit
        {
            MetersPerSecondSquared,
            StandardGravity,
        };

        class Acceleration
        {

                double _meters_per_second_squared;

            public:
                static double standard_gravity_meters_per_second_squared();

                static Acceleration from_raw_si(double meters_per_second_squared);

                static Acceleration meters_per_second_squared(double value);

                static std::optional<Acceleration> try_meters_per_second_squared(double value);

                static Acceleration standard_gravity(double value);

                static std::optional<Acceleration> try_standard_gravity(double value);

                double raw_si();

                double to_meters_per_second_squared();

                double to_standard_gravity();

                bool approximately_equals(Acceleration other, double epsilon);

                QuantityDisplay<Acceleration, AccelerationUnit> display_as(AccelerationUnit unit);

                QuantityDisplay<Acceleration, AccelerationUnit> display_as_precision(AccelerationUnit unit, int precision);

                QuantityDisplay<Acceleration, AccelerationUnit> display_meters_per_second_squared();
                QuantityDisplay<Acceleration, AccelerationUnit> display_standard_gravity();

                QuantityDisplay<Acceleration, AccelerationUnit> display_meters_per_second_squared_precision(int precision);
                QuantityDisplay<Acceleration, AccelerationUnit> display_standard_gravity_precision(int precision);

                friend bool operator==(Acceleration left, Acceleration right);

            private:
                explicit Acceleration(double meters_per_second_squared);
        };

        Acceleration meters_per_second_squared(double value);
        std::optional<Acceleration> try_meters_per_second_squared(double value);
        Acceleration meters_pr_second_pr_second(double value);
        Acceleration standard_gravity(double value);
        std::optional<Acceleration> try_standard_gravity(double value);

    } // namespace aztro::physics::acceleration

    namespace aztro::physics::acceleration
    {

        double Acceleration::standard_gravity_meters_per_second_squared()
        {
            return 9.80665;
        }

        Acceleration::Acceleration(double meters_per_second_squared) : _meters_per_second_squared(meters_per_second_squared)
        {
        }

        Acceleration Acceleration::from_raw_si(double meters_per_second_squared)
        {
            return Acceleration(meters_per_second_squared);
        }

        Acceleration Acceleration::meters_per_second_squared(double value)
        {
            return Acceleration(value);
        }

        Acceleration Acceleration::standard_gravity(double value)
        {
            return Acceleration(value * standard_gravity_meters_per_second_squared());
        }

        double Acceleration::raw_si()
        {
            return _meters_per_second_squared;
        }

        double Acceleration::to_meters_per_second_squared()
        {
            return _meters_per_second_squared;
        }

        double Acceleration::to_standard_gravity()
        {
            return _meters_per_second_squared / standard_gravity_meters_per_second_squared();
        }

        bool Acceleration::approximately_equals(Acceleration other, double epsilon)
        {
            return detail::absolute(_meters_per_second_squared - other._meters_per_second_squared) <= epsilon;
        }

        QuantityDisplay<Acceleration, AccelerationUnit> Acceleration::display_as(AccelerationUnit unit)
        {
            return QuantityDisplay<Acceleration, AccelerationUnit>(*this, unit);
        }

        QuantityDisplay<Acceleration, AccelerationUnit> Acceleration::display_as_precision(AccelerationUnit unit, int precision)
        {
            return QuantityDisplay<Acceleration, AccelerationUnit>(*this, unit, precision);
        }

        bool operator==(Acceleration left, Acceleration right)
        {
            return left._meters_per_second_squared == right._meters_per_second_squared;
        }

    } // namespace aztro::physics::acceleration

    namespace aztro::physics::acceleration
    {

        std::optional<Acceleration> Acceleration::try_meters_per_second_squared(double value)
        {
            if (!std::isfinite(value))
            {
                return std::nullopt;
            }

            return Acceleration::meters_per_second_squared(value);
        }

        std::optional<Acceleration> Acceleration::try_standard_gravity(double value)
        {
            if (!std::isfinite(value))
            {
                return std::nullopt;
            }

            return Acceleration::standard_gravity(value);
        }

        Acceleration meters_per_second_squared(double value)
        {
            return Acceleration::meters_per_second_squared(value);
        }

        std::optional<Acceleration> try_meters_per_second_squared(double value)
        {
            return Acceleration::try_meters_per_second_squared(value);
        }

        Acceleration meters_pr_second_pr_second(double value)
        {
            return Acceleration::meters_per_second_squared(value);
        }

        Acceleration standard_gravity(double value)
        {
            return Acceleration::standard_gravity(value);
        }

        std::optional<Acceleration> try_standard_gravity(double value)
        {
            return Acceleration::try_standard_gravity(value);
        }

    } // namespace aztro::physics::acceleration

    namespace aztro::physics::acceleration
    {

        Acceleration operator+(Acceleration left, Acceleration right)
        {
            return Acceleration::from_raw_si(left.raw_si() + right.raw_si());
        }

        Acceleration operator-(Acceleration left, Acceleration right)
        {
            return Acceleration::from_raw_si(left.raw_si() - right.raw_si());
        }

        Acceleration operator-(Acceleration value)
        {
            return Acceleration::from_raw_si(-value.raw_si());
        }

        Acceleration operator*(Acceleration value, double scalar)
        {
            return Acceleration::from_raw_si(value.raw_si() * scalar);
        }

        Acceleration operator*(double scalar, Acceleration value)
        {
            return Acceleration::from_raw_si(scalar * value.raw_si());
        }

        Acceleration operator/(Acceleration value, double scalar)
        {
            return Acceleration::from_raw_si(value.raw_si() / scalar);
        }

        double operator/(Acceleration left, Acceleration right)
        {
            return left.raw_si() / right.raw_si();
        }

    } // namespace aztro::physics::acceleration

    namespace aztro::physics::acceleration
    {

        const char* symbol(AccelerationUnit unit)
        {
            switch (unit)
            {
            case AccelerationUnit::MetersPerSecondSquared:
                return "m/s^2";
            case AccelerationUnit::StandardGravity:
                return "g0";
            }

            return "";
        }

        double value_in_unit(Acceleration value, AccelerationUnit unit)
        {
            switch (unit)
            {
            case AccelerationUnit::MetersPerSecondSquared:
                return value.to_meters_per_second_squared();
            case AccelerationUnit::StandardGravity:
                return value.to_standard_gravity();
            }

            return value.to_meters_per_second_squared();
        }

        QuantityDisplay<Acceleration, AccelerationUnit> Acceleration::display_meters_per_second_squared()
        {
            return display_as(AccelerationUnit::MetersPerSecondSquared);
        }

        QuantityDisplay<Acceleration, AccelerationUnit> Acceleration::display_standard_gravity()
        {
            return display_as(AccelerationUnit::StandardGravity);
        }

        QuantityDisplay<Acceleration, AccelerationUnit> Acceleration::display_meters_per_second_squared_precision(int precision)
        {
            return display_as_precision(AccelerationUnit::MetersPerSecondSquared, precision);
        }

        QuantityDisplay<Acceleration, AccelerationUnit> Acceleration::display_standard_gravity_precision(int precision)
        {
            return display_as_precision(AccelerationUnit::StandardGravity, precision);
        }

        std::ostream& operator<<(std::ostream& stream, Acceleration value)
        {
            return stream << value.to_meters_per_second_squared() << " m/s^2";
        }

    } // namespace aztro::physics::acceleration
}
