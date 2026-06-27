#pragma once

#include "units/detail/quantity_display.hpp"

namespace units {

enum class TimeUnit {
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds,
    Minutes,
    Hours,
};

class Time {
public:
    static Time fromRawSi(double seconds);

    static Time seconds(double value);

    static Time milliseconds(double value);

    static Time microseconds(double value);

    static Time nanoseconds(double value);

    static Time minutes(double value);

    static Time hours(double value);

    [[nodiscard]] double rawSi() const;

    [[nodiscard]] double asSeconds() const;

    [[nodiscard]] double asMilliseconds() const;

    [[nodiscard]] double asMicroseconds() const;

    [[nodiscard]] double asNanoseconds() const;

    [[nodiscard]] double asMinutes() const;

    [[nodiscard]] double asHours() const;

    [[nodiscard]] bool approximatelyEquals(Time other, double epsilon) const;

    [[nodiscard]] QuantityDisplay<Time, TimeUnit> displayAs(TimeUnit unit) const;

    [[nodiscard]] QuantityDisplay<Time, TimeUnit> displayAsPrecision(TimeUnit unit, int precision) const;

    friend bool operator==(Time left, Time right) = default;

private:
    explicit Time(double seconds);

    double seconds_;
};

} // namespace units
