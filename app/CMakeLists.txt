include_directories(${CMAKE_SOURCE_DIR}/xglib/target/cxxbridge/xglib/src)

qt_add_executable(
  machoke
  src/main.cc
  src/mainwindow.cc
)

if (${CMAKE_BUILD_TYPE} STREQUAL "Release")
  target_link_libraries(machoke
	PRIVATE Qt6::Widgets
	PRIVATE ${CMAKE_SOURCE_DIR}/xglib/target/release/libxglib.a
  )
else ()
  message(${CMAKE_SOURCE_DIR}/xglib/target/debug/libxglib.a)
  target_link_libraries(machoke
	PRIVATE Qt6::Widgets
	PRIVATE ${CMAKE_SOURCE_DIR}/xglib/target/debug/libxglib.a
  )
endif()
