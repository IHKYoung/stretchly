# Task-ID: TID-20260409-language-selector-multilang-fix

## Goals
- 移除桌面端运行时对 `desktopReady` 的语言封锁，让现有 locale registry 中的语言都可被选择并实际生效。

## Non-Goals
- 不新增或重写任何翻译内容。
- 不改 locale registry 生成脚本。

## Constraints & Assumptions
- `apps/desktop/src/locales/messages/*.json` 与 `config/*.json` 已齐全。
- 语言 fallback 仍由每语言 config 中的 `fallback` 字段负责。

## System Boundaries
- Modules:
  - `apps/desktop/src/i18n.ts`
  - `apps/desktop/src-tauri/src/i18n.rs`
  - `apps/desktop/src/App.tsx`
- Ownership:
  - 前后端语言归一化与设置页语言选择入口
- Dependency direction:
  - locale registry -> front/Rust i18n -> settings page

## Invariants
- 未知语言仍回退到默认语言。
- 已存在于 locale registry 的语言不再因为 `desktopReady: false` 被强制改写。

## Risks & Rollback
- Failure modes:
  - 前后端语言归一化不一致，导致保存后前台与 host 语言不一致
- Rollback steps:
  - 回退 `i18n.ts`、`i18n.rs` 与 `App.tsx` 中的语言选择逻辑

## Acceptance Criteria (System)
- 设置页可列出完整语言列表。
- `normalizeLanguage('tr')`、`normalizeLanguage('ja')` 等不再回退到默认语言。
