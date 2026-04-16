add_library(core STATIC
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32l103_adc.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32l103_bkp.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32l103_can.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32l103_crc.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32l103_dbgmcu.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32l103_dma.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32l103_exti.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32l103_flash.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32l103_gpio.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32l103_i2c.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32l103_iwdg.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32l103_lptim.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32l103_misc.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32l103_opa.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32l103_pwr.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32l103_rcc.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32l103_rtc.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32l103_spi.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32l103_tim.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32l103_usart.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/ch32l103_wwdg.c"
        "${CMAKE_CURRENT_LIST_DIR}/../Src/core_riscv.c"
)

target_include_directories(core PUBLIC
        "${CMAKE_CURRENT_LIST_DIR}/../Inc"
)
