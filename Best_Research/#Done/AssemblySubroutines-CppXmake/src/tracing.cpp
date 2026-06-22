#include "tracing.hpp"

#include <algorithm>
#include <filesystem>

FlameProfiler *FlameProfiler::active_ = nullptr;
thread_local std::vector<std::string> FlameProfiler::stack_;

FlameProfiler::FlameProfiler(const std::filesystem::path &output_path)
{
    if (const auto parent = output_path.parent_path(); !parent.empty())
    {
        std::filesystem::create_directories(parent);
    }

    output_.open(output_path, std::ios::out | std::ios::trunc);
    active_ = output_ ? this : nullptr;
}

FlameProfiler::~FlameProfiler()
{
    if (active_ == this)
    {
        active_ = nullptr;
    }
}

FlameProfiler *FlameProfiler::active() noexcept
{
    return active_;
}

void FlameProfiler::record(const std::int64_t elapsed_microseconds)
{
    std::scoped_lock lock(output_mutex_);

    for (std::size_t index = 0; index < stack_.size(); ++index)
    {
        if (index != 0)
        {
            output_ << ';';
        }

        output_ << stack_[index];
    }

    output_ << ' ' << std::max<std::int64_t>(elapsed_microseconds, 1) << '\n';
}

TraceSpan::TraceSpan(const std::string_view name)
    : active_(FlameProfiler::active() != nullptr), started_(std::chrono::steady_clock::now())
{
    if (active_)
    {
        FlameProfiler::stack_.emplace_back(name);
    }
}

TraceSpan::~TraceSpan()
{
    if (!active_)
    {
        return;
    }

    const auto elapsed = std::chrono::steady_clock::now() - started_;
    FlameProfiler::active()->record(std::chrono::duration_cast<std::chrono::microseconds>(elapsed).count());
    FlameProfiler::stack_.pop_back();
}
