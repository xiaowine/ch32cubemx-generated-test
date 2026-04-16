add_library(core STATIC
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32v20x_adc.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32v20x_bkp.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32v20x_can.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32v20x_crc.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32v20x_dbgmcu.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32v20x_dma.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32v20x_exti.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32v20x_flash.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32v20x_gpio.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32v20x_i2c.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32v20x_iwdg.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32v20x_misc.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32v20x_opa.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32v20x_pwr.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32v20x_rcc.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32v20x_rtc.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32v20x_spi.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32v20x_tim.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32v20x_usart.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32v20x_wwdg.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/core_riscv.c"
)

target_include_directories(core PUBLIC
        "${CMAKE_CURRENT_LIST_DIR}/../Inc"
)
