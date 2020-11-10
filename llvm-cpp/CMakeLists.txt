cmake_minimum_required(VERSION 3.14)
project(test_llvm)

set(CMAKE_CXX_STANDARD 14)

find_package(LLVM REQUIRED CONFIG)
include_directories(${LLVM_INCLUDE_DIR})
add_definitions(${LLVM_DEFINITIONS})

add_executable(test_llvm main.cpp)

target_link_libraries(test_llvm LLVM)
