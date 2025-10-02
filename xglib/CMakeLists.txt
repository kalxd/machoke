if (${CMAKE_BUILD_TYPE} STREQUAL "Release")
  set(DIST_DIR ${CMAKE_CURRENT_SOURCE_DIR}/target/release)
  set(CARGO_CMD cargo build --release)
else()
  set(DIST_DIR ${CMAKE_CURRENT_SOURCE_DIR}/target/debug)
  set(CARGO_CMD cargo build)
endif()

set(STATIC_LIB_FILE ${DIST_DIR}/libxglib.a)

add_custom_target(
  xglib.a
  COMMAND ${CARGO_CMD}
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_library(xglib STATIC IMPORTED GLOBAL)
add_dependencies(xglib xglib.a)

message(${STATIC_LIB_FILE})

set_target_properties(
  xglib
  PROPERTIES IMPORTED_LOCATION ${STATIC_LIB_FILE}
)
