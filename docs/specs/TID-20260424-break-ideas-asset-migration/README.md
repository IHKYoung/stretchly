# Task-ID: TID-20260424-break-ideas-asset-migration

## Meta
- Title: 拆分 break ideas 资产真源
- Date: 2026-04-24
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-24.md
- Log (daily): ../../logs/2026-04-24.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 2026-04-24: 用户确认 `miniBreakIdeas / longBreakIdeas` 仍然是唯一语义真源，但不应继续放在 `messages/*.json`，也不应被抽成代码常量。
- 2026-04-24: 本轮先迁移全部现有语言 break ideas 到独立 JSON 目录，不删任何 legacy 语言，只通过 registry 数据标签标记 `official / legacy`。
- 2026-04-24: 运行时改为读取 `break-ideas/registry.generated.json`，优先使用当前语言 bundle，未来若某语言 bundle 被移除，则沿 locale fallback 链补位。

## Governance Notes
- Requirement Brief: 将 `miniBreakIdeas / longBreakIdeas` 从 `messages/*.json` 迁移到独立 JSON 资产目录，避免与 UI 文案继续耦合；同时以 registry 数据标记 `official / legacy` 语言，而不是在代码里写死三语常量。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: 仅影响 desktop locale 资产、前台 helper、sync scripts、Vitest 与 docs；无服务影响、无外部副作用，回滚方式是恢复旧消息文件 ideas key 与相关 helper/script。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: N/A（仅 fallback 时填写）
- Escalation Summary: 当前 session 未获用户显式 delegation，按工具策略保持单 agent 闭环完成迁移与验证。
- Retention Decision: keep

## Notes
- 已落地：
  - `apps/desktop/src/locales/break-ideas/messages/*.json`
  - `apps/desktop/src/locales/break-ideas/registry.json`
  - `apps/desktop/src/locales/break-ideas/registry.generated.json`
  - `scripts/sync_desktop_break_ideas.py`
- 已回收：
  - `messages/*.json` 顶层的 `miniBreakIdeas / longBreakIdeas`
  - `i18n.ts` 中的 break ideas 读取职责
- 已验证：
  - `python3 scripts/sync_desktop_break_ideas.py`
  - `python3 scripts/sync_desktop_locales.py`
  - `npm run typecheck`
  - `npm test`
  - `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - `python3 scripts/validate_workflow_docs.py --mode manual`
