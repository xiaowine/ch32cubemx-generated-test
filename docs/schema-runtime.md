  # Runtime Schema (Draft-07)

## 目标

- `Res/runtime/<model>.schema.json` 当前为手工维护。
- 当前落地为 `ch32l103`：`Res/runtime/ch32l103.schema.json`。
- Schema 标准固定为 JSON Schema Draft-07。

## 维护方式

- 直接编辑 `Res/runtime/ch32l103.schema.json`。
- `Res/runtime.json` 必须保持严格 JSON（不使用注释）。

## Draft-07 约束

生成结果必须包含：

- `"$schema": "http://json-schema.org/draft-07/schema#"`

只使用 Draft-07 兼容关键字，例如：

- `type`
- `required`
- `properties`
- `additionalProperties`
- `enum`
- `const`
- `allOf`
- `if` / `then`
- `patternProperties`
- `anyOf`

## ch32l103 规则

- 根对象：
  - `required`: `model_name`, `contexts`
  - `additionalProperties: false`
- `model_name`：
  - `const: "ch32l103"`
- `contexts`：
  - `required`: `mcu`, `clock`, `linker`
  - `usart` 可选
  - `additionalProperties: false`
- `mcu.startup_file`：
  - 枚举锁死，当前值：`"startup_ch32l103.S"`
- `clock_source` / `pll_source`：
  - 保留枚举约束
  - 保留联动条件：
    - `clock_source=HSE` => `pll_source` 仅 `RCC_PLLSource_HSE_Div1|RCC_PLLSource_HSE_Div2`
    - `clock_source=HSI` => `pll_source` 必须 `RCC_PLLSource_HSI_Div2`
- `usart`（可选）：
  - 数组项为单键对象（键匹配 `^USART[1-4]$`）
  - 每个 USART 配置要求 `tx_pin` 与 `rx_pin` 至少一个为字符串（不能同时为 `null`）

## 样例

通过样例片段（`tx_pin` 非空）：

```json
{
  "contexts": {
    "usart": [
      {
        "USART1": {
          "tx_pin": "GPIO_Pin_9",
          "rx_pin": null,
          "gpio_port": "GPIOA",
          "baud_rate": 115200,
          "word_length": 8,
          "stop_bits": 1,
          "parity": null,
          "hardware_flow_control": {
            "cts": false,
            "rts": false
          },
          "remap": null
        }
      }
    ]
  }
}
```

失败样例片段（`tx_pin` 与 `rx_pin` 同时为 `null`）：

```json
{
  "contexts": {
    "usart": [
      {
        "USART1": {
          "tx_pin": null,
          "rx_pin": null
        }
      }
    ]
  }
}
```

## 维护约定

- 修改 runtime schema 规则时，必须同步更新本文件。
- 若变更 Draft 版本，必须显式更新 `$schema`。
