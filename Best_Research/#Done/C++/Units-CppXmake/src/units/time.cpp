#include "units/time.hpp"

#include "units/detail/math.hpp"

namespace units {

Time::Time(double seconds) : seconds_(seconds) {
}

Time Time::fromRawSi(double seconds) {
    return Time(seconds);
}

Time Time::seconds(double value) {
    return Time(value);
}

Time Time::milliseconds(double value) {
    return Time(value / 1'000.0);
}

Time Time::microseconds(double value) {
    return Time(value / 1'000'000.0);
}

Time Time::nanoseconds(double value) {
    return Time(value / 1'000'000'000.0);
}

Time Time::minutes(double value) {
    return Time(value * 60.0);
}

Time Time::hours(double value) {
    return Time(value * 3'600.0);
}

double Time::rawSi() {
    return seconds_;
}

double Time::asSeconds() {
    return seconds_;
}

double Time::asMilliseconds() {
    return seconds_ * 1'000.0;
}

double Time::asMicroseconds() {
    return seconds_ * 1'000'000.0;
}

double Time::asNanoseconds() {
    return seconds_ * 1'000'000'000.0;
}

double Time::asMinutes() {
    return seconds_ / 60.0;
}

double Time::asHours() {
    return seconds_ / 3'600.0;
}

bool Time::approximatelyEquals(Time other, double epsilon) {
    return detail::absolute(seconds_ - other.seconds_) <= epsilon;
}

QuantityDisplay<Time, TimeUnit> Time::displayAs(TimeUnit unit) {
    return QuantityDisplay<Time, TimeUnit>(*this, unit);
}

QuantityDisplay<Time, TimeUnit> Time::displayAsPrecision(TimeUnit unit, int precision) {
    return QuantityDisplay<Time, TimeUnit>(*this, unit, precision);
}

bool operator==(Time left, Time right) {
    return left.seconds_ == right.seconds_;
}

} // namespace units
