cmake_minimum_required(VERSION 3.31)

project(machoke LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 6.5 COMPONENTS Widgets REQUIRED)

qt_standard_project_setup(REQUIRES 6.5)

add_subdirectory(xglib)
add_subdirectory(app)
