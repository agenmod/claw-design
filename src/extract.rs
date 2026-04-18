//! 从完整系统提示文本中解析 `<functions>` 内嵌的 JSON 工具定义（Rust 侧结构化）。

use anyhow::{anyhow, Context, Result};
use serde_json::Value;

/// 取出 `<functions> ... </functions>` 之间的原始片段（不含外层标签）。
pub fn functions_section(prompt: &str) -> Option<&str> {
    let start = prompt.find("<functions>")? + "<functions>".len();
    let rel = prompt.get(start..)?;
    let end = rel.find("</functions>")?;
    Some(rel.get(..end)?.trim())
}

/// 在片段中顺序解析每个 `<function>{ JSON }</function>` 的内层 JSON。
pub fn parse_function_blocks(section: &str) -> Result<Vec<Value>> {
    let mut out = Vec::new();
    let mut rest = section;
    while let Some(pos) = rest.find("<function>") {
        rest = rest
            .get(pos + "<function>".len()..)
            .ok_or_else(|| anyhow!("invalid slice after <function>"))?;
        let end = rest
            .find("</function>")
            .ok_or_else(|| anyhow!("unclosed <function> block"))?;
        let json_str = rest.get(..end).ok_or_else(|| anyhow!("empty function body"))?.trim();
        let v: Value = serde_json::from_str(json_str)
            .with_context(|| format!("invalid JSON inside <function>: {}…", &json_str[..json_str.len().min(80)]))?;
        out.push(v);
        rest = rest.get(end + "</function>".len()..).unwrap_or("");
    }
    Ok(out)
}

/// 从完整提示词解析全部工具 JSON。
pub fn tools_from_prompt(prompt: &str) -> Result<Vec<Value>> {
    let section = functions_section(prompt).ok_or_else(|| anyhow!("missing <functions> section"))?;
    parse_function_blocks(section)
}

/// 工具 `name` 字段列表（顺序与原文一致）。
pub fn tool_names(tools: &[Value]) -> Vec<String> {
    tools
        .iter()
        .filter_map(|v| v.get("name").and_then(|n| n.as_str()).map(str::to_owned))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_minimal_section() {
        let prompt = r#"
blah
<functions>
<function>{"name":"read_file","parameters":{"type":"object"}}</function>
<function>{"name":"write_file","parameters":{"type":"object"}}</function>
</functions>
tail"#;
        let tools = tools_from_prompt(prompt).unwrap();
        assert_eq!(tool_names(&tools), vec!["read_file", "write_file"]);
    }
}
