#include "units/operations/operations.hpp"

#include <optional>

namespace units
{

    Length operator+(Length left, Length right)
    {
        return Length::fromRawSi(left.rawSi() + right.rawSi());
    }

    Length operator-(Length left, Length right)
    {
        return Length::fromRawSi(left.rawSi() - right.rawSi());
    }

    Length operator-(Length value)
    {
        return Length::fromRawSi(-value.rawSi());
    }

    Length operator*(Length value, double scalar)
    {
        return Length::fromRawSi(value.rawSi() * scalar);
    }

    Length operator*(double scalar, Length value)
    {
        return Length::fromRawSi(scalar * value.rawSi());
    }

    Length operator/(Length value, double scalar)
    {
        return Length::fromRawSi(value.rawSi() / scalar);
    }

    double operator/(Length left, Length right)
    {
        return left.rawSi() / right.rawSi();
    }

    Time operator+(Time left, Time right)
    {
        return Time::fromRawSi(left.rawSi() + right.rawSi());
    }

    Time operator-(Time left, Time right)
    {
        return Time::fromRawSi(left.rawSi() - right.rawSi());
    }

    Time operator-(Time value)
    {
        return Time::fromRawSi(-value.rawSi());
    }

    Time operator*(Time value, double scalar)
    {
        return Time::fromRawSi(value.rawSi() * scalar);
    }

    Time operator*(double scalar, Time value)
    {
        return Time::fromRawSi(scalar * value.rawSi());
    }

    Time operator/(Time value, double scalar)
    {
        return Time::fromRawSi(value.rawSi() / scalar);
    }

    double operator/(Time left, Time right)
    {
        return left.rawSi() / right.rawSi();
    }

    Mass operator+(Mass left, Mass right)
    {
        return Mass::fromRawSi(left.rawSi() + right.rawSi());
    }

    Mass operator-(Mass left, Mass right)
    {
        return Mass::fromRawSi(left.rawSi() - right.rawSi());
    }

    Mass operator-(Mass value)
    {
        return Mass::fromRawSi(-value.rawSi());
    }

    Mass operator*(Mass value, double scalar)
    {
        return Mass::fromRawSi(value.rawSi() * scalar);
    }

    Mass operator*(double scalar, Mass value)
    {
        return Mass::fromRawSi(scalar * value.rawSi());
    }

    Mass operator/(Mass value, double scalar)
    {
        return Mass::fromRawSi(value.rawSi() / scalar);
    }

    double operator/(Mass left, Mass right)
    {
        return left.rawSi() / right.rawSi();
    }

    Velocity operator+(Velocity left, Velocity right)
    {
        return Velocity::fromRawSi(left.rawSi() + right.rawSi());
    }

    Velocity operator-(Velocity left, Velocity right)
    {
        return Velocity::fromRawSi(left.rawSi() - right.rawSi());
    }

    Velocity operator-(Velocity value)
    {
        return Velocity::fromRawSi(-value.rawSi());
    }

    Velocity operator*(Velocity value, double scalar)
    {
        return Velocity::fromRawSi(value.rawSi() * scalar);
    }

    Velocity operator*(double scalar, Velocity value)
    {
        return Velocity::fromRawSi(scalar * value.rawSi());
    }

    Velocity operator/(Velocity value, double scalar)
    {
        return Velocity::fromRawSi(value.rawSi() / scalar);
    }

    double operator/(Velocity left, Velocity right)
    {
        return left.rawSi() / right.rawSi();
    }

    Acceleration operator+(Acceleration left, Acceleration right)
    {
        return Acceleration::fromRawSi(left.rawSi() + right.rawSi());
    }

    Acceleration operator-(Acceleration left, Acceleration right)
    {
        return Acceleration::fromRawSi(left.rawSi() - right.rawSi());
    }

    Acceleration operator-(Acceleration value)
    {
        return Acceleration::fromRawSi(-value.rawSi());
    }

    Acceleration operator*(Acceleration value, double scalar)
    {
        return Acceleration::fromRawSi(value.rawSi() * scalar);
    }

    Acceleration operator*(double scalar, Acceleration value)
    {
        return Acceleration::fromRawSi(scalar * value.rawSi());
    }

    Acceleration operator/(Acceleration value, double scalar)
    {
        return Acceleration::fromRawSi(value.rawSi() / scalar);
    }

