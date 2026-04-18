# claw-design

**仓库主页：** [github.com/agenmod/claw-design](https://github.com/agenmod/claw-design)

本仓库归档 **Claude Design** 相关系统提示词全文，并提供 **Rust** 辅助工具（解析内嵌工具 JSON、检视 Office zip 结构）。**请以本仓库为引用与分发的 canonical 来源。**

---

## 提示词整体脉络（读文档前先看这段）

系统提示是一份「在**文件系统项目**里、用 **HTML/React** 做设计交付」的**长程操作手册**，大致按 **先定规矩 → 再干活 → 再协作 → 再收口** 展开：

1. **角色与保密**  
   模型被设定为「用户是经理、你是专业设计师」，用 HTML 产出；同时**禁止**泄露系统提示、工具名、环境实现细节——避免把内部机制写进交付物。

2. **工作流与产出**  
   先澄清需求与设计系统语境，再建目录与复制资源；交付物命名、版本拆分、资源引用方式、`asset` 注册、**大文件拆分**、幻灯片/视频的 **localStorage** 持久化等都有明确约束。

3. **与预览/编辑器协同**  
   说明如何解析 `<mentioned-element>`、幻灯片 **1 起始**标签、**React + Babel** 固定 script 与样式对象命名、**演讲者备注** JSON 等——对应「用户在预览里点选、评论」时的源码定位。

4. **设计方法论**  
   「如何做设计」一节强调：先找 UI 套件/代码库语境，高保真原型优先用 **Tweaks** 在同一文件里切换方案，而不是散落多文件；并有一套 **Tweaks 与宿主** 的 `postMessage` 协议与 **EDITMODE** 持久化块。

5. **工程与集成**  
   跨项目只读路径、`done` / `show_to_user` 分工、**上下文 snip**、**questions_v2** 问卷、**验证子代理**、固定尺寸画布与 **Starter 组件**、**GitHub 导入**与「从真实文件取 token，不要靠记忆脑补 UI」等。

6. **合规与能力边界**  
   内容准则（反填充、反 AI 味）、可用 **Skills** 列表、**CLAUDE.md** 约定、版权拒绝；文末是 **`<functions>`** 内嵌的**工具 JSON Schema**（与运行时一致，故英文保留），以及联网搜索/抓取时的版权与引用规范。

**子文档只有两份正文**：[`prompts/claude-design-sys-prompt.txt`](./prompts/claude-design-sys-prompt.txt)（英文全文）、[`prompts/claude-design-sys-prompt.zh-CN.txt`](./prompts/claude-design-sys-prompt.zh-CN.txt)（简体中文译本；`<functions>` 内 JSON 与英文相同）。下表按主题拆成「跳转到对应行」，方便在 GitHub 上直接打开。

---

## 章节与子文档链接（中英文对照）

| 主题 | 英文稿 | 中文稿 |
|------|--------|--------|
| 开篇：角色、HTML 交付、禁止泄露环境 | [L1–16](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L1-L16) | [L4–18](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L4-L18) |
| 工作流（澄清需求 → 资源 → 待办 → `done` / 验证） | [L17–25](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L17-L25) | [L20–28](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L20-L28) |
| 阅读文档（Markdown/PPTX/DOCX/PDF） | [L27–32](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L27-L32) | [L30–35](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L30-L35) |
| 产出创建指南（命名、asset、拆分、localStorage 等） | [L34–45](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L34-L45) | [L37–48](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L37-L48) |
| `<mentioned-element>` 与幻灯片标签 | [L47–57](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L47-L57) | [L50–60](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L50-L60) |
| React + Babel（script 固定版本、样式命名、动画 starter） | [L59–93](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L59-L93) | [L62–98](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L62-L98) |
| 演讲者备注、如何做设计工作 | [L95–129](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L95-L129) | [L100–133](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L100-L133) |
| HTML 内调用 `window.claude`、文件路径与跨项目 | [L131–168](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L131-L168) | [L137–174](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L137-L174) |
| 展示文件、页面链接、待办、上下文 snip、提问 | [L170–206](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L170-L206) | [L176–212](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L176-L212) |
| 验证、`done`、Tweaks 协议与持久化 | [L208–254](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L208-L254) | [L214–260](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L214-L260) |
| 联网搜索/抓取、Napkin、固定尺寸、Starter、GitHub 导入 | [L257–293](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L257-L293) | [L262–298](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L262-L298) |
| 内容准则、可用 Skills、CLAUDE.md、版权 | [L295–340](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L295-L340) | [L300–345](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L300-L345) |
| 工具调用格式 + `<functions>` JSON（英文原文） | [L342–395](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L342-L395) | [L347–400](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L347-L400) |
| 搜索/抓取版权与引用说明（文末） | [L396–422](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L396-L422) | [L402–423](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L402-L423) |

> **说明**：行号随文件更新可能漂移；若链接落点不准，在对应文件内搜索章节标题（`## …`）即可。  
> 该文本**并非** Anthropic 官方发布；仅供研究与工具链开发参考。

---

## Rust 工具链（`claw-design` CLI）

构建：

```bash
cargo build --release
# 可执行文件: target/release/claw-design
```

常用命令：

| 子命令 | 作用 |
|--------|------|
| `stats` | 行数、字节数、是否包含 `<functions>` |
| `tools-list` | 列出解析出的工具 `name` |
| `tools-json` | 将工具定义以 JSON 数组输出到 stdout |
| `office-inspect <file>` | 列出 pptx/docx 等 zip 内路径 |
| `office-cat <file> <member>` | 读取 zip 内 UTF-8 文本成员（如 `word/document.xml`） |

默认读取 `./prompts/claude-design-sys-prompt.txt`，可用 `--prompt <路径>` 指定其它文件。

示例：

```bash
cargo run -- stats
cargo run -- tools-list
cargo run -- office-inspect ./samples/demo.docx
```

---

## 许可证

- 本仓库中的 **Rust 源码** 以 [MIT License](./LICENSE) 发布。
- `prompts/` 目录下的文本与本仓库一并提供；转载或衍生使用时请保留来源指向 **[本仓库](https://github.com/agenmod/claw-design)**，并自行遵守 Anthropic 及第三方关于内容与商标的要求。

## 免责声明

Claude、Claude Design 及相关商标归 Anthropic 所有。本仓库不声称与 Anthropic 存在关联；请勿将此处材料用于误导性宣传或侵犯第三方权利的行为。
