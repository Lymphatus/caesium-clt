cmake_minimum_required(VERSION 3.16)
project(caesiumclt)

# The version number.
set(VERSION_MAJOR 0)
set(VERSION_MINOR 18)
set(VERSION_PATCH 0)

configure_file(
        "src/config.h.in"
        "${PROJECT_BINARY_DIR}/config.h"
)
if (NOT DEFINED LIBCAESIUM_PATH)
    message(FATAL_ERROR "LIBCAESIUM_PATH is not defined. Cannot find libcaesium.")
endif ()

link_directories(${LIBCAESIUM_PATH})

include_directories("${PROJECT_BINARY_DIR}")

add_subdirectory(src)