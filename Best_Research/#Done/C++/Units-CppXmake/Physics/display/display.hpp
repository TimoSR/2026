#pragma once

#include <iosfwd>

#include "Physics/acceleration/acceleration.hpp"
#include "Physics/detail/quantity_display.hpp"
#include "Physics/force/force.hpp"
#include "Physics/length/length.hpp"
#include "Physics/mass/mass.hpp"
#include "Physics/time/time.hpp"
#include "Physics/velocity/velocity.hpp"

namespace Physics::length
{

    const char* symbol(LengthUnit unit);
    double valueInUnit(Length value, LengthUnit unit);
    std::ostream& operator<<(std::ostream& stream, Length value);

} // namespace Physics::length

namespace Physics::time
{

    const char* symbol(TimeUnit unit);
    double valueInUnit(Time value, TimeUnit unit);
    std::ostream& operator<<(std::ostream& stream, Time value);

} // namespace Physics::time

namespace Physics::mass
{

    const char* symbol(MassUnit unit);
    double valueInUnit(Mass value, MassUnit unit);
    std::ostream& operator<<(std::ostream& stream, Mass value);

} // namespace Physics::mass

namespace Physics::velocity
{

    const char* symbol(VelocityUnit unit);
    double valueInUnit(Velocity value, VelocityUnit unit);
    std::ostream& operator<<(std::ostream& stream, Velocity value);

} // namespace Physics::velocity

namespace Physics::acceleration
{

    const char* symbol(AccelerationUnit unit);
    double valueInUnit(Acceleration value, AccelerationUnit unit);
    std::ostream& operator<<(std::ostream& stream, Acceleration value);

} // namespace Physics::acceleration

namespace Physics::force
{

    const char* symbol(ForceUnit unit);
    double valueInUnit(Force value, ForceUnit unit);
    std::ostream& operator<<(std::ostream& stream, Force value);

} // namespace Physics::force

namespace Physics
{

    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<length::Length, length::LengthUnit> display);
    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<time::Time, time::TimeUnit> display);
    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<mass::Mass, mass::MassUnit> display);
    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<velocity::Velocity, velocity::VelocityUnit> display);
    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<acceleration::Acceleration, acceleration::AccelerationUnit> display);
    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<force::Force, force::ForceUnit> display);

} // namespace Physics
