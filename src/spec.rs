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
    serde_json::from_str(&raw).map_err(|e| {
        format!(
            "Failed to parse runtime config {}: {}",
            runtime_path.display(),
            e
        )
    })
}

