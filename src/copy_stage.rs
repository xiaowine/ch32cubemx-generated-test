use crate::user_code::merge_user_code_sections;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn should_merge_user_code_on_copy(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| matches!(ext.to_ascii_lowercase().as_str(), "c" | "h"))
        .unwrap_or(false)
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

/// 仅复制非模板资源（非 .tera 文件）。
pub fn copy_no_template(src_dir: &Path, dst_dir: &Path) -> Result<(), String> {
    let files = files_by_extension(src_dir, false);
    for source_path in files {
        let relative_path = source_path.strip_prefix(src_dir).unwrap_or(&source_path);
        let destination = dst_dir.join(relative_path);

        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory {}: {}", parent.display(), e))?;
        }

        if should_merge_user_code_on_copy(&destination) {
            // 仅对 .c/.h 文本文件执行 USER CODE 区块回填；非 UTF-8 回退为字节复制。
            match fs::read_to_string(&source_path) {
                Ok(source_text) => {
                    let out_text = if destination.exists() {
                        match fs::read_to_string(&destination) {
                            Ok(existing_text) => {
                                merge_user_code_sections(&source_text, &existing_text)
                            }
                            Err(_) => source_text.clone(),
                        }
                    } else {
                        source_text.clone()
                    };

                    fs::write(&destination, out_text.as_bytes()).map_err(|e| {
                        format!(
                            "Failed to write copied text {} -> {}: {}",
                            source_path.display(),
                            destination.display(),
                            e
                        )
                    })?;
                }
                Err(_) => {
                    fs::copy(&source_path, &destination).map_err(|e| {
                        format!(
                            "Failed to copy {} -> {}: {}",
                            source_path.display(),
                            destination.display(),
                            e
                        )
                    })?;
                }
            }
        } else {
            fs::copy(&source_path, &destination).map_err(|e| {
                format!(
                    "Failed to copy {} -> {}: {}",
                    source_path.display(),
                    destination.display(),
                    e
                )
            })?;
        }
    }
    Ok(())
}

/// 单阶段拷贝：目录不存在则跳过。
pub fn process_copy_stage(src_dir: &Path, dst_dir: &Path) -> Result<(), String> {
    if !src_dir.exists() {
        return Ok(());
    }

    copy_no_template(src_dir, dst_dir)?;
    Ok(())
}
