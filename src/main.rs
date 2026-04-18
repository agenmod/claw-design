mod extract;
mod office;

use anyhow::Result;
use clap::{Parser, Subcommand};
use extract::{tool_names, tools_from_prompt};
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_PROMPT_REL: &str = "prompts/claude-design-sys-prompt.txt";

#[derive(Parser)]
#[command(name = "claw-design", version, about = "Claude Design 提示词与工具清单（Rust 解析器）")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 提示词文件元信息（行数、字节、是否含 <functions>）
    Stats {
        #[arg(long, default_value = DEFAULT_PROMPT_REL)]
        prompt: PathBuf,
    },
    /// 列出从提示词解析出的工具名称
    ToolsList {
        #[arg(long, default_value = DEFAULT_PROMPT_REL)]
        prompt: PathBuf,
    },
    /// 导出工具定义为 JSON 数组到 stdout
    ToolsJson {
        #[arg(long, default_value = DEFAULT_PROMPT_REL)]
        prompt: PathBuf,
    },
    /// 列出 Office 文档（pptx/docx）zip 内文件路径
    OfficeInspect {
        file: PathBuf,
    },
    /// 从 Office zip 中读取一段 UTF-8 文本（如 word/document.xml）
    OfficeCat {
        file: PathBuf,
        member: String,
    },
}

fn read_prompt(path: &Path) -> Result<String> {
    Ok(fs::read_to_string(path)?)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Stats { prompt } => {
            let text = read_prompt(&prompt)?;
            let lines = text.lines().count();
            let bytes = text.len();
            let has_functions = text.contains("<functions>");
            println!("path: {}", prompt.display());
            println!("lines: {lines}");
            println!("bytes: {bytes}");
            println!("has <functions>: {has_functions}");
        }
        Commands::ToolsList { prompt } => {
            let text = read_prompt(&prompt)?;
            let tools = tools_from_prompt(&text)?;
            let names = tool_names(&tools);
            for n in names {
                println!("{n}");
            }
        }
        Commands::ToolsJson { prompt } => {
            let text = read_prompt(&prompt)?;
            let tools = tools_from_prompt(&text)?;
            let s = serde_json::to_string_pretty(&tools)?;
            println!("{s}");
        }
        Commands::OfficeInspect { file } => {
            for p in office::list_zip_member_paths(&file)? {
                println!("{p}");
            }
        }
        Commands::OfficeCat { file, member } => {
            let s = office::read_zip_text_entry(&file, &member)?;
            print!("{s}");
        }
    }
    Ok(())
}
