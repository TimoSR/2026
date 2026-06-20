#include <iostream>

struct FrameRate
{
    int value;

    static constexpr FrameRate from_fps(int fps) noexcept
    {
        return FrameRate{fps};
    }
};

constexpr FrameRate operator""_fps(unsigned long long fps) noexcept
{
    return FrameRate::from_fps(static_cast<int>(fps));
}

constexpr FrameRate operator+(FrameRate lhs, FrameRate rhs) noexcept
{
    return FrameRate::from_fps(lhs.value + rhs.value);
}

constexpr FrameRate operator-(FrameRate lhs, FrameRate rhs) noexcept
{
    return FrameRate::from_fps(lhs.value - rhs.value);
}

constexpr FrameRate operator*(FrameRate rate, int scale) noexcept
{
    return FrameRate::from_fps(rate.value * scale);
}

constexpr FrameRate operator*(int scale, FrameRate rate) noexcept
{
    return rate * scale;
}

constexpr bool operator==(FrameRate lhs, FrameRate rhs) noexcept
{
    return lhs.value == rhs.value;
}

constexpr bool operator<(FrameRate lhs, FrameRate rhs) noexcept
{
    return lhs.value < rhs.value;
}

std::ostream& operator<<(std::ostream& os, FrameRate rate)
{
    return os << rate.value << " fps";
}

double frame_time_ms(FrameRate rate)
{
    return 1000.0 / static_cast<double>(rate.value);
}

int main()
{
    constexpr FrameRate film = 24_fps;
    constexpr FrameRate game = 60_fps;
    constexpr FrameRate replay = 2 * game;
    constexpr FrameRate capped = replay - 24_fps;

    static_assert(film < game, "24 fps should be less than 60 fps");
    static_assert(capped == 96_fps, "Expected 96 fps after operations");

    std::cout << "Film mode:   " << film << '\n';
    std::cout << "Game mode:   " << game << '\n';
    std::cout << "Replay mode: " << replay << '\n';
    std::cout << "Capped mode: " << capped << '\n';

    std::cout << '\n';
    std::cout << "Frame time at " << film << ": " << frame_time_ms(film) << " ms\n";
    std::cout << "Frame time at " << game << ": " << frame_time_ms(game) << " ms\n";

    return 0;
}
