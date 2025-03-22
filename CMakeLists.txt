cmake_minimum_required(VERSION 3.30)
project(anl)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_STANDARD 23)

file(GLOB_RECURSE LIB_SOURCES
    "${CMAKE_CURRENT_SOURCE_DIR}/libs/*.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/libs/*.h"
)

add_executable(anl src/main.cpp ${LIB_SOURCES})

target_include_directories(anl PRIVATE "${CMAKE_CURRENT_SOURCE_DIR}/libs")
