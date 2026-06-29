module;
#include <cmath>
#include <optional>
#include <ostream>

export module aztro_physics:mass;
import :detail_math;
import :quantity_display;

namespace aztro::physics::mass
{

    export enum class MassUnit
    {
        Kilograms,
        Grams,
        Milligrams,
        Micrograms,
        Tons,
    };

    export class Mass
    {
        private:
            double _kilograms;

        private:
            explicit Mass(double kilograms)
            {
                _kilograms = kilograms;
            }

        public:
            static Mass from_raw_si(double kilograms);

            static Mass kilograms(double value);

            static std::optional<Mass> try_kilograms(double value);

            static Mass kilogram(double value);

            static std::optional<Mass> try_kilogram(double value);

            static Mass grams(double value);

            static std::optional<Mass> try_grams(double value);

            static Mass milligrams(double value);

            static std::optional<Mass> try_milligrams(double value);

            static Mass micrograms(double value);

            static std::optional<Mass> try_micrograms(double value);

            static Mass tons(double value);

            static std::optional<Mass> try_tons(double value);

            double raw_si();

            double to_kilograms();

            double to_grams();

            double to_milligrams();

            double to_micrograms();

            double to_tons();

            bool approximately_equals(Mass other, double epsilon);

            QuantityDisplay<Mass, MassUnit> display_as(MassUnit unit);

            QuantityDisplay<Mass, MassUnit> display_as_precision(MassUnit unit, int precision);

            QuantityDisplay<Mass, MassUnit> display_kilograms();
            QuantityDisplay<Mass, MassUnit> display_grams();
            QuantityDisplay<Mass, MassUnit> display_milligrams();
            QuantityDisplay<Mass, MassUnit> display_micrograms();
            QuantityDisplay<Mass, MassUnit> display_tons();

            QuantityDisplay<Mass, MassUnit> display_kilograms_precision(int precision);
            QuantityDisplay<Mass, MassUnit> display_grams_precision(int precision);
            QuantityDisplay<Mass, MassUnit> display_milligrams_precision(int precision);
            QuantityDisplay<Mass, MassUnit> display_micrograms_precision(int precision);
            QuantityDisplay<Mass, MassUnit> display_tons_precision(int precision);

            friend bool operator==(Mass left, Mass right);
    };

    export Mass kilograms(double value);
    export std::optional<Mass> try_kilograms(double value);
    export Mass kilogram(double value);
    export std::optional<Mass> try_kilogram(double value);
    export Mass grams(double value);
    export std::optional<Mass> try_grams(double value);
    export Mass milligrams(double value);
    export std::optional<Mass> try_milligrams(double value);
    export Mass micrograms(double value);
    export std::optional<Mass> try_micrograms(double value);
    export Mass tons(double value);
    export std::optional<Mass> try_tons(double value);

} // namespace aztro::physics::mass

namespace aztro::physics::mass
{

    Mass Mass::from_raw_si(double kilograms)
    {
        return Mass(kilograms);
    }

    Mass Mass::kilograms(double value)
    {
        return Mass(value);
    }

    Mass Mass::kilogram(double value)
    {
        return kilograms(value);
    }

