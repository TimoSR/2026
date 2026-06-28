#pragma once

#include <iosfwd>

#include "units/acceleration/acceleration.hpp"
#include "units/detail/quantity_display.hpp"
#include "units/force/force.hpp"
#include "units/length/length.hpp"
#include "units/mass/mass.hpp"
#include "units/time/time.hpp"
#include "units/velocity/velocity.hpp"

namespace units::length
{

    const char* symbol(LengthUnit unit);
    double valueInUnit(Length value, LengthUnit unit);
    std::ostream& operator<<(std::ostream& stream, Length value);

} // namespace units::length

namespace units::time
{

    const char* symbol(TimeUnit unit);
    double valueInUnit(Time value, TimeUnit unit);
    std::ostream& operator<<(std::ostream& stream, Time value);

} // namespace units::time

namespace units::mass
{

    const char* symbol(MassUnit unit);
    double valueInUnit(Mass value, MassUnit unit);
    std::ostream& operator<<(std::ostream& stream, Mass value);

} // namespace units::mass

namespace units::velocity
{

    const char* symbol(VelocityUnit unit);
    double valueInUnit(Velocity value, VelocityUnit unit);
    std::ostream& operator<<(std::ostream& stream, Velocity value);

} // namespace units::velocity

namespace units::acceleration
{

    const char* symbol(AccelerationUnit unit);
    double valueInUnit(Acceleration value, AccelerationUnit unit);
    std::ostream& operator<<(std::ostream& stream, Acceleration value);

} // namespace units::acceleration

namespace units::force
{

    const char* symbol(ForceUnit unit);
    double valueInUnit(Force value, ForceUnit unit);
    std::ostream& operator<<(std::ostream& stream, Force value);

} // namespace units::force

namespace units
{

    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<length::Length, length::LengthUnit> display);
    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<time::Time, time::TimeUnit> display);
    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<mass::Mass, mass::MassUnit> display);
    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<velocity::Velocity, velocity::VelocityUnit> display);
    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<acceleration::Acceleration, acceleration::AccelerationUnit> display);
    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<force::Force, force::ForceUnit> display);

} // namespace units
