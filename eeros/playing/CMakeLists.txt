cmake_minimum_required(VERSION 3.5.1)
project(eeros_playground)

set(CMAKE_CXX_STANDARD 14)
set(EXECUTABLE_OUTPUT_PATH ${CMAKE_BINARY_DIR})

find_package(EEROS REQUIRED)
include_directories(${EEROS_INCLUDE_DIR})
link_directories(${EEROS_LIB_DIR})

add_library(librobotcontrol SHARED IMPORTED GLOBAL)

include_directories(
  include
  src
)

file(GLOB all_SRCS
  "include/*.h"
  "include/*.hpp"
  "include/*/*.h"
  "include/*/*.hpp"
  "src/*.cpp"
  "src/*.c"
)

add_custom_command(OUTPUT copyHalConfig POST_BUILD COMMAND ${CMAKE_COMMAND} -E copy "${PROJECT_SOURCE_DIR}/config/*.json" "${CMAKE_BINARY_DIR}")

add_executable(${PROJECT_NAME} ${all_SRCS} copyHalConfig)
target_link_libraries(${PROJECT_NAME} eeros ucl ${CMAKE_DL_LIBS} robotcontrol)
