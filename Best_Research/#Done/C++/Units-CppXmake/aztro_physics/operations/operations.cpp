module;
#include <optional>

export module aztro_physics:operations;
import :time;
import :mass;
import :acceleration;
import :velocity;
import :length;
import :force;

namespace aztro::physics::length
{

    export Length operator+(Length left, Length right);
    export Length operator-(Length left, Length right);
    export Length operator-(Length value);
    export Length operator*(Length value, double scalar);
    export Length operator*(double scalar, Length value);
    export Length operator/(Length value, double scalar);
    export double operator/(Length left, Length right);

    export velocity::Velocity operator/(Length distance, time::Time time);
    export time::Time operator/(Length distance, velocity::Velocity velocity);

} // namespace aztro::physics::length

namespace aztro::physics::time
{

    export Time operator+(Time left, Time right);
    export Time operator-(Time left, Time right);
    export Time operator-(Time value);
    export Time operator*(Time value, double scalar);
    export Time operator*(double scalar, Time value);
    export Time operator/(Time value, double scalar);
    export double operator/(Time left, Time right);

    export length::Length operator*(Time time, velocity::Velocity velocity);
    export velocity::Velocity operator*(Time time, acceleration::Acceleration acceleration);

} // namespace aztro::physics::time

namespace aztro::physics::mass
{

    export Mass operator+(Mass left, Mass right);
    export Mass operator-(Mass left, Mass right);
    export Mass operator-(Mass value);
    export Mass operator*(Mass value, double scalar);
    export Mass operator*(double scalar, Mass value);
    export Mass operator/(Mass value, double scalar);
    export double operator/(Mass left, Mass right);

    export force::Force operator*(Mass mass, acceleration::Acceleration acceleration);

} // namespace aztro::physics::mass

namespace aztro::physics::velocity
{

    export Velocity operator+(Velocity left, Velocity right);
    export Velocity operator-(Velocity left, Velocity right);
    export Velocity operator-(Velocity value);
    export Velocity operator*(Velocity value, double scalar);
    export Velocity operator*(double scalar, Velocity value);
    export Velocity operator/(Velocity value, double scalar);
    export double operator/(Velocity left, Velocity right);

    export Velocity calculate(length::Length distance, time::Time time);
    export std::optional<Velocity> checked_calculate(length::Length distance, time::Time time);

    export length::Length operator*(Velocity velocity, time::Time time);
    export acceleration::Acceleration operator/(Velocity velocity, time::Time time);
    export time::Time operator/(Velocity velocity, acceleration::Acceleration acceleration);

} // namespace aztro::physics::velocity

namespace aztro::physics::acceleration
{

    export Acceleration operator+(Acceleration left, Acceleration right);
    export Acceleration operator-(Acceleration left, Acceleration right);
    export Acceleration operator-(Acceleration value);
    export Acceleration operator*(Acceleration value, double scalar);
    export Acceleration operator*(double scalar, Acceleration value);
    export Acceleration operator/(Acceleration value, double scalar);
    export double operator/(Acceleration left, Acceleration right);

    export Acceleration calculate(velocity::Velocity velocity, time::Time time);
    export std::optional<Acceleration> checked_calculate(velocity::Velocity velocity, time::Time time);

    export velocity::Velocity operator*(Acceleration acceleration, time::Time time);
    export force::Force operator*(Acceleration acceleration, mass::Mass mass);

} // namespace aztro::physics::acceleration

namespace aztro::physics::force
{

    export Force operator+(Force left, Force right);
    export Force operator-(Force left, Force right);
    export Force operator-(Force value);
    export Force operator*(Force value, double scalar);
    export Force operator*(double scalar, Force value);
    export Force operator/(Force value, double scalar);
    export double operator/(Force left, Force right);

    export Force calculate(mass::Mass mass, acceleration::Acceleration acceleration);

    export acceleration::Acceleration operator/(Force force, mass::Mass mass);
    export mass::Mass operator/(Force force, acceleration::Acceleration acceleration);

} // namespace aztro::physics::force

namespace aztro::physics::length
{

    velocity::Velocity operator/(Length distance, time::Time time)
    {
        return velocity::Velocity::meters_per_second(distance.to_meters() / time.to_seconds());
    }

    time::Time operator/(Length distance, velocity::Velocity velocity)
    {
        return time::Time::seconds(distance.to_meters() / velocity.to_meters_per_second());
    }

} // namespace aztro::physics::length

namespace aztro::physics::velocity
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

} // namespace aztro::physics::velocity

namespace aztro::physics::time
{

    length::Length operator*(Time time, velocity::Velocity velocity)
    {
        return length::Length::meters(time.to_seconds() * velocity.to_meters_per_second());
    }

    velocity::Velocity operator*(Time time, acceleration::Acceleration acceleration)
    {
        return velocity::Velocity::meters_per_second(time.to_seconds() * acceleration.to_meters_per_second_squared());
    }

} // namespace aztro::physics::time

namespace aztro::physics::acceleration
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

} // namespace aztro::physics::acceleration

namespace aztro::physics::mass
{

    force::Force operator*(Mass mass, acceleration::Acceleration acceleration)
    {
        return force::Force::newtons(mass.to_kilograms() * acceleration.to_meters_per_second_squared());
    }

} // namespace aztro::physics::mass

namespace aztro::physics::force
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

} // namespace aztro::physics::force

namespace aztro::physics::length
{

    std::optional<velocity::Velocity> Length::checked_div_time(time::Time time)
    {
        if (time.to_seconds() == 0.0)
        {
            return std::nullopt;
        }

        return *this / time;
    }

    std::optional<time::Time> Length::checked_div_velocity(velocity::Velocity velocity)
    {
        if (velocity.to_meters_per_second() == 0.0)
        {
            return std::nullopt;
        }

        return *this / velocity;
    }

} // namespace aztro::physics::length

namespace aztro::physics::velocity
{

    std::optional<Velocity> checked_calculate(length::Length distance, time::Time time)
    {
        if (time.to_seconds() == 0.0)
        {
            return std::nullopt;
        }

        return calculate(distance, time);
    }

    std::optional<acceleration::Acceleration> Velocity::checked_div_time(time::Time time)
    {
        if (time.to_seconds() == 0.0)
        {
            return std::nullopt;
        }

        return *this / time;
    }

    std::optional<time::Time> Velocity::checked_div_acceleration(acceleration::Acceleration acceleration)
    {
        if (acceleration.to_meters_per_second_squared() == 0.0)
        {
            return std::nullopt;
        }

        return *this / acceleration;
    }

} // namespace aztro::physics::velocity

namespace aztro::physics::force
{

    std::optional<acceleration::Acceleration> Force::checked_div_mass(mass::Mass mass)
    {
        if (mass.to_kilograms() == 0.0)
        {
            return std::nullopt;
        }

        return *this / mass;
    }

    std::optional<mass::Mass> Force::checked_div_acceleration(acceleration::Acceleration acceleration)
    {
        if (acceleration.to_meters_per_second_squared() == 0.0)
        {
            return std::nullopt;
        }

        return *this / acceleration;
    }

} // namespace aztro::physics::force

namespace aztro::physics::acceleration
{

    std::optional<Acceleration> checked_calculate(velocity::Velocity velocity, time::Time time)
    {
        if (time.to_seconds() == 0.0)
        {
            return std::nullopt;
        }

        return calculate(velocity, time);
    }

} // namespace aztro::physics::acceleration
