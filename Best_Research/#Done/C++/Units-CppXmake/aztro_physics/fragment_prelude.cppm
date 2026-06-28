#pragma once

#include <format>
#include <iosfwd>
#include <optional>

namespace aztro::physics
{

    template <typename Quantity, typename Unit> class QuantityDisplay;

} // namespace aztro::physics

namespace aztro::physics::acceleration
{

    enum class AccelerationUnit;
    class Acceleration;

} // namespace aztro::physics::acceleration

namespace aztro::physics::force
{

    enum class ForceUnit;
    class Force;

} // namespace aztro::physics::force

namespace aztro::physics::length
{

    enum class LengthUnit;
    class Length;

} // namespace aztro::physics::length

namespace aztro::physics::mass
{

    enum class MassUnit;
    class Mass;

} // namespace aztro::physics::mass

namespace aztro::physics::time
{

    enum class TimeUnit;
    class Time;

} // namespace aztro::physics::time

namespace aztro::physics::velocity
{

    enum class VelocityUnit;
    class Velocity;

} // namespace aztro::physics::velocity
