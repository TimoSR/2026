#include "Physics/velocity/velocity.hpp"

#include "Physics/detail/math.hpp"

namespace Physics::velocity
{

    Velocity::Velocity(double metersPerSecond) : _metersPerSecond(metersPerSecond)
    {
    }

    Velocity Velocity::fromRawSi(double metersPerSecond)
    {
        return Velocity(metersPerSecond);
    }

    Velocity Velocity::metersPerSecond(double value)
    {
        return Velocity(value);
    }

    Velocity Velocity::kilometersPerHour(double value)
    {
        return Velocity(value / 3.6);
    }

    double Velocity::rawSi()
    {
        return _metersPerSecond;
    }

    double Velocity::asMetersPerSecond()
    {
        return _metersPerSecond;
    }

    double Velocity::asKilometersPerHour()
    {
        return _metersPerSecond * 3.6;
    }

    bool Velocity::approximatelyEquals(Velocity other, double epsilon)
    {
        return detail::absolute(_metersPerSecond - other._metersPerSecond) <= epsilon;
    }

    QuantityDisplay<Velocity, VelocityUnit> Velocity::displayAs(VelocityUnit unit)
    {
        return QuantityDisplay<Velocity, VelocityUnit>(*this, unit);
    }

    QuantityDisplay<Velocity, VelocityUnit> Velocity::displayAsPrecision(VelocityUnit unit, int precision)
    {
        return QuantityDisplay<Velocity, VelocityUnit>(*this, unit, precision);
    }

    bool operator==(Velocity left, Velocity right)
    {
        return left._metersPerSecond == right._metersPerSecond;
    }

} // namespace Physics::velocity
