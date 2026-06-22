#pragma once

#include <filesystem>
#include <string>
#include <string_view>
#include <unordered_map>

class EnvConfig
{
  public:
    static EnvConfig load_file(const std::filesystem::path &path);

    [[nodiscard]] std::string get(std::string_view key, std::string_view fallback) const;

  private:
    std::unordered_map<std::string, std::string> values_;
};
