#include "units/operations.hpp"

namespace units {

#define UNITS_DEFINE_SAME_QUANTITY_OPERATORS(Quantity)                                         \
    Quantity operator+(Quantity left, Quantity right) {                                        \
        return Quantity::fromRawSi(left.rawSi() + right.rawSi());                              \
    }                                                                                           \
    Quantity operator-(Quantity left, Quantity right) {                                        \
        return Quantity::fromRawSi(left.rawSi() - right.rawSi());                              \
    }                                                                                           \
    Quantity operator-(Quantity value) {                                                       \
        return Quantity::fromRawSi(-value.rawSi());                                            \
    }                                                                                           \
    Quantity operator*(Quantity value, double scalar) {                                        \
        return Quantity::fromRawSi(value.rawSi() * scalar);                                    \
    }                                                                                           \
    Quantity operator*(double scalar, Quantity value) {                                        \
        return Quantity::fromRawSi(scalar * value.rawSi());                                    \
    }                                                                                           \
    Quantity operator/(Quantity value, double scalar) {                                        \
        return Quantity::fromRawSi(value.rawSi() / scalar);                                    \
    }                                                                                           \
    double operator/(Quantity left, Quantity right) {                                          \
        return left.rawSi() / right.rawSi();                                                   \
    }

UNITS_DEFINE_SAME_QUANTITY_OPERATORS(Length)
UNITS_DEFINE_SAME_QUANTITY_OPERATORS(Time)
UNITS_DEFINE_SAME_QUANTITY_OPERATORS(Mass)
UNITS_DEFINE_SAME_QUANTITY_OPERATORS(Velocity)
UNITS_DEFINE_SAME_QUANTITY_OPERATORS(Acceleration)
UNITS_DEFINE_SAME_QUANTITY_OPERATORS(Force)

#undef UNITS_DEFINE_SAME_QUANTITY_OPERATORS

Velocity operator/(Length distance, Time time) {
    return Velocity::metersPerSecond(distance.asMeters() / time.asSeconds());
}

Time operator/(Length distance, Velocity velocity) {
    return Time::seconds(distance.asMeters() / velocity.asMetersPerSecond());
}

Length operator*(Velocity velocity, Time time) {
    return Length::meters(velocity.asMetersPerSecond() * time.asSeconds());
}

Length operator*(Time time, Velocity velocity) {
    return Length::meters(time.asSeconds() * velocity.asMetersPerSecond());
}

Acceleration operator/(Velocity velocity, Time time) {
    return Acceleration::metersPerSecondSquared(velocity.asMetersPerSecond() / time.asSeconds());
}

Time operator/(Velocity velocity, Acceleration acceleration) {
    return Time::seconds(velocity.asMetersPerSecond() / acceleration.asMetersPerSecondSquared());
}

Velocity operator*(Acceleration acceleration, Time time) {
    return Velocity::metersPerSecond(acceleration.asMetersPerSecondSquared() * time.asSeconds());
}

Velocity operator*(Time time, Acceleration acceleration) {
    return Velocity::metersPerSecond(time.asSeconds() * acceleration.asMetersPerSecondSquared());
}

Force operator*(Mass mass, Acceleration acceleration) {
    return Force::newtons(mass.asKilograms() * acceleration.asMetersPerSecondSquared());
}

Force operator*(Acceleration acceleration, Mass mass) {
    return Force::newtons(acceleration.asMetersPerSecondSquared() * mass.asKilograms());
}

Acceleration operator/(Force force, Mass mass) {
    return Acceleration::metersPerSecondSquared(force.asNewtons() / mass.asKilograms());
}

Mass operator/(Force force, Acceleration acceleration) {
    return Mass::kilograms(force.asNewtons() / acceleration.asMetersPerSecondSquared());
}

std::optional<Velocity> Length::checkedDivTime(Time time) const {
    if (time.asSeconds() == 0.0) {
        return std::nullopt;
    }

    return *this / time;
}

std::optional<Time> Length::checkedDivVelocity(Velocity velocity) const {
    if (velocity.asMetersPerSecond() == 0.0) {
        return std::nullopt;
    }

    return *this / velocity;
}

std::optional<Acceleration> Velocity::checkedDivTime(Time time) const {
    if (time.asSeconds() == 0.0) {
        return std::nullopt;
    }

    return *this / time;
}

std::optional<Time> Velocity::checkedDivAcceleration(Acceleration acceleration) const {
    if (acceleration.asMetersPerSecondSquared() == 0.0) {
        return std::nullopt;
    }

    return *this / acceleration;
}

std::optional<Acceleration> Force::checkedDivMass(Mass mass) const {
    if (mass.asKilograms() == 0.0) {
        return std::nullopt;
    }

    return *this / mass;
}

std::optional<Mass> Force::checkedDivAcceleration(Acceleration acceleration) const {
    if (acceleration.asMetersPerSecondSquared() == 0.0) {
        return std::nullopt;
    }

    return *this / acceleration;
}

} // namespace units
