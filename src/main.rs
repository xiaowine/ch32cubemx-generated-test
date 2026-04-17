use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tera::{Context, Tera};
use walkdir::WalkDir;

#[derive(Debug, Deserialize)]
struct SpecDoc {
    #[serde(default)]
    contexts: HashMap<String, Value>,
    entries: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RuntimeDoc {
    model_name: String,
    #[serde(default)]
    contexts: HashMap<String, Value>,
}

fn files_by_extension(dir: &Path, include_extension: bool) -> Vec<PathBuf> {
    const EXTENSION: &str = "tera";
    let mut files = Vec::new();
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .for_each(|entry| {
            let is_target_extension = entry.path().extension().is_some_and(|ext| ext == EXTENSION);

            if include_extension {
                if is_target_extension {
                    files.push(entry.path().to_path_buf());
                }
            } else if !is_target_extension {
                files.push(entry.path().to_path_buf());
            }
        });
    files
}

fn copy_no_template(src_dir: &Path, dst_dir: &Path) -> Result<(), String> {
    let files = files_by_extension(src_dir, false);
    for source_path in files {
        let relative_path = source_path.strip_prefix(src_dir).unwrap_or(&source_path);
        let destination = dst_dir.join(relative_path);

        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                format!(
                    "Failed to create directory {}: {}",
                    parent.display(),
                    e
                )
            })?;
        }

        fs::copy(&source_path, &destination).map_err(|e| {
            format!(
                "Failed to copy {} -> {}: {}",
                source_path.display(),
                destination.display(),
                e
            )
        })?;
    }
    Ok(())
}

fn load_spec(spec_path: &Path) -> Result<SpecDoc, String> {
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

fn load_runtime(runtime_path: &Path) -> Result<RuntimeDoc, String> {
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

fn resolve_template_and_output_paths(
    res_dir: &Path,
    output_root: &Path,
    model_name: &str,
    template_name: &str,
) -> Result<(PathBuf, PathBuf), String> {
    if !template_name.ends_with(".tera") {
        return Err(format!(
            "Invalid template name '{}': must end with .tera",
            template_name
        ));
    }

    let relative_template = Path::new(template_name);
    let prefix = relative_template
        .components()
        .next()
        .and_then(|comp| comp.as_os_str().to_str())
        .ok_or_else(|| format!("Invalid template path '{}'", template_name))?;

    if prefix != "global" && prefix != model_name {
        return Err(format!(
            "Invalid template prefix '{}' in '{}': must be 'global' or '{}'",
            prefix, template_name, model_name
        ));
    }

    let template_path = res_dir.join(relative_template);
    let without_prefix = relative_template.strip_prefix(prefix).map_err(|_| {
        format!(
            "Invalid template path '{}' after prefix validation",
            template_name
        )
    })?;

    let mut out_relative = without_prefix.to_path_buf();
    out_relative.set_extension("");
    let out_path = output_root.join(out_relative);

    Ok((template_path, out_path))
}

fn render_one_template(
    res_dir: &Path,
    output_root: &Path,
    model_name: &str,
    template_name: &str,
    context_value: &Value,
) -> Result<(), String> {
    let (template_path, out_path) =
        resolve_template_and_output_paths(res_dir, output_root, model_name, template_name)?;

    if !template_path.exists() {
        println!(
            "WARN: template '{}' configured in spec but file not found: {}",
            template_name,
            template_path.display()
        );
        return Ok(());
    }

    let template_text = fs::read_to_string(&template_path)
        .map_err(|e| format!("Failed to read template {}: {}", template_path.display(), e))?;

    let context = Context::from_serialize(context_value).map_err(|e| {
        format!(
            "Failed to build context for template {}: {}",
            template_name, e
        )
    })?;

    let rendered = Tera::one_off(&template_text, &context, false)
        .map_err(|e| format!("Failed to render {}: {}", template_path.display(), e))?;

    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory {}: {}", parent.display(), e))?;
    }

    fs::write(&out_path, rendered.as_bytes()).map_err(|e| {
        format!(
            "Failed to write {} from {}: {}",
            out_path.display(),
            template_path.display(),
            e
        )
    })?;

    Ok(())
}

fn render_templates_from_entries(
    res_dir: &Path,
    output_root: &Path,
    model_name: &str,
    entries: &[String],
    context_value: &Value,
) -> Result<(), String> {
    for entry in entries {
        render_one_template(
            res_dir,
            output_root,
            model_name,
            entry,
            context_value,
        )?;
    }

    Ok(())
}

fn merge_contexts_flattened(contexts: &HashMap<String, Value>) -> Result<Value, String> {
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

fn merge_flat_objects_override(base: &Value, overlay: &Value) -> Result<Value, String> {
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

fn process_copy_stage(src_dir: &Path, dst_dir: &Path) -> Result<(), String> {
    if !src_dir.exists() {
        return Ok(());
    }

    copy_no_template(src_dir, dst_dir)?;
    Ok(())
}

fn run() -> Result<(), String> {
    let res_dir = Path::new("./Res");
    let runtime_path = res_dir.join("runtime.json");
    let global_dir = res_dir.join("global");
    let output_root = Path::new("./output");

    let runtime_doc = load_runtime(&runtime_path)?;
    let model_name = &runtime_doc.model_name;
    let model_spec_path = res_dir.join("spec").join(format!("{model_name}.json"));
    let model_dir = res_dir.join(model_name);

    fs::create_dir_all(output_root).map_err(|e| {
        format!(
            "Failed to create output directory {}: {}",
            output_root.display(),
            e
        )
    })?;

    if !model_dir.exists() {
        return Err(format!("Model directory not found: {}", model_dir.display()));
    }

    let model_spec = load_spec(&model_spec_path)?;
    let static_context = merge_contexts_flattened(&model_spec.contexts)?;
    let runtime_context = merge_contexts_flattened(&runtime_doc.contexts)?;
    let merged_context = merge_flat_objects_override(&static_context, &runtime_context)?;

    println!("Build model: {}", model_name);

    // Stage 1: copy global non-template resources.
    process_copy_stage(&global_dir, output_root)?;
    // Stage 2: copy model-specific non-template resources (can override global files).
    process_copy_stage(&model_dir, output_root)?;
    // Stage 3: render templates fully driven by the model spec.
    render_templates_from_entries(
        res_dir,
        output_root,
        model_name,
        &model_spec.entries,
        &merged_context,
    )?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}
