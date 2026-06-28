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
