use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// 规格文件：描述静态上下文和要渲染的模板列表。
#[derive(Debug, Deserialize)]
pub struct SpecDoc {
    #[serde(default)]
    pub contexts: HashMap<String, Value>,
    pub entries: Vec<String>,
}

/// 运行时文件：描述当前要构建的型号和动态上下文。
#[derive(Debug, Deserialize)]
pub struct RuntimeDoc {
    pub model_name: String,
    #[serde(default)]
    pub contexts: HashMap<String, Value>,
}

/// 加载型号规格 JSON。
pub fn load_spec(spec_path: &Path) -> Result<SpecDoc, String> {
    let raw = fs::read_to_string(spec_path)
        .map_err(|e| format!("Failed to read spec {}: {}", spec_path.display(), e))?;
    serde_json::from_str(&raw).map_err(|e| {
        format!(
            "Failed to parse spec {} with new schema: {}",
            spec_path.display(),
            e
        )
    })
}

/// 加载运行时配置 JSON。
pub fn load_runtime(runtime_path: &Path) -> Result<RuntimeDoc, String> {
    let raw = fs::read_to_string(runtime_path)
        .map_err(|e| format!("Failed to read runtime config {}: {}", runtime_path.display(), e))?;
    let runtime_value: Value = serde_json::from_str(&raw).map_err(|e| {
        format!(
            "Failed to parse runtime config {} as JSON value: {}",
            runtime_path.display(),
            e
        )
    })?;

    let model_name = runtime_value
        .get("model_name")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            format!(
                "Runtime config {} missing required string field 'model_name'",
                runtime_path.display()
            )
        })?;

    let runtime_dir = runtime_path.parent().ok_or_else(|| {
        format!(
            "Failed to locate parent directory for runtime config {}",
            runtime_path.display()
        )
    })?;
    let schema_path = runtime_dir
        .join("runtime")
        .join(format!("{}.schema.json", model_name));

    if !schema_path.exists() {
        return Err(format!(
            "Runtime schema not found for model '{}': {}",
            model_name,
            schema_path.display()
        ));
    }

    let schema_raw = fs::read_to_string(&schema_path).map_err(|e| {
        format!(
            "Failed to read runtime schema {}: {}",
            schema_path.display(),
            e
        )
    })?;
    let schema_value: Value = serde_json::from_str(&schema_raw).map_err(|e| {
        format!(
            "Failed to parse runtime schema {}: {}",
            schema_path.display(),
            e
        )
    })?;

    let validator = jsonschema::validator_for(&schema_value).map_err(|e| {
        format!(
            "Failed to compile runtime schema {}: {}",
            schema_path.display(),
            e
        )
    })?;

    let errors: Vec<String> = validator
        .iter_errors(&runtime_value)
        .map(|e| {
            format!(
                "- {} (instance_path: {}, schema_path: {})",
                e,
                e.instance_path(),
                e.schema_path()
            )
        })
        .collect();

    if !errors.is_empty() {
        return Err(format!(
            "Runtime config validation failed for {} with schema {}:\n{}",
            runtime_path.display(),
            schema_path.display(),
            errors.join("\n")
        ));
    }

    serde_json::from_value(runtime_value).map_err(|e| {
        format!(
            "Failed to deserialize validated runtime config {}: {}",
            runtime_path.display(),
            e
        )
    })
}
