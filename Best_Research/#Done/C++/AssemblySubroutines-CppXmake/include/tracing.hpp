#pragma once

#include <chrono>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <mutex>
#include <string>
#include <string_view>
#include <vector>

class FlameProfiler
{
  public:
    explicit FlameProfiler(const std::filesystem::path &output_path);
    ~FlameProfiler();

    FlameProfiler(const FlameProfiler &) = delete;
    FlameProfiler &operator=(const FlameProfiler &) = delete;

    [[nodiscard]] static FlameProfiler *active() noexcept;
    void record(std::int64_t elapsed_microseconds);

  private:
    static FlameProfiler *active_;
    static thread_local std::vector<std::string> stack_;

    std::ofstream output_;
    std::mutex output_mutex_;

    friend class TraceSpan;
};

class TraceSpan
{
  public:
    explicit TraceSpan(std::string_view name);
    ~TraceSpan();

    TraceSpan(const TraceSpan &) = delete;
    TraceSpan &operator=(const TraceSpan &) = delete;

  private:
    bool active_ = false;
    std::chrono::steady_clock::time_point started_;
};
