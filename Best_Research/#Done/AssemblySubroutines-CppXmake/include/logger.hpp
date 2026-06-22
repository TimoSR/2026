#pragma once

#include <string_view>

enum class LogLevel
{
    error,
    warn,
    info,
    debug,
    trace
};

[[nodiscard]] LogLevel parse_log_level(std::string_view value);

class Logger
{
  public:
    explicit Logger(LogLevel threshold) noexcept;

    void error(std::string_view message) const;
    void warn(std::string_view message) const;
    void info(std::string_view message) const;
    void debug(std::string_view message) const;
    void trace(std::string_view message) const;

  private:
    void log(LogLevel level, std::string_view message) const;

    LogLevel threshold_;
};
