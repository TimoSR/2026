cmake_minimum_required(VERSION 3.20)
project(source_divided_example LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_executable(source_divided_demo
    src/main.cpp
    src/orders/order_service.cpp
    src/payments/stripe_payment_processor.cpp
)

target_include_directories(source_divided_demo PRIVATE src)