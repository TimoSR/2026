module;
#include <cmath>
#include <optional>
#include <ostream>

export module aztro_physics:force;
import :detail_math;
import :quantity_display;
import :mass;
import :acceleration;

namespace aztro::physics::force
{

    export enum class ForceUnit
    {
        Newtons,
        Millinewtons,
        Kilonewtons,
    };

    export class Force
    {
        private:
            double _newtons;

        private:
            explicit Force(double newtons)
            {
                _newtons = newtons;
            }

        public:
            static Force from_raw_si(double newtons);

            static Force newtons(double value);

            static std::optional<Force> try_newtons(double value);

            static Force millinewtons(double value);

            static std::optional<Force> try_millinewtons(double value);

            static Force kilonewtons(double value);

            static std::optional<Force> try_kilonewtons(double value);

            double raw_si();

            double to_newtons();

            double to_millinewtons();

            double to_kilonewtons();

            bool approximately_equals(Force other, double epsilon);

            QuantityDisplay<Force, ForceUnit> display_as(ForceUnit unit);

            QuantityDisplay<Force, ForceUnit> display_as_precision(ForceUnit unit, int precision);

            QuantityDisplay<Force, ForceUnit> display_newtons();
            QuantityDisplay<Force, ForceUnit> display_millinewtons();
            QuantityDisplay<Force, ForceUnit> display_kilonewtons();

            QuantityDisplay<Force, ForceUnit> display_newtons_precision(int precision);
            QuantityDisplay<Force, ForceUnit> display_millinewtons_precision(int precision);
            QuantityDisplay<Force, ForceUnit> display_kilonewtons_precision(int precision);

            std::optional<acceleration::Acceleration> checked_div_mass(mass::Mass mass);
            std::optional<mass::Mass> checked_div_acceleration(acceleration::Acceleration acceleration);

            friend bool operator==(Force left, Force right);
    };

    export Force newtons(double value);
    export std::optional<Force> try_newtons(double value);
    export Force force(double value);
    export Force millinewtons(double value);
    export std::optional<Force> try_millinewtons(double value);
    export Force kilonewtons(double value);
    export std::optional<Force> try_kilonewtons(double value);

} // namespace aztro::physics::force

namespace aztro::physics::force
{

    Force Force::from_raw_si(double newtons)
    {
        return Force(newtons);
    }

    Force Force::newtons(double value)
    {
        return Force(value);
    }

    Force Force::millinewtons(double value)
    {
        return Force(value / 1'000.0);
    }

    Force Force::kilonewtons(double value)
    {
        return Force(value * 1'000.0);
    }

    double Force::raw_si()
    {
        return _newtons;
    }

    double Force::to_newtons()
    {
        return _newtons;
    }

    double Force::to_millinewtons()
    {
        return _newtons * 1'000.0;
    }

    double Force::to_kilonewtons()
    {
        return _newtons / 1'000.0;
    }

    bool Force::approximately_equals(Force other, double epsilon)
    {
        return detail::absolute(_newtons - other._newtons) <= epsilon;
    }

    QuantityDisplay<Force, ForceUnit> Force::display_as(ForceUnit unit)
    {
        return QuantityDisplay<Force, ForceUnit>(*this, unit);
    }

    QuantityDisplay<Force, ForceUnit> Force::display_as_precision(ForceUnit unit, int precision)
    {
        return QuantityDisplay<Force, ForceUnit>(*this, unit, precision);
    }

    export bool operator==(Force left, Force right)
    {
        return left._newtons == right._newtons;
    }

} // namespace aztro::physics::force

namespace aztro::physics::force
{

    std::optional<Force> Force::try_newtons(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Force::newtons(value);
    }

    std::optional<Force> Force::try_millinewtons(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Force::millinewtons(value);
    }

    std::optional<Force> Force::try_kilonewtons(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Force::kilonewtons(value);
    }

    export Force newtons(double value)
    {
        return Force::newtons(value);
    }

    std::optional<Force> try_newtons(double value)
    {
        return Force::try_newtons(value);
    }

    export Force force(double value)
    {
        return Force::newtons(value);
    }

    export Force millinewtons(double value)
    {
        return Force::millinewtons(value);
    }

    std::optional<Force> try_millinewtons(double value)
    {
        return Force::try_millinewtons(value);
    }

    export Force kilonewtons(double value)
    {
        return Force::kilonewtons(value);
    }

    std::optional<Force> try_kilonewtons(double value)
    {
        return Force::try_kilonewtons(value);
    }

} // namespace aztro::physics::force

namespace aztro::physics::force
{

    export Force operator+(Force left, Force right)
    {
        return Force::from_raw_si(left.raw_si() + right.raw_si());
    }

    export Force operator-(Force left, Force right)
    {
        return Force::from_raw_si(left.raw_si() - right.raw_si());
    }

    export Force operator-(Force value)
    {
        return Force::from_raw_si(-value.raw_si());
    }

    export Force operator*(Force value, double scalar)
    {
        return Force::from_raw_si(value.raw_si() * scalar);
    }

    export Force operator*(double scalar, Force value)
    {
        return Force::from_raw_si(scalar * value.raw_si());
    }

    export Force operator/(Force value, double scalar)
    {
        return Force::from_raw_si(value.raw_si() / scalar);
    }

    export double operator/(Force left, Force right)
    {
        return left.raw_si() / right.raw_si();
    }

} // namespace aztro::physics::force

namespace aztro::physics::force
{

    export const char* symbol(ForceUnit unit)
    {
        switch (unit)
        {
        case ForceUnit::Newtons:
            return "N";
        case ForceUnit::Millinewtons:
            return "mN";
        case ForceUnit::Kilonewtons:
            return "kN";
        }

        return "";
    }

    export double value_in_unit(Force value, ForceUnit unit)
    {
        switch (unit)
        {
        case ForceUnit::Newtons:
            return value.to_newtons();
        case ForceUnit::Millinewtons:
            return value.to_millinewtons();
        case ForceUnit::Kilonewtons:
            return value.to_kilonewtons();
        }

        return value.to_newtons();
    }

    QuantityDisplay<Force, ForceUnit> Force::display_newtons()
    {
        return display_as(ForceUnit::Newtons);
    }

    QuantityDisplay<Force, ForceUnit> Force::display_millinewtons()
    {
        return display_as(ForceUnit::Millinewtons);
    }

    QuantityDisplay<Force, ForceUnit> Force::display_kilonewtons()
    {
        return display_as(ForceUnit::Kilonewtons);
    }

    QuantityDisplay<Force, ForceUnit> Force::display_newtons_precision(int precision)
    {
        return display_as_precision(ForceUnit::Newtons, precision);
    }

    QuantityDisplay<Force, ForceUnit> Force::display_millinewtons_precision(int precision)
    {
        return display_as_precision(ForceUnit::Millinewtons, precision);
    }

    QuantityDisplay<Force, ForceUnit> Force::display_kilonewtons_precision(int precision)
    {
        return display_as_precision(ForceUnit::Kilonewtons, precision);
    }

    export std::ostream& operator<<(std::ostream& stream, Force value)
    {
        return stream << value.to_newtons() << " N";
    }

} // namespace aztro::physics::force
