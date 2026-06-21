cmake_minimum_required(VERSION 3.20)
project(cmake_math LANGUAGES CXX)

add_library(cmake_math STATIC src/cmake_math.cpp)
target_compile_features(cmake_math PUBLIC cxx_std_20)
target_include_directories(cmake_math
    PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
        $<INSTALL_INTERFACE:include>
)

install(TARGETS cmake_math
    ARCHIVE DESTINATION lib
    LIBRARY DESTINATION lib
    RUNTIME DESTINATION bin
)
install(DIRECTORY include/ DESTINATION include)
