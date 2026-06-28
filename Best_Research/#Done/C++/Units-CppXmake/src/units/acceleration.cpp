#include "units/acceleration.hpp"

#include "units/detail/math.hpp"

namespace units
{

    double Acceleration::standardGravityMetersPerSecondSquared()
    {
        return 9.80665;
    }

    Acceleration::Acceleration(double metersPerSecondSquared) : metersPerSecondSquared_(metersPerSecondSquared)
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
        return metersPerSecondSquared_;
    }

    double Acceleration::asMetersPerSecondSquared()
    {
        return metersPerSecondSquared_;
    }

    double Acceleration::asStandardGravity()
    {
        return metersPerSecondSquared_ / standardGravityMetersPerSecondSquared();
    }

    bool Acceleration::approximatelyEquals(Acceleration other, double epsilon)
    {
        return detail::absolute(metersPerSecondSquared_ - other.metersPerSecondSquared_) <= epsilon;
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
        return left.metersPerSecondSquared_ == right.metersPerSecondSquared_;
    }

} // namespace units
