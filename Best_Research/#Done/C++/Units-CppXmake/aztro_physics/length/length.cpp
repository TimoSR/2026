module;
#include <cmath>
#include <optional>
#include <ostream>

export module aztro_physics:length;
import :detail_math;
import :quantity_display;
import :time;
import :velocity;

export {
    namespace aztro::physics::length
    {

        enum class LengthUnit
        {
            Meters,
            Kilometers,
            Centimeters,
            Millimeters,
            Micrometers,
            Nanometers,
        };

        class Length
        {
                double _meters;

            public:
                static Length from_raw_si(double meters);

                static Length meters(double value);

                static std::optional<Length> try_meters(double value);

                static Length kilometers(double value);

                static std::optional<Length> try_kilometers(double value);

                static Length centimeters(double value);

                static std::optional<Length> try_centimeters(double value);

                static Length millimeters(double value);

                static std::optional<Length> try_millimeters(double value);

                static Length micrometers(double value);

                static std::optional<Length> try_micrometers(double value);

                static Length nanometers(double value);

                static std::optional<Length> try_nanometers(double value);

                double raw_si();

                double to_meters();

                double to_kilometers();

                double to_centimeters();

                double to_millimeters();

                double to_micrometers();

                double to_nanometers();

                bool approximately_equals(Length other, double epsilon);

                QuantityDisplay<Length, LengthUnit> display_as(LengthUnit unit);

                QuantityDisplay<Length, LengthUnit> display_as_precision(LengthUnit unit, int precision);

                QuantityDisplay<Length, LengthUnit> display_meters();
                QuantityDisplay<Length, LengthUnit> display_kilometers();
                QuantityDisplay<Length, LengthUnit> display_centimeters();
                QuantityDisplay<Length, LengthUnit> display_millimeters();
                QuantityDisplay<Length, LengthUnit> display_micrometers();
                QuantityDisplay<Length, LengthUnit> display_nanometers();

                QuantityDisplay<Length, LengthUnit> display_meters_precision(int precision);
                QuantityDisplay<Length, LengthUnit> display_kilometers_precision(int precision);
                QuantityDisplay<Length, LengthUnit> display_centimeters_precision(int precision);
                QuantityDisplay<Length, LengthUnit> display_millimeters_precision(int precision);
                QuantityDisplay<Length, LengthUnit> display_micrometers_precision(int precision);
                QuantityDisplay<Length, LengthUnit> display_nanometers_precision(int precision);

                std::optional<velocity::Velocity> checked_div_time(time::Time time);
                std::optional<time::Time> checked_div_velocity(velocity::Velocity velocity);

                friend bool operator==(Length left, Length right);

            private:
                explicit Length(double meters);
        };

        Length meters(double value);
        std::optional<Length> try_meters(double value);
        Length kilometers(double value);
        std::optional<Length> try_kilometers(double value);
        Length centimeters(double value);
        std::optional<Length> try_centimeters(double value);
        Length millimeters(double value);
        std::optional<Length> try_millimeters(double value);
        Length micrometers(double value);
        std::optional<Length> try_micrometers(double value);
        Length nanometers(double value);
        std::optional<Length> try_nanometers(double value);

    } // namespace aztro::physics::length

    namespace aztro::physics::length
    {

        Length::Length(double meters) : _meters(meters)
        {
        }

        Length Length::from_raw_si(double meters)
        {
            return Length(meters);
        }

        Length Length::meters(double value)
        {
            return Length(value);
        }

        Length Length::kilometers(double value)
        {
            return Length(value * 1'000.0);
        }

        Length Length::centimeters(double value)
        {
            return Length(value / 100.0);
        }

        Length Length::millimeters(double value)
        {
            return Length(value / 1'000.0);
        }

        Length Length::micrometers(double value)
        {
            return Length(value / 1'000'000.0);
        }

        Length Length::nanometers(double value)
        {
            return Length(value / 1'000'000'000.0);
        }

        double Length::raw_si()
        {
            return _meters;
        }

        double Length::to_meters()
        {
            return _meters;
        }

        double Length::to_kilometers()
        {
            return _meters / 1'000.0;
        }

        double Length::to_centimeters()
        {
            return _meters * 100.0;
        }

        double Length::to_millimeters()
        {
            return _meters * 1'000.0;
        }

        double Length::to_micrometers()
        {
            return _meters * 1'000'000.0;
        }

        double Length::to_nanometers()
        {
            return _meters * 1'000'000'000.0;
        }

        bool Length::approximately_equals(Length other, double epsilon)
        {
            return detail::absolute(_meters - other._meters) <= epsilon;
        }

        QuantityDisplay<Length, LengthUnit> Length::display_as(LengthUnit unit)
        {
            return QuantityDisplay<Length, LengthUnit>(*this, unit);
        }

        QuantityDisplay<Length, LengthUnit> Length::display_as_precision(LengthUnit unit, int precision)
        {
            return QuantityDisplay<Length, LengthUnit>(*this, unit, precision);
        }

        bool operator==(Length left, Length right)
        {
            return left._meters == right._meters;
        }

    } // namespace aztro::physics::length

    namespace aztro::physics::length
    {

        std::optional<Length> Length::try_meters(double value)
        {
            if (!std::isfinite(value))
            {
                return std::nullopt;
            }

            return Length::meters(value);
        }

        std::optional<Length> Length::try_kilometers(double value)
        {
            if (!std::isfinite(value))
            {
                return std::nullopt;
            }

            return Length::kilometers(value);
        }

        std::optional<Length> Length::try_centimeters(double value)
        {
            if (!std::isfinite(value))
            {
                return std::nullopt;
            }

            return Length::centimeters(value);
        }

        std::optional<Length> Length::try_millimeters(double value)
        {
            if (!std::isfinite(value))
            {
                return std::nullopt;
            }

            return Length::millimeters(value);
        }

        std::optional<Length> Length::try_micrometers(double value)
        {
            if (!std::isfinite(value))
            {
                return std::nullopt;
            }

            return Length::micrometers(value);
        }

        std::optional<Length> Length::try_nanometers(double value)
        {
            if (!std::isfinite(value))
            {
                return std::nullopt;
            }

            return Length::nanometers(value);
        }

        Length meters(double value)
        {
            return Length::meters(value);
        }

        std::optional<Length> try_meters(double value)
        {
            return Length::try_meters(value);
        }

        Length kilometers(double value)
        {
            return Length::kilometers(value);
        }

        std::optional<Length> try_kilometers(double value)
        {
            return Length::try_kilometers(value);
        }

        Length centimeters(double value)
        {
            return Length::centimeters(value);
        }

        std::optional<Length> try_centimeters(double value)
        {
            return Length::try_centimeters(value);
        }

        Length millimeters(double value)
        {
            return Length::millimeters(value);
        }

        std::optional<Length> try_millimeters(double value)
        {
            return Length::try_millimeters(value);
        }

        Length micrometers(double value)
        {
            return Length::micrometers(value);
        }

        std::optional<Length> try_micrometers(double value)
        {
            return Length::try_micrometers(value);
        }

        Length nanometers(double value)
        {
            return Length::nanometers(value);
        }

        std::optional<Length> try_nanometers(double value)
        {
            return Length::try_nanometers(value);
        }

    } // namespace aztro::physics::length

    namespace aztro::physics::length
    {

        Length operator+(Length left, Length right)
        {
            return Length::from_raw_si(left.raw_si() + right.raw_si());
        }

        Length operator-(Length left, Length right)
        {
            return Length::from_raw_si(left.raw_si() - right.raw_si());
        }

        Length operator-(Length value)
        {
            return Length::from_raw_si(-value.raw_si());
        }

        Length operator*(Length value, double scalar)
        {
            return Length::from_raw_si(value.raw_si() * scalar);
        }

        Length operator*(double scalar, Length value)
        {
            return Length::from_raw_si(scalar * value.raw_si());
        }

        Length operator/(Length value, double scalar)
        {
            return Length::from_raw_si(value.raw_si() / scalar);
        }

        double operator/(Length left, Length right)
        {
            return left.raw_si() / right.raw_si();
        }

    } // namespace aztro::physics::length

    namespace aztro::physics::length
    {

        const char* symbol(LengthUnit unit)
        {
            switch (unit)
            {
            case LengthUnit::Meters:
                return "m";
            case LengthUnit::Kilometers:
                return "km";
            case LengthUnit::Centimeters:
                return "cm";
            case LengthUnit::Millimeters:
                return "mm";
            case LengthUnit::Micrometers:
                return "um";
            case LengthUnit::Nanometers:
                return "nm";
            }

            return "";
        }

        double value_in_unit(Length value, LengthUnit unit)
        {
            switch (unit)
            {
            case LengthUnit::Meters:
                return value.to_meters();
            case LengthUnit::Kilometers:
                return value.to_kilometers();
            case LengthUnit::Centimeters:
                return value.to_centimeters();
            case LengthUnit::Millimeters:
                return value.to_millimeters();
            case LengthUnit::Micrometers:
                return value.to_micrometers();
            case LengthUnit::Nanometers:
                return value.to_nanometers();
            }

            return value.to_meters();
        }

        QuantityDisplay<Length, LengthUnit> Length::display_meters()
        {
            return display_as(LengthUnit::Meters);
        }

        QuantityDisplay<Length, LengthUnit> Length::display_kilometers()
        {
            return display_as(LengthUnit::Kilometers);
        }

        QuantityDisplay<Length, LengthUnit> Length::display_centimeters()
        {
            return display_as(LengthUnit::Centimeters);
        }

        QuantityDisplay<Length, LengthUnit> Length::display_millimeters()
        {
            return display_as(LengthUnit::Millimeters);
        }

        QuantityDisplay<Length, LengthUnit> Length::display_micrometers()
        {
            return display_as(LengthUnit::Micrometers);
        }

        QuantityDisplay<Length, LengthUnit> Length::display_nanometers()
        {
            return display_as(LengthUnit::Nanometers);
        }

        QuantityDisplay<Length, LengthUnit> Length::display_meters_precision(int precision)
        {
            return display_as_precision(LengthUnit::Meters, precision);
        }

        QuantityDisplay<Length, LengthUnit> Length::display_kilometers_precision(int precision)
        {
            return display_as_precision(LengthUnit::Kilometers, precision);
        }

        QuantityDisplay<Length, LengthUnit> Length::display_centimeters_precision(int precision)
        {
            return display_as_precision(LengthUnit::Centimeters, precision);
        }

        QuantityDisplay<Length, LengthUnit> Length::display_millimeters_precision(int precision)
        {
            return display_as_precision(LengthUnit::Millimeters, precision);
        }

        QuantityDisplay<Length, LengthUnit> Length::display_micrometers_precision(int precision)
        {
            return display_as_precision(LengthUnit::Micrometers, precision);
        }

        QuantityDisplay<Length, LengthUnit> Length::display_nanometers_precision(int precision)
        {
            return display_as_precision(LengthUnit::Nanometers, precision);
        }

        std::ostream& operator<<(std::ostream& stream, Length value)
        {
            return stream << value.to_meters() << " m";
        }

    } // namespace aztro::physics::length
}
