cmake_minimum_required(VERSION 3.28)
project(vulkan_demo LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_CXX_SCAN_FOR_MODULES ON)

set(VULKAN_HEADERS_INCLUDE_DIR "" CACHE PATH "Path resolved by xmake's vulkan-headers package")
if(NOT EXISTS "${VULKAN_HEADERS_INCLUDE_DIR}/vulkan/vulkan.h")
    message(FATAL_ERROR
        "VULKAN_HEADERS_INCLUDE_DIR must point to xmake's vulkan-headers package. "
        "Configure the project with xmake f --with_vulkan_demo=y.")
endif()

add_library(vulkan_demo STATIC)
target_sources(vulkan_demo
    PRIVATE
        src/vulkan_demo.cpp
    PRIVATE
        FILE_SET CXX_MODULES
        FILES src/vulkan_context.cppm
    PUBLIC
        FILE_SET HEADERS
        BASE_DIRS include
        FILES include/vulkan_demo/vulkan_demo.hpp
)
target_include_directories(vulkan_demo SYSTEM PRIVATE "${VULKAN_HEADERS_INCLUDE_DIR}")

target_compile_options(vulkan_demo PRIVATE
    $<$<CXX_COMPILER_ID:Clang>:-Wall;-Wextra;-Wpedantic;-Wconversion;-Wshadow;-Werror>
)

install(TARGETS vulkan_demo
    ARCHIVE DESTINATION lib
    LIBRARY DESTINATION lib
    RUNTIME DESTINATION bin
    FILE_SET HEADERS DESTINATION include
)
