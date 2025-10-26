cmake_minimum_required(VERSION 3.13)
set(CMAKE_TRY_COMPILE_TARGET_TYPE STATIC_LIBRARY)
project(picosdkshim C CXX ASM)

set(PICO_SDK_PATH ${CMAKE_CURRENT_LIST_DIR}/../sdk)
include(${PICO_SDK_PATH}/external/pico_sdk_import.cmake)
pico_sdk_init()

add_library(picosdkshim STATIC
    cshim.c
    stubs.c
)

target_include_directories(picosdkshim PUBLIC
    ${CMAKE_CURRENT_LIST_DIR}
)

target_link_libraries(picosdkshim
    pico_platform
    hardware_gpio
    hardware_clocks
    hardware_resets
    hardware_xosc
    hardware_vreg
    # hardware_timer
    # pico_multicore
)

target_compile_definitions(picosdkshim PRIVATE
    PICO_STDIO_USB=0
    PICO_STDIO_UART=0
)

# Install the static archive into "<OUT_DIR>/lib"
install(TARGETS picosdkshim
    ARCHIVE DESTINATION lib
    LIBRARY DESTINATION lib
    RUNTIME DESTINATION bin
)