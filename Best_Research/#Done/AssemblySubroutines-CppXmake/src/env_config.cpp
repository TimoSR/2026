#include "env_config.hpp"

#include <algorithm>
#include <cctype>
#include <fstream>

namespace
{
std::string trim(std::string value)
{
    const auto first = std::find_if_not(value.begin(), value.end(),
                                        [](const unsigned char character) { return std::isspace(character) != 0; });
    const auto last = std::find_if_not(value.rbegin(), value.rend(),
                                       [](const unsigned char character) { return std::isspace(character) != 0; })
                          .base();

    if (first >= last)
    {
        return {};
    }

    return {first, last};
}
} // namespace

EnvConfig EnvConfig::load_file(const std::filesystem::path &path)
{
    EnvConfig config;
    std::ifstream input(path);

    if (!input)
    {
        return config;
    }

    std::string line;
    while (std::getline(input, line))
    {
        line = trim(std::move(line));

        if (line.empty() || line.starts_with('#'))
        {
            continue;
        }

        const auto separator = line.find('=');
        if (separator == std::string::npos)
        {
            continue;
        }

        auto key = trim(line.substr(0, separator));
        auto value = trim(line.substr(separator + 1));

        if (!key.empty())
        {
            config.values_.insert_or_assign(std::move(key), std::move(value));
        }
    }

    return config;
}

std::string EnvConfig::get(const std::string_view key, const std::string_view fallback) const
{
    const auto iterator = values_.find(std::string(key));
    return iterator == values_.end() ? std::string(fallback) : iterator->second;
}
