qt_add_executable(
  machoke
  src/main.cc
)

target_link_libraries(machoke
  PRIVATE Qt6::Widgets
)
