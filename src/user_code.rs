use std::collections::HashMap;

/// 从已有文件中提取全部 USER CODE 区块内容。
fn extract_user_code_sections(content: &str) -> Result<HashMap<String, String>, String> {
    const USER_BEGIN: &str = "/* USER CODE BEGIN ";
    let mut sections = HashMap::new();
    let mut cursor = 0usize;

    while let Some(begin_rel) = content[cursor..].find(USER_BEGIN) {
        let begin_start = cursor + begin_rel;
        let name_start = begin_start + USER_BEGIN.len();
        let marker_end_rel = content[name_start..]
            .find("*/")
            .ok_or_else(|| "Malformed USER CODE marker: missing */".to_string())?;
        let name_end = name_start + marker_end_rel;
        let section_name = content[name_start..name_end].trim().to_string();

        let body_start = name_end + 2;
        let end_marker = format!("/* USER CODE END {} */", section_name);
        let end_rel = content[body_start..]
            .find(&end_marker)
            .ok_or_else(|| format!("Missing end marker for USER CODE section '{}'", section_name))?;
        let end_start = body_start + end_rel;

        sections.insert(section_name, content[body_start..end_start].to_string());
        cursor = end_start + end_marker.len();
    }

    Ok(sections)
}

/// 将已有文件中的 USER CODE 内容回填到新渲染结果中。
/// 解析失败时保守降级：直接返回渲染结果，不阻断生成流程。
pub fn merge_user_code_sections(rendered: &str, existing: &str) -> String {
    let existing_sections = match extract_user_code_sections(existing) {
        Ok(sections) => sections,
        Err(_) => return rendered.to_string(),
    };
    if existing_sections.is_empty() {
        return rendered.to_string();
    }

    const USER_BEGIN: &str = "/* USER CODE BEGIN ";
    let mut out = String::with_capacity(rendered.len());
    let mut cursor = 0usize;

    while let Some(begin_rel) = rendered[cursor..].find(USER_BEGIN) {
        let begin_start = cursor + begin_rel;
        out.push_str(&rendered[cursor..begin_start]);

        let name_start = begin_start + USER_BEGIN.len();
        let marker_end_rel = match rendered[name_start..].find("*/") {
            Some(pos) => pos,
            None => return rendered.to_string(),
        };
        let name_end = name_start + marker_end_rel;
        let section_name = rendered[name_start..name_end].trim();
        let body_start = name_end + 2;
        let end_marker = format!("/* USER CODE END {} */", section_name);
        let end_rel = match rendered[body_start..].find(&end_marker) {
            Some(pos) => pos,
            None => return rendered.to_string(),
        };
        let end_start = body_start + end_rel;
        let end_end = end_start + end_marker.len();

        out.push_str(&rendered[begin_start..body_start]);
        if let Some(saved_body) = existing_sections.get(section_name) {
            out.push_str(saved_body);
        } else {
            out.push_str(&rendered[body_start..end_start]);
        }
        out.push_str(&rendered[end_start..end_end]);

        cursor = end_end;
    }

    out.push_str(&rendered[cursor..]);
    out
}

