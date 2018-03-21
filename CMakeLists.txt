project(gobject-rs)
cmake_minimum_required(VERSION 3.6.0)
cmake_policy(SET CMP0046 NEW)
cmake_policy(SET CMP0063 NEW)

# 加入其它cmake模块
list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")


string(TOUPPER  "${CMAKE_BUILD_TYPE}" CMAKE_BUILD_TYPE_UPPER)
if (CMAKE_BUILD_TYPE_UPPER STREQUAL DEBUG)
    set(RUST_TARGET_DIR target/debug/)
    set(RUST_BUILD_FLAG)
else()
    set(RUST_TARGET_DIR target/release/)
    set(RUST_BUILD_FLAG --release)
endif()

# rust target
include(FeatureSummary)
find_package(Cargo REQUIRED)
find_package(Rust REQUIRED)

set(RUST_DIR "${CMAKE_CURRENT_SOURCE_DIR}/rust")
set(RUST_LIB "${RUST_DIR}/${RUST_TARGET_DIR}/librust.a")
file(GLOB_RECURSE RUST_SRC ${RUST_DIR}/src/**/*.rs)

#message("${RUST_SRC}")
add_custom_command(
        OUTPUT "${RUST_LIB}"
        COMMAND ${Cargo_EXECUTABLE} build ${RUST_BUILD_FLAG}
        DEPENDS ${RUST_SRC}
        WORKING_DIRECTORY "${RUST_DIR}")
add_custom_target(rust_target DEPENDS "${RUST_LIB}")

# glib gobject gtk
find_package(PkgConfig REQUIRED)
pkg_check_modules(GOBJECT REQUIRED gobject-2.0)
link_directories(${GOBJECT_LIBRARY_DIRS})
include_directories(${GOBJECT_INCLUDE_DIRS})

# c/cpp target
list(APPEND Libs "${RUST_LIB}")
list(APPEND Libs ${GOBJECT_LIBRARIES})
list(APPEND Libs "resolv")
file(GLOB_RECURSE SOURCE src/**/*.c src/**/*.cpp src/include/*.h src/main.cpp)
include_directories(src/include)
add_executable(${PROJECT_NAME} ${SOURCE})
add_dependencies(${PROJECT_NAME} rust_target)
target_link_libraries(${PROJECT_NAME} ${Libs})
set_target_properties(${PROJECT_NAME} PROPERTIES
        CXX_STANDARD 14
        CXX_STANDARD_REQUIRED ON)


