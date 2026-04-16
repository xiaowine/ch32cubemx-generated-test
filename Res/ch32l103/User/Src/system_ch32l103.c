/********************************** (C) COPYRIGHT *******************************
 * File Name          : system_ch32l103.c
 * Author             : WCH
 * Version            : V1.0.1
 * Date               : 2026/03/11
 * Description        : Device Peripheral Access Layer System Source File.
 *********************************************************************************
 * Copyright (c) 2021 Nanjing Qinheng Microelectronics Co., Ltd.
 * Attention: This software (modified or not) and binary are used for
 * microcontroller manufactured by Nanjing Qinheng Microelectronics.
 *******************************************************************************/
#include "ch32l103.h"


uint32_t SystemCoreClock = 0;

__I uint8_t HBPrescTable[16] = {0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 6, 7, 8, 9};

/*********************************************************************
 * @fn      SystemCoreClockUpdate
 *
 * @brief   Update SystemCoreClock variable according to Clock Register Values.
 *
 * @return  none
 */
void SystemCoreClockUpdate(void)
{
    uint32_t tmp = 0, pllmull = 0, pllsource = 0;

    tmp = RCC->CFGR0 & RCC_SWS;

    switch (tmp)
    {
    case 0x00:
        if (RCC->CTLR & (1 << 2))
        {
            SystemCoreClock = HSI_LP_VALUE;
        }
        else
        {
            SystemCoreClock = HSI_VALUE;
        }
        break;
    case 0x04:
        SystemCoreClock = HSE_VALUE;
        break;
    case 0x08:
        pllmull = RCC->CFGR0 & RCC_PLLMULL;
        pllsource = RCC->CFGR0 & RCC_PLLSRC;
        pllmull = (pllmull >> 18) + 2;

        if (pllmull == 17) pllmull = 18;

        if (pllsource == 0x00)
        {
            if (EXTEN->EXTEN_CTR & EXTEN_PLL_HSI_PRE)
            {
                SystemCoreClock = HSI_VALUE * pllmull;
            }
            else
            {
                SystemCoreClock = (HSI_VALUE >> 1) * pllmull;
            }
        }
        else
        {
            if ((RCC->CFGR0 & RCC_PLLXTPRE) != (uint32_t)RESET)
            {
                SystemCoreClock = (HSE_VALUE >> 1) * pllmull;
            }
            else
            {
                SystemCoreClock = HSE_VALUE * pllmull;
            }
        }

        break;
    default:
        SystemCoreClock = HSI_VALUE;
        break;
    }

    tmp = HBPrescTable[((RCC->CFGR0 & RCC_HPRE) >> 4)];
    SystemCoreClock >>= tmp;
}