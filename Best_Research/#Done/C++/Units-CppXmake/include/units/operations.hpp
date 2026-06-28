#pragma once

#include "units/acceleration.hpp"
#include "units/force.hpp"
#include "units/length.hpp"
#include "units/mass.hpp"
#include "units/time.hpp"
#include "units/velocity.hpp"

namespace units
{

    Length operator+(Length left, Length right);
    Length operator-(Length left, Length right);
    Length operator-(Length value);
    Length operator*(Length value, double scalar);
    Length operator*(double scalar, Length value);
    Length operator/(Length value, double scalar);
    double operator/(Length left, Length right);

    Time operator+(Time left, Time right);
    Time operator-(Time left, Time right);
    Time operator-(Time value);
    Time operator*(Time value, double scalar);
    Time operator*(double scalar, Time value);
    Time operator/(Time value, double scalar);
    double operator/(Time left, Time right);

    Mass operator+(Mass left, Mass right);
    Mass operator-(Mass left, Mass right);
    Mass operator-(Mass value);
    Mass operator*(Mass value, double scalar);
    Mass operator*(double scalar, Mass value);
    Mass operator/(Mass value, double scalar);
    double operator/(Mass left, Mass right);

    Velocity operator+(Velocity left, Velocity right);
    Velocity operator-(Velocity left, Velocity right);
    Velocity operator-(Velocity value);
    Velocity operator*(Velocity value, double scalar);
    Velocity operator*(double scalar, Velocity value);
    Velocity operator/(Velocity value, double scalar);
    double operator/(Velocity left, Velocity right);

    Acceleration operator+(Acceleration left, Acceleration right);
    Acceleration operator-(Acceleration left, Acceleration right);
    Acceleration operator-(Acceleration value);
    Acceleration operator*(Acceleration value, double scalar);
    Acceleration operator*(double scalar, Acceleration value);
    Acceleration operator/(Acceleration value, double scalar);
    double operator/(Acceleration left, Acceleration right);

    Force operator+(Force left, Force right);
    Force operator-(Force left, Force right);
    Force operator-(Force value);
    Force operator*(Force value, double scalar);
    Force operator*(double scalar, Force value);
    Force operator/(Force value, double scalar);
    double operator/(Force left, Force right);

    Velocity operator/(Length distance, Time time);
    Time operator/(Length distance, Velocity velocity);
    Length operator*(Velocity velocity, Time time);
    Length operator*(Time time, Velocity velocity);

    Acceleration operator/(Velocity velocity, Time time);
    Time operator/(Velocity velocity, Acceleration acceleration);
    Velocity operator*(Acceleration acceleration, Time time);
    Velocity operator*(Time time, Acceleration acceleration);

    Force operator*(Mass mass, Acceleration acceleration);
    Force operator*(Acceleration acceleration, Mass mass);
    Acceleration operator/(Force force, Mass mass);
    Mass operator/(Force force, Acceleration acceleration);

} // namespace units
