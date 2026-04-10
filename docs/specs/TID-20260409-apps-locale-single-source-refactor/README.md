# Task-ID: TID-20260409-apps-locale-single-source-refactor

## Meta
- Title: 收口 apps 文案到单一 messages 真源
- Date: 2026-04-09
- Level: complex  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-09.md
- Log (daily): ../../logs/2026-04-09.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- `apps/desktop/src/locales/messages/*.json` 是当前唯一有效的桌面端文案真源；break prompt 专属提示语也必须并入这里。
- `apps/desktop/src/locales/config/*.json` 只保留语言元信息，不承担任何用户文案。
- `break-message-copy.ts` / `break-message-copy.json` 作为过渡文件退出默认架构，不再保留第二条文案读取链路。
- break prompt 运行时改为直接使用 `i18n.ts` 提供的 `t()` / `tList()`，不再绕过 `registry.generated.json`。

## Governance Notes
- Requirement Brief: 用户明确要求不要再使用 `break-message-copy.ts` / `break-message-copy.json`，并强调这不是小修，而是把 `apps/desktop` 的文案体系彻底收口为 `messages` 单真源；`config` 只保留多语言元信息，`app/` 不应重新参与当前桌面端文案链路。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: service_impact=仅限桌面端 locale 真源、break prompt 文案读取路径、locale registry 产物、回归测试与相关文档；touches_running_service=no；backup_required=no；backup_plan=依赖 Git diff、`python3 scripts/sync_desktop_locales.py`、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`、`npm test` 与 docs validator；rollback_plan=回退 `apps/desktop/src/{App.tsx,lib/break-prompt.ts,locales/messages/**}`、删除的 `break-message-copy.*`、测试与 docs；destructive_operations=none；operator_approval_required=no；rationale=不涉及线上服务、提权、外部副作用或历史重写。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked（当前 session 未获用户显式授权调用 `spawn_agent`）
- Escalation Summary: 无；当前范围不涉及破坏性动作或额外外部依赖。
- Retention Decision: keep

## Notes
- 本任务是对同日 `break-message-copy` 过渡方案的架构收敛：保留“可编辑、可多语言”的目标，但撤销独立文件路线。
- 当前桌面端 `desktopReady` 语言为 `zh-CN` / `en`，因此 break prompt 专属 key 先在这两份 `messages` 中显式维护；其它语言继续走 fallback 链。
- 已完成 `sync_desktop_locales.py`、typecheck、build、test 与 docs validator，当前 `apps/desktop/src` 中对 `break-message-copy.*` 的运行时引用为 0。
