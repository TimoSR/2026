cmake_minimum_required(VERSION 3.20)
project(engine_config_example LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_executable(engine_config_example
    src/main.cpp
    src/file_system.cpp
    src/asset_store.cpp
    src/render_engine.cpp
    src/audio_engine.cpp
    src/physics_engine.cpp
    src/window.cpp
    src/config/app_configs.cpp
)

target_include_directories(engine_config_example PRIVATE include)

target_compile_options(engine_config_example PRIVATE
    $<$<CXX_COMPILER_ID:MSVC>:/W4 /permissive->
    $<$<NOT:$<CXX_COMPILER_ID:MSVC>>:-Wall -Wextra -Wpedantic>
)
