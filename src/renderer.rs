use crate::user_code::merge_user_code_sections;
use crate::spec::SpecEntry;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use tera::{Context, Tera};

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

/// 渲染单个模板，并在目标文件已存在时回填 USER CODE 区块。
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

    let merged_rendered = if out_path.exists() {
        match fs::read_to_string(&out_path) {
            Ok(existing) => merge_user_code_sections(&rendered, &existing),
            Err(_) => rendered.clone(),
        }
    } else {
        rendered.clone()
    };

    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory {}: {}", parent.display(), e))?;
    }

    fs::write(&out_path, merged_rendered.as_bytes()).map_err(|e| {
        format!(
            "Failed to write {} from {}: {}",
            out_path.display(),
            template_path.display(),
            e
        )
    })?;

    Ok(())
}

/// 按 entries 顺序渲染模板。
pub fn render_templates_from_entries(
    res_dir: &Path,
    output_root: &Path,
    model_name: &str,
    entries: &[SpecEntry],
    context_value: &Value,
) -> Result<(), String> {
    for entry in entries {
        if !entry.should_render(context_value)? {
            let (_, out_path) = resolve_template_and_output_paths(
                res_dir,
                output_root,
                model_name,
                entry.template_name(),
            )?;
            if out_path.exists() {
                fs::remove_file(&out_path).map_err(|e| {
                    format!(
                        "Failed to remove skipped output file {}: {}",
                        out_path.display(),
                        e
                    )
                })?;
            }
            continue;
        }
        render_one_template(
            res_dir,
            output_root,
            model_name,
            entry.template_name(),
            context_value,
        )?;
    }

    Ok(())
}
