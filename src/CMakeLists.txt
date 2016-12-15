FILE(GLOB CSources *.c)
FILE(GLOB CHeaders *.h)

add_executable(caesiumclt ${CSources} ${CHeaders})

find_library(caesium caesium /usr/local/lib)
target_link_libraries(caesiumclt LINK_PUBLIC caesium)

install(TARGETS caesiumclt DESTINATION bin)