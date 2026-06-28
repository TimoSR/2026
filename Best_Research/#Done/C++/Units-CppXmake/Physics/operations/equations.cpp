#include "physics/operations/operations.hpp"

namespace physics::length
{

    velocity::Velocity operator/(Length distance, time::Time time)
    {
        return velocity::Velocity::meters_per_second(distance.to_meters() / time.to_seconds());
    }

    time::Time operator/(Length distance, velocity::Velocity velocity)
    {
        return time::Time::seconds(distance.to_meters() / velocity.to_meters_per_second());
    }

} // namespace physics::length

namespace physics::velocity
{

    Velocity calculate(length::Length distance, time::Time time)
    {
        return Velocity::meters_per_second(distance.to_meters() / time.to_seconds());
    }

    length::Length operator*(Velocity velocity, time::Time time)
    {
        return length::Length::meters(velocity.to_meters_per_second() * time.to_seconds());
    }

    acceleration::Acceleration operator/(Velocity velocity, time::Time time)
    {
        return acceleration::Acceleration::meters_per_second_squared(velocity.to_meters_per_second() / time.to_seconds());
    }

    time::Time operator/(Velocity velocity, acceleration::Acceleration acceleration)
    {
        return time::Time::seconds(velocity.to_meters_per_second() / acceleration.to_meters_per_second_squared());
    }

} // namespace physics::velocity

namespace physics::time
{

    length::Length operator*(Time time, velocity::Velocity velocity)
    {
        return length::Length::meters(time.to_seconds() * velocity.to_meters_per_second());
    }

    velocity::Velocity operator*(Time time, acceleration::Acceleration acceleration)
    {
        return velocity::Velocity::meters_per_second(time.to_seconds() * acceleration.to_meters_per_second_squared());
    }

} // namespace physics::time

namespace physics::acceleration
{

    Acceleration calculate(velocity::Velocity velocity, time::Time time)
    {
        return Acceleration::meters_per_second_squared(velocity.to_meters_per_second() / time.to_seconds());
    }

    velocity::Velocity operator*(Acceleration acceleration, time::Time time)
    {
        return velocity::Velocity::meters_per_second(acceleration.to_meters_per_second_squared() * time.to_seconds());
    }

    force::Force operator*(Acceleration acceleration, mass::Mass mass)
    {
        return force::Force::newtons(acceleration.to_meters_per_second_squared() * mass.to_kilograms());
    }

} // namespace physics::acceleration

namespace physics::mass
{

    force::Force operator*(Mass mass, acceleration::Acceleration acceleration)
    {
        return force::Force::newtons(mass.to_kilograms() * acceleration.to_meters_per_second_squared());
    }

} // namespace physics::mass

namespace physics::force
{

    Force calculate(mass::Mass mass, acceleration::Acceleration acceleration)
    {
        return Force::newtons(mass.to_kilograms() * acceleration.to_meters_per_second_squared());
    }

    acceleration::Acceleration operator/(Force force, mass::Mass mass)
    {
        return acceleration::Acceleration::meters_per_second_squared(force.to_newtons() / mass.to_kilograms());
    }

    mass::Mass operator/(Force force, acceleration::Acceleration acceleration)
    {
        return mass::Mass::kilograms(force.to_newtons() / acceleration.to_meters_per_second_squared());
    }

} // namespace physics::force
