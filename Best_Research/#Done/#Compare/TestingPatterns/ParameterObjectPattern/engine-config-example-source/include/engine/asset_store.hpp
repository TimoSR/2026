#pragma once

#include <string>
#include <unordered_map>

#include "engine/file_system.hpp"
#include "engine/types.hpp"

namespace demo
{
    class AssetStore final
    {
    public:
        [[nodiscard]] static AssetStore Create(const FileSystem& fileSystem);

        ImageId LoadImage(const std::string& logicalName, const std::string& path);
        SoundId LoadSound(const std::string& logicalName, const std::string& path);

        [[nodiscard]] const std::string& ResolveImagePath(const ImageId& imageId) const;
        [[nodiscard]] const std::string& ResolveSoundPath(const SoundId& soundId) const;

    private:
        explicit AssetStore(FileSystem fileSystem);

        FileSystem fileSystem_;
        std::unordered_map<std::string, std::string> images_;
        std::unordered_map<std::string, std::string> sounds_;
    };
}
