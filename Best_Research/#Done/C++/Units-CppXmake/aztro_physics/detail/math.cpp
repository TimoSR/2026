module;

export module aztro_physics:detail_math;

namespace aztro::physics::detail
{

    export double absolute(double value);

} // namespace aztro::physics::detail

namespace aztro::physics::detail
{

    double absolute(double value)
    {
        return value < 0.0 ? -value : value;
    }

} // namespace aztro::physics::detail