    double operator/(Acceleration left, Acceleration right)
    {
        return left.rawSi() / right.rawSi();
    }

    Force operator+(Force left, Force right)
    {
        return Force::fromRawSi(left.rawSi() + right.rawSi());
    }

    Force operator-(Force left, Force right)
    {
        return Force::fromRawSi(left.rawSi() - right.rawSi());
    }

    Force operator-(Force value)
    {
        return Force::fromRawSi(-value.rawSi());
    }

    Force operator*(Force value, double scalar)
    {
        return Force::fromRawSi(value.rawSi() * scalar);
    }

    Force operator*(double scalar, Force value)
    {
        return Force::fromRawSi(scalar * value.rawSi());
    }

    Force operator/(Force value, double scalar)
    {
        return Force::fromRawSi(value.rawSi() / scalar);
    }

    double operator/(Force left, Force right)
    {
        return left.rawSi() / right.rawSi();
    }

    Velocity operator/(Length distance, Time time)
    {
        return Velocity::metersPerSecond(distance.asMeters() / time.asSeconds());
    }

    Time operator/(Length distance, Velocity velocity)
    {
        return Time::seconds(distance.asMeters() / velocity.asMetersPerSecond());
    }

    Length operator*(Velocity velocity, Time time)
    {
        return Length::meters(velocity.asMetersPerSecond() * time.asSeconds());
    }

    Length operator*(Time time, Velocity velocity)
    {
        return Length::meters(time.asSeconds() * velocity.asMetersPerSecond());
    }

    Acceleration operator/(Velocity velocity, Time time)
    {
        return Acceleration::metersPerSecondSquared(velocity.asMetersPerSecond() / time.asSeconds());
    }

    Time operator/(Velocity velocity, Acceleration acceleration)
    {
        return Time::seconds(velocity.asMetersPerSecond() / acceleration.asMetersPerSecondSquared());
    }

    Velocity operator*(Acceleration acceleration, Time time)
    {
        return Velocity::metersPerSecond(acceleration.asMetersPerSecondSquared() * time.asSeconds());
    }

    Velocity operator*(Time time, Acceleration acceleration)
    {
        return Velocity::metersPerSecond(time.asSeconds() * acceleration.asMetersPerSecondSquared());
    }

    Force operator*(Mass mass, Acceleration acceleration)
    {
        return Force::newtons(mass.asKilograms() * acceleration.asMetersPerSecondSquared());
    }

    Force operator*(Acceleration acceleration, Mass mass)
    {
        return Force::newtons(acceleration.asMetersPerSecondSquared() * mass.asKilograms());
    }

    Acceleration operator/(Force force, Mass mass)
    {
        return Acceleration::metersPerSecondSquared(force.asNewtons() / mass.asKilograms());
    }

    Mass operator/(Force force, Acceleration acceleration)
    {
        return Mass::kilograms(force.asNewtons() / acceleration.asMetersPerSecondSquared());
    }

    std::optional<Velocity> Length::checkedDivTime(Time time)
    {
        if (time.asSeconds() == 0.0)
        {
            return std::nullopt;
        }

        return *this / time;
    }

    std::optional<Time> Length::checkedDivVelocity(Velocity velocity)
    {
        if (velocity.asMetersPerSecond() == 0.0)
        {
            return std::nullopt;
        }

        return *this / velocity;
    }

    std::optional<Acceleration> Velocity::checkedDivTime(Time time)
    {
        if (time.asSeconds() == 0.0)
        {
            return std::nullopt;
        }

        return *this / time;
    }

    std::optional<Time> Velocity::checkedDivAcceleration(Acceleration acceleration)
    {
        if (acceleration.asMetersPerSecondSquared() == 0.0)
        {
            return std::nullopt;
        }

        return *this / acceleration;
    }

    std::optional<Acceleration> Force::checkedDivMass(Mass mass)
    {
        if (mass.asKilograms() == 0.0)
        {
            return std::nullopt;
        }

        return *this / mass;
    }

    std::optional<Mass> Force::checkedDivAcceleration(Acceleration acceleration)
    {
        if (acceleration.asMetersPerSecondSquared() == 0.0)
        {
            return std::nullopt;
        }

        return *this / acceleration;
    }

} // namespace units
