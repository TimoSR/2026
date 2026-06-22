#include "logger.hpp"

#include <algorithm>
#include <array>
#include <cctype>
#include <chrono>
#include <ctime>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <string>

namespace
{
constexpr std::array level_names{"error", "warn", "info", "debug", "trace"};

std::string normalise(std::string_view value)
{
    std::string result(value);
    std::ranges::transform(result, result.begin(),
                           [](const unsigned char character) { return static_cast<char>(std::tolower(character)); });
    return result;
}

std::string timestamp()
{
    const auto now = std::chrono::system_clock::now();
    const auto time = std::chrono::system_clock::to_time_t(now);
    std::tm local_time{};

#if defined(_WIN32)
    localtime_s(&local_time, &time);
#else
    localtime_r(&time, &local_time);
#endif

    std::ostringstream output;
    output << std::put_time(&local_time, "%H:%M:%S");
    return output.str();
}
} // namespace

LogLevel parse_log_level(const std::string_view value)
{
    const auto normalised = normalise(value);

    for (std::size_t index = 0; index < level_names.size(); ++index)
    {
        if (normalised == level_names[index])
        {
            return static_cast<LogLevel>(index);
        }
    }

    return LogLevel::warn;
}

Logger::Logger(const LogLevel threshold) noexcept : threshold_(threshold) {}

void Logger::error(const std::string_view message) const
{
    log(LogLevel::error, message);
}

void Logger::warn(const std::string_view message) const
{
    log(LogLevel::warn, message);
}

void Logger::info(const std::string_view message) const
{
    log(LogLevel::info, message);
}

void Logger::debug(const std::string_view message) const
{
    log(LogLevel::debug, message);
}

void Logger::trace(const std::string_view message) const
{
    log(LogLevel::trace, message);
}

void Logger::log(const LogLevel level, const std::string_view message) const
{
    if (static_cast<int>(level) > static_cast<int>(threshold_))
    {
        return;
    }

    std::clog << timestamp() << " " << level_names[static_cast<std::size_t>(level)] << " " << message << '\n';
}
