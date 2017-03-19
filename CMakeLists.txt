cmake_minimum_required(VERSION 2.8.11)
project(caesiumclt)

# The version number.
set(VERSION_MAJOR 0)
set(VERSION_MINOR 10)
set(VERSION_PATCH 2)

configure_file(
        "src/config.h.in"
        "${PROJECT_BINARY_DIR}/config.h"
)

include_directories("${PROJECT_BINARY_DIR}")
if (WIN32)
    include_directories("C:\\libcaesium\\caesium")
endif()
add_subdirectory(src)