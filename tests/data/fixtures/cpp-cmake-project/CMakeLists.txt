cmake_minimum_required(VERSION 3.15)
project(CMakeTestProject CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Add executable with intentional errors for testing
add_executable(test_app
    src/main.cpp
    src/utils.cpp
    src/math.cpp
)

# Add include directories
target_include_directories(test_app PRIVATE include)

# Enable all warnings
if(MSVC)
    target_compile_options(test_app PRIVATE /W4 /WX-)
else()
    target_compile_options(test_app PRIVATE -Wall -Wextra)
endif()
