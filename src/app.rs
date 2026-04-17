use crate::context_merge::{merge_contexts_flattened, merge_flat_objects_override};
use crate::copy_stage::process_copy_stage;
use crate::renderer::render_templates_from_entries;
use crate::spec::{load_runtime, load_spec};
use std::fs;
use std::path::Path;

/// 构建主流程：加载配置 -> 合并上下文 -> 拷贝资源 -> 渲染模板。
pub fn run() -> Result<(), String> {
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

    // 阶段 1：拷贝全局非模板资源。
    process_copy_stage(&global_dir, output_root)?;
    // 阶段 2：拷贝型号非模板资源（可覆盖全局同名文件）。
    process_copy_stage(&model_dir, output_root)?;
    // 阶段 3：按规格渲染模板。
    render_templates_from_entries(
        res_dir,
        output_root,
        model_name,
        &model_spec.entries,
        &merged_context,
    )?;

    Ok(())
}

