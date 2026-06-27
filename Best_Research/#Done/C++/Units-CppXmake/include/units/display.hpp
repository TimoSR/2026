#pragma once

#include <iosfwd>

#include "units/acceleration.hpp"
#include "units/detail/quantity_display.hpp"
#include "units/force.hpp"
#include "units/length.hpp"
#include "units/mass.hpp"
#include "units/time.hpp"
#include "units/velocity.hpp"

namespace units {

[[nodiscard]] const char* symbol(LengthUnit unit);
[[nodiscard]] const char* symbol(TimeUnit unit);
[[nodiscard]] const char* symbol(MassUnit unit);
[[nodiscard]] const char* symbol(VelocityUnit unit);
[[nodiscard]] const char* symbol(AccelerationUnit unit);
[[nodiscard]] const char* symbol(ForceUnit unit);

[[nodiscard]] double valueInUnit(Length value, LengthUnit unit);
[[nodiscard]] double valueInUnit(Time value, TimeUnit unit);
[[nodiscard]] double valueInUnit(Mass value, MassUnit unit);
[[nodiscard]] double valueInUnit(Velocity value, VelocityUnit unit);
[[nodiscard]] double valueInUnit(Acceleration value, AccelerationUnit unit);
[[nodiscard]] double valueInUnit(Force value, ForceUnit unit);

std::ostream& operator<<(std::ostream& stream, QuantityDisplay<Length, LengthUnit> display);
std::ostream& operator<<(std::ostream& stream, QuantityDisplay<Time, TimeUnit> display);
std::ostream& operator<<(std::ostream& stream, QuantityDisplay<Mass, MassUnit> display);
std::ostream& operator<<(std::ostream& stream, QuantityDisplay<Velocity, VelocityUnit> display);
std::ostream& operator<<(
    std::ostream& stream,
    QuantityDisplay<Acceleration, AccelerationUnit> display
);
std::ostream& operator<<(std::ostream& stream, QuantityDisplay<Force, ForceUnit> display);

std::ostream& operator<<(std::ostream& stream, Length value);
std::ostream& operator<<(std::ostream& stream, Time value);
std::ostream& operator<<(std::ostream& stream, Mass value);
std::ostream& operator<<(std::ostream& stream, Velocity value);
std::ostream& operator<<(std::ostream& stream, Acceleration value);
std::ostream& operator<<(std::ostream& stream, Force value);

} // namespace units
