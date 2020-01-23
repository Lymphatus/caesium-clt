cmake_minimum_required(VERSION 2.8.11)
project(caesiumclt)

# The version number.
set(VERSION_MAJOR 0)
set(VERSION_MINOR 15)
set(VERSION_PATCH 1)

configure_file(
        "src/config.h.in"
        "${PROJECT_BINARY_DIR}/config.h"
)

include_directories("${PROJECT_BINARY_DIR}")
if (WIN32)
    include_directories("C:\\libcaesium\\include")
else ()
    include_directories("/usr/local/include")
endif ()

if (APPLE)
    link_directories(/usr/local/lib)
elseif (WIN32)
    link_directories("C:\\libcaesium\\lib")
endif ()

add_subdirectory(src)