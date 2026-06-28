#pragma once

#include "Physics/acceleration/acceleration.hpp"
#include "Physics/force/force.hpp"
#include "Physics/length/length.hpp"
#include "Physics/mass/mass.hpp"
#include "Physics/time/time.hpp"
#include "Physics/velocity/velocity.hpp"

namespace Physics::length
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

} // namespace Physics::length

namespace Physics::time
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

} // namespace Physics::time

namespace Physics::mass
{

    Mass operator+(Mass left, Mass right);
    Mass operator-(Mass left, Mass right);
    Mass operator-(Mass value);
    Mass operator*(Mass value, double scalar);
    Mass operator*(double scalar, Mass value);
    Mass operator/(Mass value, double scalar);
    double operator/(Mass left, Mass right);

    force::Force operator*(Mass mass, acceleration::Acceleration acceleration);

} // namespace Physics::mass

namespace Physics::velocity
{

    Velocity operator+(Velocity left, Velocity right);
    Velocity operator-(Velocity left, Velocity right);
    Velocity operator-(Velocity value);
    Velocity operator*(Velocity value, double scalar);
    Velocity operator*(double scalar, Velocity value);
    Velocity operator/(Velocity value, double scalar);
    double operator/(Velocity left, Velocity right);

    length::Length operator*(Velocity velocity, time::Time time);
    acceleration::Acceleration operator/(Velocity velocity, time::Time time);
    time::Time operator/(Velocity velocity, acceleration::Acceleration acceleration);

} // namespace Physics::velocity

namespace Physics::acceleration
{

    Acceleration operator+(Acceleration left, Acceleration right);
    Acceleration operator-(Acceleration left, Acceleration right);
    Acceleration operator-(Acceleration value);
    Acceleration operator*(Acceleration value, double scalar);
    Acceleration operator*(double scalar, Acceleration value);
    Acceleration operator/(Acceleration value, double scalar);
    double operator/(Acceleration left, Acceleration right);

    velocity::Velocity operator*(Acceleration acceleration, time::Time time);
    force::Force operator*(Acceleration acceleration, mass::Mass mass);

} // namespace Physics::acceleration

namespace Physics::force
{

    Force operator+(Force left, Force right);
    Force operator-(Force left, Force right);
    Force operator-(Force value);
    Force operator*(Force value, double scalar);
    Force operator*(double scalar, Force value);
    Force operator/(Force value, double scalar);
    double operator/(Force left, Force right);

    acceleration::Acceleration operator/(Force force, mass::Mass mass);
    mass::Mass operator/(Force force, acceleration::Acceleration acceleration);

} // namespace Physics::force
