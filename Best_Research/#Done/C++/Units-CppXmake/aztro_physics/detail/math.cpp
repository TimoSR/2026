module;

#include <format>
#include <iomanip>
#include <limits>
#include <optional>
#include <ostream>

module aztro_physics;

namespace aztro::physics::detail
{

    double absolute(double value)
    {
        return value < 0.0 ? -value : value;
    }

} // namespace aztro::physics::detail
