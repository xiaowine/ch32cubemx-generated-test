use serde_json::Value;
use std::collections::HashMap;

/// 将多组命名 context 扁平化为一个对象。
/// 若出现同名 key 且值不一致，直接报错，避免静默覆盖。
pub fn merge_contexts_flattened(contexts: &HashMap<String, Value>) -> Result<Value, String> {
    let mut merged = serde_json::Map::new();

    let mut names: Vec<&str> = contexts.keys().map(String::as_str).collect();
    names.sort_unstable();

    for name in names {
        let context_value = contexts
            .get(name)
            .ok_or_else(|| format!("Context '{}' is missing", name))?;
        let context_obj = context_value
            .as_object()
            .ok_or_else(|| format!("Context '{}' must be a JSON object", name))?;

        for (key, value) in context_obj {
            if let Some(existing) = merged.get(key) {
                if existing != value {
                    return Err(format!(
                        "Conflicting context key '{}' between merged contexts",
                        key
                    ));
                }
                continue;
            }
            merged.insert(key.clone(), value.clone());
        }
    }

    Ok(Value::Object(merged))
}

/// 将 overlay 覆盖到 base 上，用于运行时参数覆盖静态参数。
pub fn merge_flat_objects_override(base: &Value, overlay: &Value) -> Result<Value, String> {
    let mut base_obj = base
        .as_object()
        .ok_or_else(|| "Base context must be a JSON object".to_string())?
        .clone();
    let overlay_obj = overlay
        .as_object()
        .ok_or_else(|| "Overlay context must be a JSON object".to_string())?;

    for (key, value) in overlay_obj {
        base_obj.insert(key.clone(), value.clone());
    }

    Ok(Value::Object(base_obj))
}

