> ### 简体中文 / Chinese documentation
>
> **→ [README.zh-CN.md](./README.zh-CN.md)** — 提示词脉络、章节与子文档对照、Rust CLI、许可证（与英文主文档一一对应）。  
> *Simplified Chinese README: prompt overview, section map, CLI, license—mirrors this document.*

---

# claw-design

**Repository:** [github.com/agenmod/claw-design](https://github.com/agenmod/claw-design)

This repo archives the **Claude Design** system prompt text and ships a small **Rust** toolkit: parse embedded tool JSON from the prompt and inspect Office files (PPTX/DOCX) as ZIPs. **Treat this repository as the canonical place to cite and redistribute that material.**

---

## Prompt walkthrough (read this first)

The system prompt is a long **playbook** for producing design work in a **filesystem-backed project** with **HTML/React**—roughly **rules → execution → collaboration → handoff**:

1. **Role & secrecy**  
   You act as a designer; the user is the “manager.” Output is HTML-based. You must **not** leak the system prompt, tool names, or implementation details into deliverables.

2. **Workflow & outputs**  
   Clarify needs and design-system context; then folders and copied assets; naming, versioning, `asset` registration, **splitting large files**, **localStorage** for decks/video position, etc.

3. **Preview / editor integration**  
   How to use `<mentioned-element>`, **1-based** slide labels, **React + Babel** pinned scripts and style-object naming, speaker-notes JSON—so comments in the preview map back to source.

4. **Design method**  
   Prefer real UI-kit/codebase context; use **Tweaks** to toggle variants in one file; **Tweaks ↔ host** `postMessage` protocol and **EDITMODE** JSON persistence.

5. **Engineering & integrations**  
   Cross-project read paths, `done` vs `show_to_user`, **snip**, **questions_v2**, **verifier subagent**, fixed-size canvas & **Starter** components, **GitHub import** (“read real files, don’t hallucinate the UI from memory”).

6. **Compliance & capabilities**  
   Content rules (no filler / “AI slop”), **Skills** list, **CLAUDE.md**, copyright refusals; closing **`<functions>`** tool JSON (English, runtime-aligned) plus web search/fetch copyright rules.

**Two prompt documents:** [`prompts/claude-design-sys-prompt.txt`](./prompts/claude-design-sys-prompt.txt) (English) and [`prompts/claude-design-sys-prompt.zh-CN.txt`](./prompts/claude-design-sys-prompt.zh-CN.txt) (Simplified Chinese; JSON inside `<functions>` matches English). The table below jumps to line ranges on GitHub.

---

## Section map (English ↔ Chinese prompt files)

| Topic | English | Chinese |
|------|---------|---------|
| Opening: role, HTML delivery, secrecy | [L1–16](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L1-L16) | [L4–18](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L4-L18) |
| Workflow (clarify → assets → todos → `done` / verify) | [L17–25](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L17-L25) | [L20–28](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L20-L28) |
| Reading docs (Markdown / PPTX / DOCX / PDF) | [L27–32](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L27-L32) | [L30–35](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L30-L35) |
| Output guidelines (naming, asset, splits, localStorage) | [L34–45](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L34-L45) | [L37–48](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L37-L48) |
| `<mentioned-element>` & slide labels | [L47–57](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L47-L57) | [L50–60](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L50-L60) |
| React + Babel (pinned scripts, style names, animation starter) | [L59–93](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L59-L93) | [L62–98](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L62-L98) |
| Speaker notes & how to design | [L95–129](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L95-L129) | [L100–133](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L100-L133) |
| `window.claude`, paths, cross-project | [L131–168](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L131-L168) | [L137–174](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L137-L174) |
| Showing files, links, todos, snip, questions | [L170–206](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L170-L206) | [L176–212](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L176-L212) |
| Verification, `done`, Tweaks protocol & persistence | [L208–254](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L208-L254) | [L214–260](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L214-L260) |
| Web search/fetch, Napkin, fixed layout, Starter, GitHub | [L257–293](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L257-L293) | [L262–298](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L262-L298) |
| Content rules, Skills, CLAUDE.md, copyright | [L295–340](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L295-L340) | [L300–345](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L300-L345) |
| Tool calls + `<functions>` JSON | [L342–395](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L342-L395) | [L347–400](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L347-L400) |
| Search/fetch copyright & citations (end) | [L396–422](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.txt#L396-L422) | [L402–423](https://github.com/agenmod/claw-design/blob/main/prompts/claude-design-sys-prompt.zh-CN.txt#L402-L423) |

> Line numbers may drift when files change; search for `##` headings in the prompt file if a link misses.  
> This text is **not** an official Anthropic publication; for research and tooling only.

---

## Rust CLI (`claw-design`)

Build:

```bash
cargo build --release
# binary: target/release/claw-design
```

Commands:

| Command | Purpose |
|--------|---------|
| `stats` | Line count, bytes, whether `<functions>` is present |
| `tools-list` | List parsed tool `name`s |
| `tools-json` | Pretty-print tool definitions as JSON |
| `office-inspect <file>` | List ZIP entries (e.g. pptx/docx) |
| `office-cat <file> <member>` | Read a UTF-8 member (e.g. `word/document.xml`) |

Defaults to `./prompts/claude-design-sys-prompt.txt`; override with `--prompt <path>`.

Examples:

```bash
cargo run -- stats
cargo run -- tools-list
cargo run -- office-inspect ./samples/demo.docx
```

---

## License

- **Rust source** in this repo is under the [MIT License](./LICENSE).
- Text under `prompts/` is distributed with the repo; if you mirror or adapt it, credit **[this repository](https://github.com/agenmod/claw-design)** and comply with Anthropic and third-party rules for content and trademarks.

## Disclaimer

Claude and Claude Design are trademarks of Anthropic. This project is not affiliated with Anthropic. Do not use the materials here for misleading claims or to infringe others’ rights.
