module;

#include <format>
#include <iosfwd>
#include <optional>

export module aztro_physics;

export {

    // From Physics/detail/quantity_display.hpp

    namespace aztro::physics
    {

        template <typename Quantity, typename Unit> class QuantityDisplay
        {
            public:
                QuantityDisplay(Quantity value, Unit unit, std::optional<int> precision = std::nullopt);

                Quantity value();

                Unit unit();

                std::optional<int> precision();

            private:
                Quantity value_;
                Unit unit_;
                std::optional<int> precision_;
        };

    } // namespace aztro::physics

    // From Physics/detail/math.hpp

    namespace aztro::physics::detail
    {

        double absolute(double value);

    } // namespace aztro::physics::detail

    // From Physics/acceleration/acceleration.hpp

    namespace aztro::physics::acceleration
    {

        enum class AccelerationUnit
        {
            MetersPerSecondSquared,
            StandardGravity,
        };

        class Acceleration
        {

                double _meters_per_second_squared;

            public:
                static double standard_gravity_meters_per_second_squared();

                static Acceleration from_raw_si(double meters_per_second_squared);

                static Acceleration meters_per_second_squared(double value);

                static std::optional<Acceleration> try_meters_per_second_squared(double value);

                static Acceleration standard_gravity(double value);

                static std::optional<Acceleration> try_standard_gravity(double value);

                double raw_si();

                double to_meters_per_second_squared();

                double to_standard_gravity();

                bool approximately_equals(Acceleration other, double epsilon);

                QuantityDisplay<Acceleration, AccelerationUnit> display_as(AccelerationUnit unit);

                QuantityDisplay<Acceleration, AccelerationUnit> display_as_precision(AccelerationUnit unit, int precision);

                QuantityDisplay<Acceleration, AccelerationUnit> display_meters_per_second_squared();
                QuantityDisplay<Acceleration, AccelerationUnit> display_standard_gravity();

                QuantityDisplay<Acceleration, AccelerationUnit> display_meters_per_second_squared_precision(int precision);
                QuantityDisplay<Acceleration, AccelerationUnit> display_standard_gravity_precision(int precision);

                friend bool operator==(Acceleration left, Acceleration right);

            private:
                explicit Acceleration(double meters_per_second_squared);
        };

        Acceleration meters_per_second_squared(double value);
        std::optional<Acceleration> try_meters_per_second_squared(double value);
        Acceleration meters_pr_second_pr_second(double value);
        Acceleration standard_gravity(double value);
        std::optional<Acceleration> try_standard_gravity(double value);

    } // namespace aztro::physics::acceleration

    // From Physics/time/time.hpp

    namespace aztro::physics::time
    {

        enum class TimeUnit
        {
            Seconds,
            Milliseconds,
            Microseconds,
            Nanoseconds,
            Minutes,
            Hours,
        };

        class Time
        {
                double _seconds;

            public:
                static Time from_raw_si(double seconds);

                static Time seconds(double value);

                static std::optional<Time> try_seconds(double value);

                static Time milliseconds(double value);

                static std::optional<Time> try_milliseconds(double value);

                static Time microseconds(double value);

                static std::optional<Time> try_microseconds(double value);

                static Time nanoseconds(double value);

                static std::optional<Time> try_nanoseconds(double value);

                static Time minutes(double value);

                static std::optional<Time> try_minutes(double value);

                static Time hours(double value);

                static std::optional<Time> try_hours(double value);

                double raw_si();

                double to_seconds();

                double to_milliseconds();

                double to_microseconds();

                double to_nanoseconds();

                double to_minutes();

                double to_hours();

                bool approximately_equals(Time other, double epsilon);

                QuantityDisplay<Time, TimeUnit> display_as(TimeUnit unit);

                QuantityDisplay<Time, TimeUnit> display_as_precision(TimeUnit unit, int precision);

                QuantityDisplay<Time, TimeUnit> display_seconds();
                QuantityDisplay<Time, TimeUnit> display_milliseconds();
                QuantityDisplay<Time, TimeUnit> display_microseconds();
                QuantityDisplay<Time, TimeUnit> display_nanoseconds();
                QuantityDisplay<Time, TimeUnit> display_minutes();
                QuantityDisplay<Time, TimeUnit> display_hours();

                QuantityDisplay<Time, TimeUnit> display_seconds_precision(int precision);
                QuantityDisplay<Time, TimeUnit> display_milliseconds_precision(int precision);
                QuantityDisplay<Time, TimeUnit> display_microseconds_precision(int precision);
                QuantityDisplay<Time, TimeUnit> display_nanoseconds_precision(int precision);
                QuantityDisplay<Time, TimeUnit> display_minutes_precision(int precision);
                QuantityDisplay<Time, TimeUnit> display_hours_precision(int precision);

                friend bool operator==(Time left, Time right);

            private:
                explicit Time(double seconds);
        };

        Time seconds(double value);
        std::optional<Time> try_seconds(double value);
        Time milliseconds(double value);
        std::optional<Time> try_milliseconds(double value);
        Time microseconds(double value);
        std::optional<Time> try_microseconds(double value);
        Time nanoseconds(double value);
        std::optional<Time> try_nanoseconds(double value);
        Time minutes(double value);
        std::optional<Time> try_minutes(double value);
        Time hours(double value);
        std::optional<Time> try_hours(double value);

    } // namespace aztro::physics::time

    // From Physics/mass/mass.hpp

    namespace aztro::physics::mass
    {

        enum class MassUnit
        {
            Kilograms,
            Grams,
            Milligrams,
            Micrograms,
            Tons,
        };

        class Mass
        {
                double _kilograms;

            public:
                static Mass from_raw_si(double kilograms);

                static Mass kilograms(double value);

                static std::optional<Mass> try_kilograms(double value);

                static Mass kilogram(double value);

                static std::optional<Mass> try_kilogram(double value);

                static Mass grams(double value);

                static std::optional<Mass> try_grams(double value);

                static Mass milligrams(double value);

                static std::optional<Mass> try_milligrams(double value);

                static Mass micrograms(double value);

                static std::optional<Mass> try_micrograms(double value);

                static Mass tons(double value);

                static std::optional<Mass> try_tons(double value);

                double raw_si();

                double to_kilograms();

                double to_grams();

                double to_milligrams();

                double to_micrograms();

                double to_tons();

                bool approximately_equals(Mass other, double epsilon);

                QuantityDisplay<Mass, MassUnit> display_as(MassUnit unit);

                QuantityDisplay<Mass, MassUnit> display_as_precision(MassUnit unit, int precision);

                QuantityDisplay<Mass, MassUnit> display_kilograms();
                QuantityDisplay<Mass, MassUnit> display_grams();
                QuantityDisplay<Mass, MassUnit> display_milligrams();
                QuantityDisplay<Mass, MassUnit> display_micrograms();
                QuantityDisplay<Mass, MassUnit> display_tons();

                QuantityDisplay<Mass, MassUnit> display_kilograms_precision(int precision);
                QuantityDisplay<Mass, MassUnit> display_grams_precision(int precision);
                QuantityDisplay<Mass, MassUnit> display_milligrams_precision(int precision);
                QuantityDisplay<Mass, MassUnit> display_micrograms_precision(int precision);
                QuantityDisplay<Mass, MassUnit> display_tons_precision(int precision);

                friend bool operator==(Mass left, Mass right);

            private:
                explicit Mass(double kilograms);
        };

        Mass kilograms(double value);
        std::optional<Mass> try_kilograms(double value);
        Mass kilogram(double value);
        std::optional<Mass> try_kilogram(double value);
        Mass grams(double value);
        std::optional<Mass> try_grams(double value);
        Mass milligrams(double value);
        std::optional<Mass> try_milligrams(double value);
        Mass micrograms(double value);
        std::optional<Mass> try_micrograms(double value);
        Mass tons(double value);
        std::optional<Mass> try_tons(double value);

    } // namespace aztro::physics::mass

    // From Physics/velocity/velocity.hpp

    namespace aztro::physics::velocity
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

    } // namespace aztro::physics::velocity

    // From Physics/length/length.hpp

    namespace aztro::physics::length
    {

        enum class LengthUnit
        {
            Meters,
            Kilometers,
            Centimeters,
            Millimeters,
            Micrometers,
            Nanometers,
        };

        class Length
        {
                double _meters;

            public:
                static Length from_raw_si(double meters);

                static Length meters(double value);

                static std::optional<Length> try_meters(double value);

                static Length kilometers(double value);

                static std::optional<Length> try_kilometers(double value);

                static Length centimeters(double value);

                static std::optional<Length> try_centimeters(double value);

                static Length millimeters(double value);

                static std::optional<Length> try_millimeters(double value);

                static Length micrometers(double value);

                static std::optional<Length> try_micrometers(double value);

                static Length nanometers(double value);

                static std::optional<Length> try_nanometers(double value);

                double raw_si();

                double to_meters();

                double to_kilometers();

                double to_centimeters();

                double to_millimeters();

                double to_micrometers();

                double to_nanometers();

                bool approximately_equals(Length other, double epsilon);

                QuantityDisplay<Length, LengthUnit> display_as(LengthUnit unit);

                QuantityDisplay<Length, LengthUnit> display_as_precision(LengthUnit unit, int precision);

                QuantityDisplay<Length, LengthUnit> display_meters();
                QuantityDisplay<Length, LengthUnit> display_kilometers();
                QuantityDisplay<Length, LengthUnit> display_centimeters();
                QuantityDisplay<Length, LengthUnit> display_millimeters();
                QuantityDisplay<Length, LengthUnit> display_micrometers();
                QuantityDisplay<Length, LengthUnit> display_nanometers();

                QuantityDisplay<Length, LengthUnit> display_meters_precision(int precision);
                QuantityDisplay<Length, LengthUnit> display_kilometers_precision(int precision);
                QuantityDisplay<Length, LengthUnit> display_centimeters_precision(int precision);
                QuantityDisplay<Length, LengthUnit> display_millimeters_precision(int precision);
                QuantityDisplay<Length, LengthUnit> display_micrometers_precision(int precision);
                QuantityDisplay<Length, LengthUnit> display_nanometers_precision(int precision);

                std::optional<velocity::Velocity> checked_div_time(time::Time time);
                std::optional<time::Time> checked_div_velocity(velocity::Velocity velocity);

                friend bool operator==(Length left, Length right);

            private:
                explicit Length(double meters);
        };

        Length meters(double value);
        std::optional<Length> try_meters(double value);
        Length kilometers(double value);
        std::optional<Length> try_kilometers(double value);
        Length centimeters(double value);
        std::optional<Length> try_centimeters(double value);
        Length millimeters(double value);
        std::optional<Length> try_millimeters(double value);
        Length micrometers(double value);
        std::optional<Length> try_micrometers(double value);
        Length nanometers(double value);
        std::optional<Length> try_nanometers(double value);

    } // namespace aztro::physics::length

    // From Physics/force/force.hpp

    namespace aztro::physics::force
    {

        enum class ForceUnit
        {
            Newtons,
            Millinewtons,
            Kilonewtons,
        };

        class Force
        {
                double _newtons;

            public:
                static Force from_raw_si(double newtons);

                static Force newtons(double value);

                static std::optional<Force> try_newtons(double value);

                static Force millinewtons(double value);

                static std::optional<Force> try_millinewtons(double value);

                static Force kilonewtons(double value);

                static std::optional<Force> try_kilonewtons(double value);

                double raw_si();

                double to_newtons();

                double to_millinewtons();

                double to_kilonewtons();

                bool approximately_equals(Force other, double epsilon);

                QuantityDisplay<Force, ForceUnit> display_as(ForceUnit unit);

                QuantityDisplay<Force, ForceUnit> display_as_precision(ForceUnit unit, int precision);

                QuantityDisplay<Force, ForceUnit> display_newtons();
                QuantityDisplay<Force, ForceUnit> display_millinewtons();
                QuantityDisplay<Force, ForceUnit> display_kilonewtons();

                QuantityDisplay<Force, ForceUnit> display_newtons_precision(int precision);
                QuantityDisplay<Force, ForceUnit> display_millinewtons_precision(int precision);
                QuantityDisplay<Force, ForceUnit> display_kilonewtons_precision(int precision);

                std::optional<acceleration::Acceleration> checked_div_mass(mass::Mass mass);
                std::optional<mass::Mass> checked_div_acceleration(acceleration::Acceleration acceleration);

                friend bool operator==(Force left, Force right);

            private:
                explicit Force(double newtons);
        };

        Force newtons(double value);
        std::optional<Force> try_newtons(double value);
        Force force(double value);
        Force millinewtons(double value);
        std::optional<Force> try_millinewtons(double value);
        Force kilonewtons(double value);
        std::optional<Force> try_kilonewtons(double value);

    } // namespace aztro::physics::force

    // From Physics/display/display.hpp

    namespace aztro::physics::length
    {

        const char* symbol(LengthUnit unit);
        double value_in_unit(Length value, LengthUnit unit);
        std::ostream& operator<<(std::ostream& stream, Length value);

    } // namespace aztro::physics::length

    namespace aztro::physics::time
    {

        const char* symbol(TimeUnit unit);
        double value_in_unit(Time value, TimeUnit unit);
        std::ostream& operator<<(std::ostream& stream, Time value);

    } // namespace aztro::physics::time

    namespace aztro::physics::mass
    {

        const char* symbol(MassUnit unit);
        double value_in_unit(Mass value, MassUnit unit);
        std::ostream& operator<<(std::ostream& stream, Mass value);

    } // namespace aztro::physics::mass

    namespace aztro::physics::velocity
    {

        const char* symbol(VelocityUnit unit);
        double value_in_unit(Velocity value, VelocityUnit unit);
        std::ostream& operator<<(std::ostream& stream, Velocity value);

    } // namespace aztro::physics::velocity

    namespace aztro::physics::acceleration
    {

        const char* symbol(AccelerationUnit unit);
        double value_in_unit(Acceleration value, AccelerationUnit unit);
        std::ostream& operator<<(std::ostream& stream, Acceleration value);

    } // namespace aztro::physics::acceleration

    namespace aztro::physics::force
    {

        const char* symbol(ForceUnit unit);
        double value_in_unit(Force value, ForceUnit unit);
        std::ostream& operator<<(std::ostream& stream, Force value);

    } // namespace aztro::physics::force

    namespace aztro::physics
    {

        std::ostream& operator<<(std::ostream& stream, QuantityDisplay<length::Length, length::LengthUnit> display);
        std::ostream& operator<<(std::ostream& stream, QuantityDisplay<time::Time, time::TimeUnit> display);
        std::ostream& operator<<(std::ostream& stream, QuantityDisplay<mass::Mass, mass::MassUnit> display);
        std::ostream& operator<<(std::ostream& stream, QuantityDisplay<velocity::Velocity, velocity::VelocityUnit> display);
        std::ostream& operator<<(std::ostream& stream, QuantityDisplay<acceleration::Acceleration, acceleration::AccelerationUnit> display);
        std::ostream& operator<<(std::ostream& stream, QuantityDisplay<force::Force, force::ForceUnit> display);

    } // namespace aztro::physics

    // From Physics/operations/operations.hpp

    namespace aztro::physics::length
    {

        Length operator+(Length left, Length right);
        Length operator-(Length left, Length right);
        Length operator-(Length value);
        Length operator*(Length value, double scalar);
        Length operator*(double scalar, Length value);
        Length operator/(Length value, double scalar);
        double operator/(Length left, Length right);

        velocity::Velocity operator/(Length distance, time::Time time);
        time::Time operator/(Length distance, velocity::Velocity velocity);

    } // namespace aztro::physics::length

    namespace aztro::physics::time
    {

        Time operator+(Time left, Time right);
        Time operator-(Time left, Time right);
        Time operator-(Time value);
        Time operator*(Time value, double scalar);
        Time operator*(double scalar, Time value);
        Time operator/(Time value, double scalar);
        double operator/(Time left, Time right);

        length::Length operator*(Time time, velocity::Velocity velocity);
        velocity::Velocity operator*(Time time, acceleration::Acceleration acceleration);

    } // namespace aztro::physics::time

    namespace aztro::physics::mass
    {

        Mass operator+(Mass left, Mass right);
        Mass operator-(Mass left, Mass right);
        Mass operator-(Mass value);
        Mass operator*(Mass value, double scalar);
        Mass operator*(double scalar, Mass value);
        Mass operator/(Mass value, double scalar);
        double operator/(Mass left, Mass right);

        force::Force operator*(Mass mass, acceleration::Acceleration acceleration);

    } // namespace aztro::physics::mass

    namespace aztro::physics::velocity
    {

        Velocity operator+(Velocity left, Velocity right);
        Velocity operator-(Velocity left, Velocity right);
        Velocity operator-(Velocity value);
        Velocity operator*(Velocity value, double scalar);
        Velocity operator*(double scalar, Velocity value);
        Velocity operator/(Velocity value, double scalar);
        double operator/(Velocity left, Velocity right);

        Velocity calculate(length::Length distance, time::Time time);
        std::optional<Velocity> checked_calculate(length::Length distance, time::Time time);

        length::Length operator*(Velocity velocity, time::Time time);
        acceleration::Acceleration operator/(Velocity velocity, time::Time time);
        time::Time operator/(Velocity velocity, acceleration::Acceleration acceleration);

    } // namespace aztro::physics::velocity

    namespace aztro::physics::acceleration
    {

        Acceleration operator+(Acceleration left, Acceleration right);
        Acceleration operator-(Acceleration left, Acceleration right);
        Acceleration operator-(Acceleration value);
        Acceleration operator*(Acceleration value, double scalar);
        Acceleration operator*(double scalar, Acceleration value);
        Acceleration operator/(Acceleration value, double scalar);
        double operator/(Acceleration left, Acceleration right);

        Acceleration calculate(velocity::Velocity velocity, time::Time time);
        std::optional<Acceleration> checked_calculate(velocity::Velocity velocity, time::Time time);

        velocity::Velocity operator*(Acceleration acceleration, time::Time time);
        force::Force operator*(Acceleration acceleration, mass::Mass mass);

    } // namespace aztro::physics::acceleration

    namespace aztro::physics::force
    {

        Force operator+(Force left, Force right);
        Force operator-(Force left, Force right);
        Force operator-(Force value);
        Force operator*(Force value, double scalar);
        Force operator*(double scalar, Force value);
        Force operator/(Force value, double scalar);
        double operator/(Force left, Force right);

        Force calculate(mass::Mass mass, acceleration::Acceleration acceleration);

        acceleration::Acceleration operator/(Force force, mass::Mass mass);
        mass::Mass operator/(Force force, acceleration::Acceleration acceleration);

    } // namespace aztro::physics::force

    // From Physics/display/format.hpp

    template <typename Quantity, typename Unit> struct std::formatter<aztro::physics::QuantityDisplay<Quantity, Unit>, char>
    {
            constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
            {
                return context.begin();
            }

            std::format_context::iterator format(aztro::physics::QuantityDisplay<Quantity, Unit> display, std::format_context& context) const;
    };

    template <> struct std::formatter<aztro::physics::length::Length, char>
    {
            constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
            {
                return context.begin();
            }

            std::format_context::iterator format(aztro::physics::length::Length value, std::format_context& context) const;
    };

    template <> struct std::formatter<aztro::physics::time::Time, char>
    {
            constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
            {
                return context.begin();
            }

            std::format_context::iterator format(aztro::physics::time::Time value, std::format_context& context) const;
    };

    template <> struct std::formatter<aztro::physics::mass::Mass, char>
    {
            constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
            {
                return context.begin();
            }

            std::format_context::iterator format(aztro::physics::mass::Mass value, std::format_context& context) const;
    };

    template <> struct std::formatter<aztro::physics::velocity::Velocity, char>
    {
            constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
            {
                return context.begin();
            }

            std::format_context::iterator format(aztro::physics::velocity::Velocity value, std::format_context& context) const;
    };

    template <> struct std::formatter<aztro::physics::acceleration::Acceleration, char>
    {
            constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
            {
                return context.begin();
            }

            std::format_context::iterator format(aztro::physics::acceleration::Acceleration value, std::format_context& context) const;
    };

    template <> struct std::formatter<aztro::physics::force::Force, char>
    {
            constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
            {
                return context.begin();
            }

            std::format_context::iterator format(aztro::physics::force::Force value, std::format_context& context) const;
    };
}
