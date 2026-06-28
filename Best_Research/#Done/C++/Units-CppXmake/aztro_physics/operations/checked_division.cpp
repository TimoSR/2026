module;

#include <format>
#include <iomanip>
#include <limits>
#include <optional>
#include <ostream>

module aztro_physics;

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
