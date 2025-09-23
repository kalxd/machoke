if (${CMAKE_BUILD_TYPE} STREQUAL "Release")
  message("release mode")
else()
  message("debug mode")
  add_custom_command(
	OUTPUT xglib
	COMMAND echo "build xglib"
	COMMAND cargo build
  )
endif()
