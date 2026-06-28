#pragma once

#include <iosfwd>

#include "physics/acceleration/acceleration.hpp"
#include "physics/detail/quantity_display.hpp"
#include "physics/force/force.hpp"
#include "physics/length/length.hpp"
#include "physics/mass/mass.hpp"
#include "physics/time/time.hpp"
#include "physics/velocity/velocity.hpp"

namespace physics::length
{

    const char* symbol(LengthUnit unit);
    double value_in_unit(Length value, LengthUnit unit);
    std::ostream& operator<<(std::ostream& stream, Length value);

} // namespace physics::length

namespace physics::time
{

    const char* symbol(TimeUnit unit);
    double value_in_unit(Time value, TimeUnit unit);
    std::ostream& operator<<(std::ostream& stream, Time value);

} // namespace physics::time

namespace physics::mass
{

    const char* symbol(MassUnit unit);
    double value_in_unit(Mass value, MassUnit unit);
    std::ostream& operator<<(std::ostream& stream, Mass value);

} // namespace physics::mass

namespace physics::velocity
{

    const char* symbol(VelocityUnit unit);
    double value_in_unit(Velocity value, VelocityUnit unit);
    std::ostream& operator<<(std::ostream& stream, Velocity value);

} // namespace physics::velocity

namespace physics::acceleration
{

    const char* symbol(AccelerationUnit unit);
    double value_in_unit(Acceleration value, AccelerationUnit unit);
    std::ostream& operator<<(std::ostream& stream, Acceleration value);

} // namespace physics::acceleration

namespace physics::force
{

    const char* symbol(ForceUnit unit);
    double value_in_unit(Force value, ForceUnit unit);
    std::ostream& operator<<(std::ostream& stream, Force value);

} // namespace physics::force

namespace physics
{

    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<length::Length, length::LengthUnit> display);
    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<time::Time, time::TimeUnit> display);
    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<mass::Mass, mass::MassUnit> display);
    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<velocity::Velocity, velocity::VelocityUnit> display);
    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<acceleration::Acceleration, acceleration::AccelerationUnit> display);
    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<force::Force, force::ForceUnit> display);

} // namespace physics
