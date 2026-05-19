cmake_minimum_required(VERSION 3.20)
project(assembly_subroutines_c C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

if(CMAKE_SYSTEM_PROCESSOR STREQUAL "x86_64" OR CMAKE_SYSTEM_PROCESSOR STREQUAL "AMD64")
    if(MSVC)
        enable_language(ASM_MASM)
        set(ASM_FILE asm/x86_64_windows/add.asm)
    else()
        enable_language(ASM)
        set(ASM_FILE asm/x86_64/add.s)
    endif()
elseif(CMAKE_SYSTEM_PROCESSOR STREQUAL "aarch64" OR CMAKE_SYSTEM_PROCESSOR STREQUAL "arm64")
    enable_language(ASM)
    set(ASM_FILE asm/aarch64/add.s)
else()
    message(FATAL_ERROR "Unsupported architecture: ${CMAKE_SYSTEM_PROCESSOR}")
endif()

add_executable(assembly_subroutines_c
    src/main.c
    ${ASM_FILE}
)
