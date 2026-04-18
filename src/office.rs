//! PPTX / DOCX 等 Office 文件本质为 ZIP：用 Rust 列出条目，对应提示词里「按 zip 解包读 XML」的技术路径。

use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;

/// 列出 zip 内所有条目的路径（含目录以 `/` 结尾的条目，若存在）。
pub fn list_zip_member_paths(path: &Path) -> Result<Vec<String>> {
    let file = File::open(path).with_context(|| format!("open {}", path.display()))?;
    let mut archive = ZipArchive::new(file).context("read zip archive")?;
    let mut names = Vec::with_capacity(archive.len());
    for i in 0..archive.len() {
        let name = archive.by_index(i)?.name().to_owned();
        names.push(name);
    }
    names.sort();
    Ok(names)
}

/// 读取 zip 内单个 UTF-8 文本（用于快速抽查 `word/document.xml` 等）。
pub fn read_zip_text_entry(path: &Path, member: &str) -> Result<String> {
    let file = File::open(path).with_context(|| format!("open {}", path.display()))?;
    let mut archive = ZipArchive::new(file).context("read zip archive")?;
    let mut zf = archive.by_name(member).with_context(|| format!("member {}", member))?;
    let mut buf = String::new();
    zf.read_to_string(&mut buf)?;
    Ok(buf)
}
