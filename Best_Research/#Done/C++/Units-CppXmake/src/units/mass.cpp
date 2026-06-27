#include "units/mass.hpp"

#include "units/detail/math.hpp"

namespace units {

Mass::Mass(double kilograms) : kilograms_(kilograms) {
}

Mass Mass::fromRawSi(double kilograms) {
    return Mass(kilograms);
}

Mass Mass::kilograms(double value) {
    return Mass(value);
}

Mass Mass::kilogram(double value) {
    return kilograms(value);
}

Mass Mass::grams(double value) {
    return Mass(value / 1'000.0);
}

Mass Mass::milligrams(double value) {
    return Mass(value / 1'000'000.0);
}

Mass Mass::micrograms(double value) {
    return Mass(value / 1'000'000'000.0);
}

Mass Mass::tons(double value) {
    return Mass(value * 1'000.0);
}

double Mass::rawSi() const {
    return kilograms_;
}

double Mass::asKilograms() const {
    return kilograms_;
}

double Mass::asGrams() const {
    return kilograms_ * 1'000.0;
}

double Mass::asMilligrams() const {
    return kilograms_ * 1'000'000.0;
}

double Mass::asMicrograms() const {
    return kilograms_ * 1'000'000'000.0;
}

double Mass::asTons() const {
    return kilograms_ / 1'000.0;
}

bool Mass::approximatelyEquals(Mass other, double epsilon) const {
    return detail::absolute(kilograms_ - other.kilograms_) <= epsilon;
}

QuantityDisplay<Mass, MassUnit> Mass::displayAs(MassUnit unit) const {
    return QuantityDisplay<Mass, MassUnit>(*this, unit);
}

QuantityDisplay<Mass, MassUnit> Mass::displayAsPrecision(MassUnit unit, int precision) const {
    return QuantityDisplay<Mass, MassUnit>(*this, unit, precision);
}

} // namespace units
