cmake_minimum_required(VERSION 3.17)

project("Crusty")

if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE DEBUG)
endif()


set(lib_rusty ${PROJECT_SOURCE_DIR}/target/debug/librusty.a)  # the full path. change if needed.

add_executable(crusty crusty.c)
target_link_libraries(crusty PUBLIC
  ${lib_rusty}
)

target_include_directories(crusty PUBLIC
  ${PROJECT_SOURCE_DIR}/target/debug
)

