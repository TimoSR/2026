#include "engine/asset_store.hpp"

#include <iostream>
#include <stdexcept>

namespace demo
{
    AssetStore::AssetStore(FileSystem fileSystem)
        : fileSystem_(std::move(fileSystem))
    {
    }

    AssetStore AssetStore::Create(const FileSystem& fileSystem)
    {
        std::cout << "[assets] create asset store\n";
        return AssetStore(fileSystem);
    }

    ImageId AssetStore::LoadImage(const std::string& logicalName, const std::string& path)
    {
        if (logicalName.empty())
        {
            throw std::invalid_argument("AssetStore::LoadImage requires a non-empty logicalName.");
        }

        if (!fileSystem_.Exists(path))
        {
            throw std::invalid_argument("AssetStore::LoadImage path does not exist.");
        }

        images_.insert_or_assign(logicalName, path);

        std::cout
            << "[assets] load image id=\"" << logicalName << "\" path=\"" << path << "\"\n";

        return ImageId{ .value = logicalName };
    }

    SoundId AssetStore::LoadSound(const std::string& logicalName, const std::string& path)
    {
        if (logicalName.empty())
        {
            throw std::invalid_argument("AssetStore::LoadSound requires a non-empty logicalName.");
        }

        if (!fileSystem_.Exists(path))
        {
            throw std::invalid_argument("AssetStore::LoadSound path does not exist.");
        }

        sounds_.insert_or_assign(logicalName, path);

        std::cout
            << "[assets] load sound id=\"" << logicalName << "\" path=\"" << path << "\"\n";

        return SoundId{ .value = logicalName };
    }

    const std::string& AssetStore::ResolveImagePath(const ImageId& imageId) const
    {
        const auto iterator = images_.find(imageId.value);

        if (iterator == images_.end())
        {
            throw std::out_of_range("AssetStore::ResolveImagePath could not find image id.");
        }

        return iterator->second;
    }

    const std::string& AssetStore::ResolveSoundPath(const SoundId& soundId) const
    {
        const auto iterator = sounds_.find(soundId.value);

        if (iterator == sounds_.end())
        {
            throw std::out_of_range("AssetStore::ResolveSoundPath could not find sound id.");
        }

        return iterator->second;
    }
}
