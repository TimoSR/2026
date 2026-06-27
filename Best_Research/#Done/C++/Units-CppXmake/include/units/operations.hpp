#pragma once

#include <optional>

#include "units/acceleration.hpp"
#include "units/force.hpp"
#include "units/length.hpp"
#include "units/mass.hpp"
#include "units/time.hpp"
#include "units/velocity.hpp"

namespace units {

[[nodiscard]] Length operator+(Length left, Length right);
[[nodiscard]] Length operator-(Length left, Length right);
[[nodiscard]] Length operator-(Length value);
[[nodiscard]] Length operator*(Length value, double scalar);
[[nodiscard]] Length operator*(double scalar, Length value);
[[nodiscard]] Length operator/(Length value, double scalar);
[[nodiscard]] double operator/(Length left, Length right);

[[nodiscard]] Time operator+(Time left, Time right);
[[nodiscard]] Time operator-(Time left, Time right);
[[nodiscard]] Time operator-(Time value);
[[nodiscard]] Time operator*(Time value, double scalar);
[[nodiscard]] Time operator*(double scalar, Time value);
[[nodiscard]] Time operator/(Time value, double scalar);
[[nodiscard]] double operator/(Time left, Time right);

[[nodiscard]] Mass operator+(Mass left, Mass right);
[[nodiscard]] Mass operator-(Mass left, Mass right);
[[nodiscard]] Mass operator-(Mass value);
[[nodiscard]] Mass operator*(Mass value, double scalar);
[[nodiscard]] Mass operator*(double scalar, Mass value);
[[nodiscard]] Mass operator/(Mass value, double scalar);
[[nodiscard]] double operator/(Mass left, Mass right);

[[nodiscard]] Velocity operator+(Velocity left, Velocity right);
[[nodiscard]] Velocity operator-(Velocity left, Velocity right);
[[nodiscard]] Velocity operator-(Velocity value);
[[nodiscard]] Velocity operator*(Velocity value, double scalar);
[[nodiscard]] Velocity operator*(double scalar, Velocity value);
[[nodiscard]] Velocity operator/(Velocity value, double scalar);
[[nodiscard]] double operator/(Velocity left, Velocity right);

[[nodiscard]] Acceleration operator+(Acceleration left, Acceleration right);
[[nodiscard]] Acceleration operator-(Acceleration left, Acceleration right);
[[nodiscard]] Acceleration operator-(Acceleration value);
[[nodiscard]] Acceleration operator*(Acceleration value, double scalar);
[[nodiscard]] Acceleration operator*(double scalar, Acceleration value);
[[nodiscard]] Acceleration operator/(Acceleration value, double scalar);
[[nodiscard]] double operator/(Acceleration left, Acceleration right);

[[nodiscard]] Force operator+(Force left, Force right);
[[nodiscard]] Force operator-(Force left, Force right);
[[nodiscard]] Force operator-(Force value);
[[nodiscard]] Force operator*(Force value, double scalar);
[[nodiscard]] Force operator*(double scalar, Force value);
[[nodiscard]] Force operator/(Force value, double scalar);
[[nodiscard]] double operator/(Force left, Force right);

[[nodiscard]] Velocity operator/(Length distance, Time time);
[[nodiscard]] Time operator/(Length distance, Velocity velocity);
[[nodiscard]] Length operator*(Velocity velocity, Time time);
[[nodiscard]] Length operator*(Time time, Velocity velocity);

[[nodiscard]] Acceleration operator/(Velocity velocity, Time time);
[[nodiscard]] Time operator/(Velocity velocity, Acceleration acceleration);
[[nodiscard]] Velocity operator*(Acceleration acceleration, Time time);
[[nodiscard]] Velocity operator*(Time time, Acceleration acceleration);

[[nodiscard]] Force operator*(Mass mass, Acceleration acceleration);
[[nodiscard]] Force operator*(Acceleration acceleration, Mass mass);
[[nodiscard]] Acceleration operator/(Force force, Mass mass);
[[nodiscard]] Mass operator/(Force force, Acceleration acceleration);

} // namespace units
