#include "ch32v20x.h"

void SystemClock_Config(void);

/*********************************************************************
 * @fn      main
 *
 * @brief   Main program.
 *
 * @return  none
 */
int main(void)
{
    SystemClock_Config();
    NVIC_PriorityGroupConfig(NVIC_PriorityGroup_1);
    Delay_Init();
    while (1)
    {
    }
}

/**
  * @brief System Clock Configuration
  * @retval None
  */
void SystemClock_Config(void)
{
    RCC_DeInit();
    /* Use undivided HSI as PLL input (for 8MHz HSI -> 8*18 = 144MHz). */
    EXTEN->EXTEN_CTR |= EXTEN_PLL_HSI_PRE;
    RCC_HSICmd(ENABLE);
    while (RCC_GetFlagStatus(RCC_FLAG_HSIRDY) == RESET)
    {
    }
    RCC_HCLKConfig(RCC_SYSCLK_Div1);
    RCC_PCLK2Config(RCC_HCLK_Div1);
    RCC_PCLK1Config(RCC_HCLK_Div2);

    RCC_PLLConfig(RCC_PLLSource_HSI_Div2, RCC_PLLMul_18);
    RCC_PLLCmd(ENABLE);
    while (RCC_GetFlagStatus(RCC_FLAG_PLLRDY) == RESET)
    {
    }

    RCC_SYSCLKConfig(RCC_SYSCLKSource_PLLCLK);
    while (RCC_GetSYSCLKSource() != 0x08)
    {
    }
}
