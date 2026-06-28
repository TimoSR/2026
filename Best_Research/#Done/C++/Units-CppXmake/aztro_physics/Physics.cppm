module;

#include <format>
#include <iosfwd>
#include <optional>

export module aztro_physics;

export {
// clang-format off
#include "detail/math.hpp"
#include "detail/quantity_display.hpp"
#include "time/time.hpp"
#include "mass/mass.hpp"
#include "acceleration/acceleration.hpp"
#include "velocity/velocity.hpp"
#include "length/length.hpp"
#include "force/force.hpp"
#include "display/display.hpp"
#include "operations/operations.hpp"
#include "display/format.hpp"
// clang-format on
}
