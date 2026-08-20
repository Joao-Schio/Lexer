cmake_minimum_required(VERSION 3.20)

project(Lexer LANGUAGES CXX)

# C++23
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Find every .cpp and .hpp under src/
file(GLOB_RECURSE SOURCES CONFIGURE_DEPENDS
    "${CMAKE_SOURCE_DIR}/src/*.cpp"
    "${CMAKE_SOURCE_DIR}/src/*.hpp"
)

add_executable(Lexer ${SOURCES})

target_include_directories(Lexer PRIVATE
    "${CMAKE_SOURCE_DIR}/src"
)

# Common compiler flags
target_compile_options(Lexer PRIVATE
    -Wall
    -Werror
    -Wextra
    -pedantic

    # Normal build: no optimization
    $<$<NOT:$<CONFIG:Release>>:-O0>

    # Release build
    $<$<CONFIG:Release>:-O2>
)

# Keep a copy of the latest successfully compiled binary
# in the repository root.
#
# CMake's normal `make clean` removes the build output,
# but does NOT know about this copied file, so ./Lexer remains.
add_custom_command(
    TARGET Lexer
    POST_BUILD

    COMMAND ${CMAKE_COMMAND} -E copy
        $<TARGET_FILE:Lexer>
        "${CMAKE_SOURCE_DIR}/Lexer"

    COMMENT "Copying Lexer executable to project root"
)

# `make release`
#
# Creates a separate Release configuration under build/release
# and builds it using -O2.
add_custom_target(release
    COMMAND ${CMAKE_COMMAND}
        -S "${CMAKE_SOURCE_DIR}"
        -B "${CMAKE_BINARY_DIR}/release"
        -DCMAKE_BUILD_TYPE=Release

    COMMAND ${CMAKE_COMMAND}
        --build "${CMAKE_BINARY_DIR}/release"

    COMMENT "Building Lexer release version with -O2"
)