    Mass Mass::grams(double value)
    {
        return Mass(value / 1'000.0);
    }

    Mass Mass::milligrams(double value)
    {
        return Mass(value / 1'000'000.0);
    }

    Mass Mass::micrograms(double value)
    {
        return Mass(value / 1'000'000'000.0);
    }

    Mass Mass::tons(double value)
    {
        return Mass(value * 1'000.0);
    }

    double Mass::raw_si()
    {
        return _kilograms;
    }

    double Mass::to_kilograms()
    {
        return _kilograms;
    }

    double Mass::to_grams()
    {
        return _kilograms * 1'000.0;
    }

    double Mass::to_milligrams()
    {
        return _kilograms * 1'000'000.0;
    }

    double Mass::to_micrograms()
    {
        return _kilograms * 1'000'000'000.0;
    }

    double Mass::to_tons()
    {
        return _kilograms / 1'000.0;
    }

    bool Mass::approximately_equals(Mass other, double epsilon)
    {
        return detail::absolute(_kilograms - other._kilograms) <= epsilon;
    }

    QuantityDisplay<Mass, MassUnit> Mass::display_as(MassUnit unit)
    {
        return QuantityDisplay<Mass, MassUnit>(*this, unit);
    }

    QuantityDisplay<Mass, MassUnit> Mass::display_as_precision(MassUnit unit, int precision)
    {
        return QuantityDisplay<Mass, MassUnit>(*this, unit, precision);
    }

    export bool operator==(Mass left, Mass right)
    {
        return left._kilograms == right._kilograms;
    }

} // namespace aztro::physics::mass

namespace aztro::physics::mass
{

    std::optional<Mass> Mass::try_kilograms(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Mass::kilograms(value);
    }

    std::optional<Mass> Mass::try_kilogram(double value)
    {
        return Mass::try_kilograms(value);
    }

    std::optional<Mass> Mass::try_grams(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Mass::grams(value);
    }

    std::optional<Mass> Mass::try_milligrams(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Mass::milligrams(value);
    }

    std::optional<Mass> Mass::try_micrograms(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Mass::micrograms(value);
    }

    std::optional<Mass> Mass::try_tons(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Mass::tons(value);
    }

    export Mass kilograms(double value)
    {
        return Mass::kilograms(value);
    }

    std::optional<Mass> try_kilograms(double value)
    {
        return Mass::try_kilograms(value);
    }

    export Mass kilogram(double value)
    {
        return Mass::kilogram(value);
    }

    std::optional<Mass> try_kilogram(double value)
    {
        return Mass::try_kilogram(value);
    }

    export Mass grams(double value)
    {
        return Mass::grams(value);
    }

    std::optional<Mass> try_grams(double value)
    {
        return Mass::try_grams(value);
    }

    export Mass milligrams(double value)
    {
        return Mass::milligrams(value);
    }

    std::optional<Mass> try_milligrams(double value)
    {
        return Mass::try_milligrams(value);
    }

    export Mass micrograms(double value)
    {
        return Mass::micrograms(value);
    }

    std::optional<Mass> try_micrograms(double value)
    {
        return Mass::try_micrograms(value);
    }

    export Mass tons(double value)
    {
        return Mass::tons(value);
    }

    std::optional<Mass> try_tons(double value)
    {
        return Mass::try_tons(value);
    }

} // namespace aztro::physics::mass

namespace aztro::physics::mass
{

    export Mass operator+(Mass left, Mass right)
    {
        return Mass::from_raw_si(left.raw_si() + right.raw_si());
    }

    export Mass operator-(Mass left, Mass right)
    {
        return Mass::from_raw_si(left.raw_si() - right.raw_si());
    }

    export Mass operator-(Mass value)
    {
        return Mass::from_raw_si(-value.raw_si());
    }

    export Mass operator*(Mass value, double scalar)
    {
        return Mass::from_raw_si(value.raw_si() * scalar);
    }

    export Mass operator*(double scalar, Mass value)
    {
        return Mass::from_raw_si(scalar * value.raw_si());
    }

    export Mass operator/(Mass value, double scalar)
    {
        return Mass::from_raw_si(value.raw_si() / scalar);
    }

    export double operator/(Mass left, Mass right)
    {
        return left.raw_si() / right.raw_si();
    }

} // namespace aztro::physics::mass

namespace aztro::physics::mass
{

    export const char* symbol(MassUnit unit)
    {
        switch (unit)
        {
        case MassUnit::Kilograms:
            return "kg";
        case MassUnit::Grams:
            return "g";
        case MassUnit::Milligrams:
            return "mg";
        case MassUnit::Micrograms:
            return "ug";
        case MassUnit::Tons:
            return "t";
        }

        return "";
    }

    export double value_in_unit(Mass value, MassUnit unit)
    {
        switch (unit)
        {
        case MassUnit::Kilograms:
            return value.to_kilograms();
        case MassUnit::Grams:
            return value.to_grams();
        case MassUnit::Milligrams:
            return value.to_milligrams();
        case MassUnit::Micrograms:
            return value.to_micrograms();
        case MassUnit::Tons:
            return value.to_tons();
        }

        return value.to_kilograms();
    }

    QuantityDisplay<Mass, MassUnit> Mass::display_kilograms()
    {
        return display_as(MassUnit::Kilograms);
    }

    QuantityDisplay<Mass, MassUnit> Mass::display_grams()
    {
        return display_as(MassUnit::Grams);
    }

    QuantityDisplay<Mass, MassUnit> Mass::display_milligrams()
    {
        return display_as(MassUnit::Milligrams);
    }

    QuantityDisplay<Mass, MassUnit> Mass::display_micrograms()
    {
        return display_as(MassUnit::Micrograms);
    }

    QuantityDisplay<Mass, MassUnit> Mass::display_tons()
    {
        return display_as(MassUnit::Tons);
    }

    QuantityDisplay<Mass, MassUnit> Mass::display_kilograms_precision(int precision)
    {
        return display_as_precision(MassUnit::Kilograms, precision);
    }

    QuantityDisplay<Mass, MassUnit> Mass::display_grams_precision(int precision)
    {
        return display_as_precision(MassUnit::Grams, precision);
    }

    QuantityDisplay<Mass, MassUnit> Mass::display_milligrams_precision(int precision)
    {
        return display_as_precision(MassUnit::Milligrams, precision);
    }

    QuantityDisplay<Mass, MassUnit> Mass::display_micrograms_precision(int precision)
    {
        return display_as_precision(MassUnit::Micrograms, precision);
    }

    QuantityDisplay<Mass, MassUnit> Mass::display_tons_precision(int precision)
    {
        return display_as_precision(MassUnit::Tons, precision);
    }

    export std::ostream& operator<<(std::ostream& stream, Mass value)
    {
        return stream << value.to_kilograms() << " kg";
    }

} // namespace aztro::physics::mass
