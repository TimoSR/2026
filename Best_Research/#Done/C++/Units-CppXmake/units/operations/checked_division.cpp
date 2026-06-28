#include "units/operations/operations.hpp"

#include <optional>

namespace units::length
{

    std::optional<velocity::Velocity> Length::checkedDivTime(time::Time time)
    {
        if (time.asSeconds() == 0.0)
        {
            return std::nullopt;
        }

        return *this / time;
    }

    std::optional<time::Time> Length::checkedDivVelocity(velocity::Velocity velocity)
    {
        if (velocity.asMetersPerSecond() == 0.0)
        {
            return std::nullopt;
        }

        return *this / velocity;
    }

} // namespace units::length

namespace units::velocity
{

    std::optional<acceleration::Acceleration> Velocity::checkedDivTime(time::Time time)
    {
        if (time.asSeconds() == 0.0)
        {
            return std::nullopt;
        }

        return *this / time;
    }

    std::optional<time::Time> Velocity::checkedDivAcceleration(acceleration::Acceleration acceleration)
    {
        if (acceleration.asMetersPerSecondSquared() == 0.0)
        {
            return std::nullopt;
        }

        return *this / acceleration;
    }

} // namespace units::velocity

namespace units::force
{

    std::optional<acceleration::Acceleration> Force::checkedDivMass(mass::Mass mass)
    {
        if (mass.asKilograms() == 0.0)
        {
            return std::nullopt;
        }

        return *this / mass;
    }

    std::optional<mass::Mass> Force::checkedDivAcceleration(acceleration::Acceleration acceleration)
    {
        if (acceleration.asMetersPerSecondSquared() == 0.0)
        {
            return std::nullopt;
        }

        return *this / acceleration;
    }

} // namespace units::force
