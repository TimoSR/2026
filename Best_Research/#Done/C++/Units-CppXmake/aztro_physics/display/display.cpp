module;
#include <iomanip>
#include <ostream>

export module aztro_physics:display;
import :quantity_display;
import :time;
import :mass;
import :acceleration;
import :velocity;
import :length;
import :force;

export {
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

    namespace aztro::physics
    {

        namespace detail
        {

            template <typename Quantity, typename Unit> std::ostream& write_display(std::ostream& stream, QuantityDisplay<Quantity, Unit> display)
            {
                auto flags = stream.flags();
                auto precision = stream.precision();

                if (display.precision().has_value())
                {
                    stream << std::fixed << std::setprecision(*display.precision());
                }

                stream << value_in_unit(display.value(), display.unit()) << ' ' << symbol(display.unit());

                stream.flags(flags);
                stream.precision(precision);
                return stream;
            }

        } // namespace detail

        std::ostream& operator<<(std::ostream& stream, QuantityDisplay<length::Length, length::LengthUnit> display)
        {
            return detail::write_display(stream, display);
        }

        std::ostream& operator<<(std::ostream& stream, QuantityDisplay<time::Time, time::TimeUnit> display)
        {
            return detail::write_display(stream, display);
        }

        std::ostream& operator<<(std::ostream& stream, QuantityDisplay<mass::Mass, mass::MassUnit> display)
        {
            return detail::write_display(stream, display);
        }

        std::ostream& operator<<(std::ostream& stream, QuantityDisplay<velocity::Velocity, velocity::VelocityUnit> display)
        {
            return detail::write_display(stream, display);
        }

        std::ostream& operator<<(std::ostream& stream, QuantityDisplay<acceleration::Acceleration, acceleration::AccelerationUnit> display)
        {
            return detail::write_display(stream, display);
        }

        std::ostream& operator<<(std::ostream& stream, QuantityDisplay<force::Force, force::ForceUnit> display)
        {
            return detail::write_display(stream, display);
        }

    } // namespace aztro::physics
}