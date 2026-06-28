#include "units/acceleration/acceleration.hpp"

#include "units/detail/math.hpp"

namespace units::acceleration
{

    double Acceleration::standardGravityMetersPerSecondSquared()
    {
        return 9.80665;
    }

    Acceleration::Acceleration(double metersPerSecondSquared) : _metersPerSecondSquared(metersPerSecondSquared)
    {
    }

    Acceleration Acceleration::fromRawSi(double metersPerSecondSquared)
    {
        return Acceleration(metersPerSecondSquared);
    }

    Acceleration Acceleration::metersPerSecondSquared(double value)
    {
        return Acceleration(value);
    }

    Acceleration Acceleration::standardGravity(double value)
    {
        return Acceleration(value * standardGravityMetersPerSecondSquared());
    }

    double Acceleration::rawSi()
    {
        return _metersPerSecondSquared;
    }

    double Acceleration::asMetersPerSecondSquared()
    {
        return _metersPerSecondSquared;
    }

    double Acceleration::asStandardGravity()
    {
        return _metersPerSecondSquared / standardGravityMetersPerSecondSquared();
    }

    bool Acceleration::approximatelyEquals(Acceleration other, double epsilon)
    {
        return detail::absolute(_metersPerSecondSquared - other._metersPerSecondSquared) <= epsilon;
    }

    QuantityDisplay<Acceleration, AccelerationUnit> Acceleration::displayAs(AccelerationUnit unit)
    {
        return QuantityDisplay<Acceleration, AccelerationUnit>(*this, unit);
    }

    QuantityDisplay<Acceleration, AccelerationUnit> Acceleration::displayAsPrecision(AccelerationUnit unit, int precision)
    {
        return QuantityDisplay<Acceleration, AccelerationUnit>(*this, unit, precision);
    }

    bool operator==(Acceleration left, Acceleration right)
    {
        return left._metersPerSecondSquared == right._metersPerSecondSquared;
    }

} // namespace units::acceleration
