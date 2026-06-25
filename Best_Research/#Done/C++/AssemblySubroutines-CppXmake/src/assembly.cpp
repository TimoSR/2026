#include "assembly.hpp"
#include "tracing.hpp"

extern "C" std::int64_t asm_add(std::int64_t left, std::int64_t right) noexcept;

std::int64_t assembly_add(const std::int64_t left, const std::int64_t right) noexcept
{
    TraceSpan span{"assembly_add"};
    return asm_add(left, right);
}
