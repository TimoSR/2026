#pragma once

#include <optional>

#include "physics/acceleration/acceleration.hpp"
#include "physics/force/force.hpp"
#include "physics/length/length.hpp"
#include "physics/mass/mass.hpp"
#include "physics/time/time.hpp"
#include "physics/velocity/velocity.hpp"

namespace physics::length
{

    Length operator+(Length left, Length right);
    Length operator-(Length left, Length right);
    Length operator-(Length value);
    Length operator*(Length value, double scalar);
    Length operator*(double scalar, Length value);
    Length operator/(Length value, double scalar);
    double operator/(Length left, Length right);

    velocity::Velocity operator/(Length distance, time::Time time);
    time::Time operator/(Length distance, velocity::Velocity velocity);

} // namespace physics::length

namespace physics::time
{

    Time operator+(Time left, Time right);
    Time operator-(Time left, Time right);
    Time operator-(Time value);
    Time operator*(Time value, double scalar);
    Time operator*(double scalar, Time value);
    Time operator/(Time value, double scalar);
    double operator/(Time left, Time right);

    length::Length operator*(Time time, velocity::Velocity velocity);
    velocity::Velocity operator*(Time time, acceleration::Acceleration acceleration);

} // namespace physics::time

namespace physics::mass
{

    Mass operator+(Mass left, Mass right);
    Mass operator-(Mass left, Mass right);
    Mass operator-(Mass value);
    Mass operator*(Mass value, double scalar);
    Mass operator*(double scalar, Mass value);
    Mass operator/(Mass value, double scalar);
    double operator/(Mass left, Mass right);

    force::Force operator*(Mass mass, acceleration::Acceleration acceleration);

} // namespace physics::mass

namespace physics::velocity
{

    Velocity operator+(Velocity left, Velocity right);
    Velocity operator-(Velocity left, Velocity right);
    Velocity operator-(Velocity value);
    Velocity operator*(Velocity value, double scalar);
    Velocity operator*(double scalar, Velocity value);
    Velocity operator/(Velocity value, double scalar);
    double operator/(Velocity left, Velocity right);

    Velocity calculate(length::Length distance, time::Time time);
    std::optional<Velocity> checked_calculate(length::Length distance, time::Time time);

    length::Length operator*(Velocity velocity, time::Time time);
    acceleration::Acceleration operator/(Velocity velocity, time::Time time);
    time::Time operator/(Velocity velocity, acceleration::Acceleration acceleration);

} // namespace physics::velocity

namespace physics::acceleration
{

    Acceleration operator+(Acceleration left, Acceleration right);
    Acceleration operator-(Acceleration left, Acceleration right);
    Acceleration operator-(Acceleration value);
    Acceleration operator*(Acceleration value, double scalar);
    Acceleration operator*(double scalar, Acceleration value);
    Acceleration operator/(Acceleration value, double scalar);
    double operator/(Acceleration left, Acceleration right);

    Acceleration calculate(velocity::Velocity velocity, time::Time time);
    std::optional<Acceleration> checked_calculate(velocity::Velocity velocity, time::Time time);

    velocity::Velocity operator*(Acceleration acceleration, time::Time time);
    force::Force operator*(Acceleration acceleration, mass::Mass mass);

} // namespace physics::acceleration

namespace physics::force
{

    Force operator+(Force left, Force right);
    Force operator-(Force left, Force right);
    Force operator-(Force value);
    Force operator*(Force value, double scalar);
    Force operator*(double scalar, Force value);
    Force operator/(Force value, double scalar);
    double operator/(Force left, Force right);

    Force calculate(mass::Mass mass, acceleration::Acceleration acceleration);

    acceleration::Acceleration operator/(Force force, mass::Mass mass);
    mass::Mass operator/(Force force, acceleration::Acceleration acceleration);

} // namespace physics::force
