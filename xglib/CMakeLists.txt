if (${CMAKE_BUILD_TYPE} STREQUAL "Release")
  message("release mode")
  add_custom_command(
	OUTPUT xglib
	COMMAND echo "build xglib"
	COMMAND cargo build --release
	DEPENDS src/lib.rs
  )
else()
  message("debug mode")
  add_custom_command(
	OUTPUT xglib
	COMMAND echo "build xglib"
	COMMAND cargo build
	DEPENDS src/lib.rs
  )
endif()
