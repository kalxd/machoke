include_directories(${CMAKE_SOURCE_DIR}/xglib/target/cxxbridge/xglib/src)

qt_add_executable(
  machoke
  src/main.cc
  src/mainframe.cc
  src/mainwindow.cc
  src/editor.cc
  src/component/input.cc
  src/widget/cover.cc
)

target_link_libraries(machoke
  PRIVATE Qt6::Widgets
  xglib
)
