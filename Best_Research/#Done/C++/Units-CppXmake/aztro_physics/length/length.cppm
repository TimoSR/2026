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
