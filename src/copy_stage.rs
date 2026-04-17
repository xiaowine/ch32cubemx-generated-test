use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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

/// 单阶段拷贝：目录不存在则跳过。
pub fn process_copy_stage(src_dir: &Path, dst_dir: &Path) -> Result<(), String> {
    if !src_dir.exists() {
        return Ok(());
    }

    copy_no_template(src_dir, dst_dir)?;
    Ok(())
}

