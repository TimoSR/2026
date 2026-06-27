#include <cassert>
#include <cmath>
#include <format>
#include <string>
#include <type_traits>

#include "units/units.hpp"

namespace {

template <typename Left, typename Right>
concept CanDivide = requires(Left left, Right right) {
    left / right;
};

bool approximatelyEqual(double left, double right, double epsilon) {
    return std::fabs(left - right) <= epsilon;
}

void testUnitNormalization() {
    assert(units::length::meters(1) == units::length::centimeters(100));
    assert(units::length::centimeters(100) == units::length::millimeters(1'000));
    assert(units::time::seconds(1) == units::time::milliseconds(1'000));
    assert(units::time::seconds(1) == units::time::microseconds(1'000'000));
    assert(units::mass::kilogram(1) == units::mass::grams(1'000));
    assert(units::mass::kilogram(1) == units::mass::milligrams(1'000'000));
}

void testEquationRulesIgnoreInputScale() {
    using units::Acceleration;
    using units::Force;
    using units::Mass;
    using units::Velocity;

    const auto distance = units::length::centimeters(10'000);
    const auto time = units::time::milliseconds(9'580);
    const Mass mass = units::mass::grams(80'000);

    const Velocity velocity = distance / time;
    const Acceleration acceleration = velocity / time;
    const Force force = mass * acceleration;

    assert(approximatelyEqual(velocity.asMetersPerSecond(), 100.0 / 9.58, 1e-12));
    assert(approximatelyEqual(acceleration.asMetersPerSecondSquared(), (100.0 / 9.58) / 9.58, 1e-12));
    assert(approximatelyEqual(force.asNewtons(), 80.0 * ((100.0 / 9.58) / 9.58), 1e-12));
}

void testUnsupportedOperationsAreOmitted() {
    static_assert(!CanDivide<units::Velocity, units::Mass>);
}

void testCheckedDivision() {
    const auto zeroTime = units::time::seconds(0);
    const auto distance = units::length::meters(10);
    const auto force = units::force::newtons(10);
    const auto zeroMass = units::mass::kilograms(0);

    assert(!distance.checkedDivTime(zeroTime).has_value());
    assert(!force.checkedDivMass(zeroMass).has_value());
    assert(distance.checkedDivTime(units::time::seconds(2)).has_value());
}

void testStdFormatSupport() {
    const auto distance = units::length::centimeters(10'000);
    const auto elapsed = units::time::milliseconds(9'580);
    const auto velocity = distance / elapsed;

    assert(std::format("{}", distance.displayAs(units::LengthUnit::Meters)) == "100 m");
    assert(std::format("{}", velocity.displayAsPrecision(units::VelocityUnit::KilometersPerHour, 2))
           == "37.58 km/h");
    assert(std::format("{}", velocity).find("m/s") != std::string::npos);
}

} // namespace

int main() {
    testUnitNormalization();
    testEquationRulesIgnoreInputScale();
    testUnsupportedOperationsAreOmitted();
    testCheckedDivision();
    testStdFormatSupport();
    return 0;
}
