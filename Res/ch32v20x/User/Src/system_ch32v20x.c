/********************************** (C) COPYRIGHT *******************************
 * File Name          : system_ch32v20x.c
 * Author             : WCH
 * Version            : V1.0.0
 * Date               : 2021/06/06
 * Description        : CH32V20x Device Peripheral Access Layer System Source File.
 *                      For HSE = 32Mhz (CH32V208x/CH32V203RBT)
 *                      For HSE = 8Mhz (other CH32V203x)
*********************************************************************************
* Copyright (c) 2021 Nanjing Qinheng Microelectronics Co., Ltd.
* Attention: This software (modified or not) and binary are used for 
* microcontroller manufactured by Nanjing Qinheng Microelectronics.
*******************************************************************************/
#include "ch32v20x.h"
uint32_t SystemCoreClock = 0;

__I uint8_t AHBPrescTable[16] = {0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 6, 7, 8, 9};


/*********************************************************************
 * @fn      SystemCoreClockUpdate
 *
 * @brief   Update SystemCoreClock variable according to Clock Register Values.
 *
 * @return  none
 */
void SystemCoreClockUpdate (void)
{
  uint32_t tmp = 0, pllmull = 0, pllsource = 0, Pll_6_5 = 0;

  tmp = RCC->CFGR0 & RCC_SWS;

  switch (tmp)
  {
    case 0x00:
      SystemCoreClock = HSI_VALUE;
      break;
    case 0x04:
      SystemCoreClock = HSE_VALUE;
      break;
    case 0x08:
      pllmull = RCC->CFGR0 & RCC_PLLMULL;
      pllsource = RCC->CFGR0 & RCC_PLLSRC;
      pllmull = ( pllmull >> 18) + 2;

      if(pllmull == 17) pllmull = 18;

      if (pllsource == 0x00)
      {
          if(EXTEN->EXTEN_CTR & EXTEN_PLL_HSI_PRE){
              SystemCoreClock = HSI_VALUE * pllmull;
          }
          else{
              SystemCoreClock = (HSI_VALUE >> 1) * pllmull;
          }
      }
      else
      {
#if defined (CH32V20x_D8W) || defined (CH32V20x_D8)
        if(((RCC->CFGR0 & (3<<22)) == (3<<22)) && (RCC_USB5PRE_JUDGE()== SET))
        {
          SystemCoreClock = ((HSE_VALUE>>1)) * pllmull;
        }
        else
#endif
        if ((RCC->CFGR0 & RCC_PLLXTPRE) != (uint32_t)RESET)
        {
#if defined (CH32V20x_D8) || defined (CH32V20x_D8W)
          SystemCoreClock = ((HSE_VALUE>>2) >> 1) * pllmull;
#else
          SystemCoreClock = (HSE_VALUE >> 1) * pllmull;
#endif
        }
        else
        {
#if defined (CH32V20x_D8) || defined (CH32V20x_D8W)
            SystemCoreClock = (HSE_VALUE>>2) * pllmull;
#else
          SystemCoreClock = HSE_VALUE * pllmull;
#endif
        }
      }

      if(Pll_6_5 == 1) SystemCoreClock = (SystemCoreClock / 2);

      break;
    default:
      SystemCoreClock = HSI_VALUE;
      break;
  }

  tmp = AHBPrescTable[((RCC->CFGR0 & RCC_HPRE) >> 4)];
  SystemCoreClock >>= tmp;
}

