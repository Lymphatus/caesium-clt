cmake_minimum_required(VERSION 2.8.11)
project(caesiumclt)

# The version number.
set(VERSION_MAJOR 0)
set(VERSION_MINOR 14)
set(VERSION_PATCH 0)

configure_file(
        "src/config.h.in"
        "${PROJECT_BINARY_DIR}/config.h"
)

include_directories("${PROJECT_BINARY_DIR}")
if (WIN32)
    include_directories("C:\\libcaesium\\caesium")
else ()
    include_directories("/usr/local/include")
endif ()

if (APPLE)
    link_directories(/usr/local/lib)
endif ()

add_subdirectory(src)