#pragma once

#include <iosfwd>

#include "units/detail/quantity_display.hpp"
#include "units/forward.hpp"

namespace units {

const char* symbol(LengthUnit unit);
const char* symbol(TimeUnit unit);
const char* symbol(MassUnit unit);
const char* symbol(VelocityUnit unit);
const char* symbol(AccelerationUnit unit);
const char* symbol(ForceUnit unit);

double valueInUnit(Length value, LengthUnit unit);
double valueInUnit(Time value, TimeUnit unit);
double valueInUnit(Mass value, MassUnit unit);
double valueInUnit(Velocity value, VelocityUnit unit);
double valueInUnit(Acceleration value, AccelerationUnit unit);
double valueInUnit(Force value, ForceUnit unit);

std::ostream& operator<<(std::ostream& stream, QuantityDisplay<Length, LengthUnit> display);
std::ostream& operator<<(std::ostream& stream, QuantityDisplay<Time, TimeUnit> display);
std::ostream& operator<<(std::ostream& stream, QuantityDisplay<Mass, MassUnit> display);
std::ostream& operator<<(std::ostream& stream, QuantityDisplay<Velocity, VelocityUnit> display);
std::ostream& operator<<(std::ostream& stream, QuantityDisplay<Acceleration, AccelerationUnit> display);
std::ostream& operator<<(std::ostream& stream, QuantityDisplay<Force, ForceUnit> display);

std::ostream& operator<<(std::ostream& stream, Length value);
std::ostream& operator<<(std::ostream& stream, Time value);
std::ostream& operator<<(std::ostream& stream, Mass value);
std::ostream& operator<<(std::ostream& stream, Velocity value);
std::ostream& operator<<(std::ostream& stream, Acceleration value);
std::ostream& operator<<(std::ostream& stream, Force value);

} // namespace units
