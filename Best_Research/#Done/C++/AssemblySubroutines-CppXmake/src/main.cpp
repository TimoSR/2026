#include "assembly.hpp"
#include "env_config.hpp"
#include "logger.hpp"
#include "tracing.hpp"

#include <cstdint>
#include <iostream>
#include <string>

namespace
{
    int run(const Logger *&logger)
    {
        TraceSpan span{"run"};

        constexpr std::int64_t left = 7;
        constexpr std::int64_t right = 35;

        logger.info("calling assembly addition: a=7 b=35");
        const auto result = assembly_add(left, right);
        logger.info("assembly addition completed: a=7 b=35 result=" + std::to_string(result));

        std::cout << left << " + " << right << " = " << result << '\n';
        return 0;
    }
} // namespace

int main()
{
    const auto config = EnvConfig::load_file(".env");

    const Logger logger {parse_log_level(config.get("APP_LOG", "warn"))};

    FlameProfiler profiler {config.get("TRACE_OUTPUT", "build/tracing.folded")};
    TraceSpan span {"main"};

    return run(logger);
}
