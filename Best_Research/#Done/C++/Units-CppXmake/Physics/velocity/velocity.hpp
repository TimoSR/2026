#pragma once

#include <optional>

#include "physics/acceleration/acceleration.hpp"
#include "physics/detail/quantity_display.hpp"
#include "physics/time/time.hpp"

namespace physics::velocity
{

    enum class VelocityUnit
    {
        MetersPerSecond,
        KilometersPerHour,
    };

    class Velocity
    {
            double _meters_per_second;

        public:
            static Velocity from_raw_si(double meters_per_second);

            static Velocity meters_per_second(double value);

            static std::optional<Velocity> try_meters_per_second(double value);

            static Velocity kilometers_per_hour(double value);

            static std::optional<Velocity> try_kilometers_per_hour(double value);

            double raw_si();

            double to_meters_per_second();

            double to_kilometers_per_hour();

            bool approximately_equals(Velocity other, double epsilon);

            QuantityDisplay<Velocity, VelocityUnit> display_as(VelocityUnit unit);

            QuantityDisplay<Velocity, VelocityUnit> display_as_precision(VelocityUnit unit, int precision);

            QuantityDisplay<Velocity, VelocityUnit> display_meters_per_second();
            QuantityDisplay<Velocity, VelocityUnit> display_kilometers_per_hour();

            QuantityDisplay<Velocity, VelocityUnit> display_meters_per_second_precision(int precision);
            QuantityDisplay<Velocity, VelocityUnit> display_kilometers_per_hour_precision(int precision);

            std::optional<acceleration::Acceleration> checked_div_time(time::Time time);
            std::optional<time::Time> checked_div_acceleration(acceleration::Acceleration acceleration);

            friend bool operator==(Velocity left, Velocity right);

        private:
            explicit Velocity(double meters_per_second);
    };

    Velocity meters_per_second(double value);
    std::optional<Velocity> try_meters_per_second(double value);
    Velocity meters_pr_second(double value);
    Velocity kilometers_per_hour(double value);
    std::optional<Velocity> try_kilometers_per_hour(double value);

} // namespace physics::velocity
