cmake_minimum_required(VERSION 3.31)

project(machoke LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 6.5 COMPONENTS Widgets REQUIRED)

qt_standard_project_setup(REQUIRES 6.5)

include_directories(${CMAKE_SOURCE_DIR}/xglib/target/cxxbridge/xglib/src)

if (LINUX)
  set(LIB_NAME libxglib.a)
elseif (WIN32)
  set(LIB_NAME libxglib.lib)
else(APPLE)
  set(LIB_NAME libxglib.dylib)
endif()

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
  set(LIB_XGFFI ${CMAKE_SOURCE_DIR}/xglib/target/debug/${LIB_NAME})
else()
  set(LIB_XGFFI ${CMAKE_SOURCE_DIR}/xglib/target/release/${LIB_NAME})
endif()

qt_add_executable(machoke
  app/main.cc
  app/mainwindow.cc
  app/fstree.cc
  app/coverhistory.cc
  app/mainwidget.cc

  app/widget/cover.cc
  app/widget/multiedit.cc
  app/widget/singleedit.cc
  app/widget/combomodel.cc
  app/rust/util.cc
)

target_link_libraries(
  machoke
  PRIVATE Qt6::Widgets
  PRIVATE ${LIB_XGFFI}
)
