set(CMAKE_C_FLAGS "--std=gnu99 -Wno-nullability-completeness")

FILE(GLOB CSources *.c)
FILE(GLOB CVendorSources vendor/*.c)
FILE(GLOB CHeaders *.h)
FILE(GLOB CVendorHeaders vendor/*.h)

add_executable(caesiumclt ${CVendorSources} ${CVendorHeaders} ${CSources} ${CHeaders})

target_link_libraries(caesiumclt LINK_PUBLIC caesium)
target_link_libraries(caesiumclt LINK_PUBLIC m)

install(TARGETS caesiumclt DESTINATION bin)