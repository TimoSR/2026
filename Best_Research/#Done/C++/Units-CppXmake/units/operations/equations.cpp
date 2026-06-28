#include "units/operations/operations.hpp"

namespace units::length
{

    velocity::Velocity operator/(Length distance, time::Time time)
    {
        return velocity::Velocity::metersPerSecond(distance.asMeters() / time.asSeconds());
    }

    time::Time operator/(Length distance, velocity::Velocity velocity)
    {
        return time::Time::seconds(distance.asMeters() / velocity.asMetersPerSecond());
    }

} // namespace units::length

namespace units::velocity
{

    length::Length operator*(Velocity velocity, time::Time time)
    {
        return length::Length::meters(velocity.asMetersPerSecond() * time.asSeconds());
    }

    acceleration::Acceleration operator/(Velocity velocity, time::Time time)
    {
        return acceleration::Acceleration::metersPerSecondSquared(velocity.asMetersPerSecond() / time.asSeconds());
    }

    time::Time operator/(Velocity velocity, acceleration::Acceleration acceleration)
    {
        return time::Time::seconds(velocity.asMetersPerSecond() / acceleration.asMetersPerSecondSquared());
    }

} // namespace units::velocity

namespace units::time
{

    length::Length operator*(Time time, velocity::Velocity velocity)
    {
        return length::Length::meters(time.asSeconds() * velocity.asMetersPerSecond());
    }

    velocity::Velocity operator*(Time time, acceleration::Acceleration acceleration)
    {
        return velocity::Velocity::metersPerSecond(time.asSeconds() * acceleration.asMetersPerSecondSquared());
    }

} // namespace units::time

namespace units::acceleration
{

    velocity::Velocity operator*(Acceleration acceleration, time::Time time)
    {
        return velocity::Velocity::metersPerSecond(acceleration.asMetersPerSecondSquared() * time.asSeconds());
    }

    force::Force operator*(Acceleration acceleration, mass::Mass mass)
    {
        return force::Force::newtons(acceleration.asMetersPerSecondSquared() * mass.asKilograms());
    }

} // namespace units::acceleration

namespace units::mass
{

    force::Force operator*(Mass mass, acceleration::Acceleration acceleration)
    {
        return force::Force::newtons(mass.asKilograms() * acceleration.asMetersPerSecondSquared());
    }

} // namespace units::mass

namespace units::force
{

    acceleration::Acceleration operator/(Force force, mass::Mass mass)
    {
        return acceleration::Acceleration::metersPerSecondSquared(force.asNewtons() / mass.asKilograms());
    }

    mass::Mass operator/(Force force, acceleration::Acceleration acceleration)
    {
        return mass::Mass::kilograms(force.asNewtons() / acceleration.asMetersPerSecondSquared());
    }

} // namespace units::force
