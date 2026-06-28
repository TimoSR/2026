module;

#include <format>
#include <iosfwd>
#include <optional>

export module aztro_physics;

export {
    // clang-format off
#include "detail/math.cppm"
#include "detail/quantity_display.cppm"
#include "time/time.cppm"
#include "mass/mass.cppm"
#include "acceleration/acceleration.cppm"
#include "velocity/velocity.cppm"
#include "length/length.cppm"
#include "force/force.cppm"
#include "display/display.cppm"
#include "operations/operations.cppm"
#include "display/format.cppm"
    // clang-format on
}
