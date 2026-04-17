# JSON Schema (Draft-07) 支持定义清单

本文档用于说明 **JSON Schema Draft-07** 可用的主要关键字（keywords）及其含义，便于编写和评审 schema。

## 1. 方言与标识

| 关键字       | 类型     | 说明                                                                       |
|-----------|--------|--------------------------------------------------------------------------|
| `$schema` | string | 指定 schema 使用的规范版本。Draft-07 通常为 `http://json-schema.org/draft-07/schema#` |
| `$id`     | string | 当前 schema 的标识 URI，用于引用解析                                                 |
| `$ref`    | string | 引用其他 schema 或当前文档内子 schema                                               |

## 2. 通用校验

| 关键字     | 类型             | 说明                                             |
|---------|----------------|------------------------------------------------|
| `type`  | string / array | 限定实例类型，如 `object`、`array`、`string`、`integer` 等 |
| `enum`  | array          | 限定值必须在给定枚举集中                                   |
| `const` | any            | 限定值必须等于指定常量                                    |

## 3. 数值相关

| 关键字                | 类型     | 说明        |
|--------------------|--------|-----------|
| `multipleOf`       | number | 必须是该值的整数倍 |
| `minimum`          | number | 最小值（含边界）  |
| `maximum`          | number | 最大值（含边界）  |
| `exclusiveMinimum` | number | 最小值（不含边界） |
| `exclusiveMaximum` | number | 最大值（不含边界） |

## 4. 字符串相关

| 关键字         | 类型      | 说明                               |
|-------------|---------|----------------------------------|
| `minLength` | integer | 最小长度                             |
| `maxLength` | integer | 最大长度                             |
| `pattern`   | string  | 正则表达式约束（ECMA-262 风格）             |
| `format`    | string  | 语义格式（如 `email`、`uri`）；是否强校验取决于实现 |

## 5. 数组相关

| 关键字               | 类型               | 说明                      |
|-------------------|------------------|-------------------------|
| `items`           | schema / array   | 列表或元组元素约束               |
| `additionalItems` | boolean / schema | 当 `items` 为数组时，控制额外位置元素 |
| `minItems`        | integer          | 最少元素个数                  |
| `maxItems`        | integer          | 最多元素个数                  |
| `uniqueItems`     | boolean          | 是否要求数组元素唯一              |
| `contains`        | schema           | 数组中至少一个元素满足该 schema     |

## 6. 对象相关

| 关键字                    | 类型               | 说明                                            |
|------------------------|------------------|-----------------------------------------------|
| `properties`           | object           | 指定属性名到子 schema 的映射                            |
| `patternProperties`    | object           | 属性名按正则匹配后的子 schema                            |
| `additionalProperties` | boolean / schema | 控制未在 `properties`/`patternProperties` 命中的额外属性 |
| `required`             | array            | 必填属性名列表                                       |
| `minProperties`        | integer          | 最少属性个数                                        |
| `maxProperties`        | integer          | 最多属性个数                                        |
| `propertyNames`        | schema           | 对属性名本身进行校验                                    |
| `dependencies`         | object           | 属性依赖（Draft-07 写法，含属性依赖和 schema 依赖）            |

## 7. 组合与条件

| 关键字     | 类型     | 说明               |
|---------|--------|------------------|
| `allOf` | array  | 全部子 schema 都必须通过 |
| `anyOf` | array  | 至少一个子 schema 通过  |
| `oneOf` | array  | 恰好一个子 schema 通过  |
| `not`   | schema | 子 schema 不通过才算通过 |
| `if`    | schema | 条件判断             |
| `then`  | schema | `if` 通过时生效       |
| `else`  | schema | `if` 不通过时生效      |

## 8. 复用与定义

| 关键字           | 类型     | 说明                                      |
|---------------|--------|-----------------------------------------|
| `definitions` | object | Draft-07 内部可复用子 schema 容器（通过 `$ref` 引用） |

示例：

```json
{
  "definitions": {
    "name": {
      "type": "string",
      "minLength": 1
    }
  },
  "type": "object",
  "properties": {
    "project_name": {
      "$ref": "#/definitions/name"
    }
  }
}
```

## 9. 注解类关键字（不直接决定通过/失败）

| 关键字           | 类型      | 说明    |
|---------------|---------|-------|
| `title`       | string  | 标题    |
| `description` | string  | 描述    |
| `default`     | any     | 默认值建议 |
| `examples`    | array   | 示例    |
| `readOnly`    | boolean | 只读提示  |
| `writeOnly`   | boolean | 只写提示  |
| `deprecated`  | boolean | 废弃提示  |

## 10. Draft-07 使用注意

- Draft-07 推荐使用 `definitions`，而不是新版本常见的 `$defs`。
- Draft-07 没有 `unevaluatedProperties`、`dependentSchemas`、`dependentRequired` 这些后续草案关键字。
- `format` 在不同验证器中可能是“提示”或“强校验”，需以具体实现配置为准。

