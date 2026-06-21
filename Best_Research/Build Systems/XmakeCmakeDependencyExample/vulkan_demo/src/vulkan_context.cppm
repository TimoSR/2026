module;

#include <cstdint>
#include <windows.h>
#include <vulkan/vulkan.h>

export module vulkan_demo.context;

export namespace vulkan_demo::detail {

struct LoaderVersion {
    bool available{};
    std::uint32_t major{};
    std::uint32_t minor{};
    std::uint32_t patch{};
};

[[nodiscard]] LoaderVersion query_loader_version() {
    const auto loader = LoadLibraryW(L"vulkan-1.dll");
    if (loader == nullptr) {
        return {};
    }

    const auto enumerate_instance_version = reinterpret_cast<PFN_vkEnumerateInstanceVersion>(
        GetProcAddress(loader, "vkEnumerateInstanceVersion"));
    if (enumerate_instance_version == nullptr) {
        FreeLibrary(loader);
        return {true, 1, 0, 0};
    }

    std::uint32_t version = VK_API_VERSION_1_0;
    const auto result = enumerate_instance_version(&version);
    FreeLibrary(loader);
    if (result != VK_SUCCESS) {
        return {};
    }

    return {
        true,
        VK_API_VERSION_MAJOR(version),
        VK_API_VERSION_MINOR(version),
        VK_API_VERSION_PATCH(version)
    };
}

} // namespace vulkan_demo::detail
