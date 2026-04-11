# Task-ID: TID-20260411-break-copy-line-layout

## Meta
- Title: 优化休息文案按句分行显示
- Date: 2026-04-11
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-11.md
- Log (daily): ../../logs/2026-04-11.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md
- Evidence: ./evidence/README.md

## Decision Log
- 当前 break prompt 会把长中文文案当普通段落渲染，浏览器因此会在任意字之间自动换行，容易把一句提示切碎。
- 修复策略改为“整句优先，超长句再按分句换行”：句末标点优先独占一行，句子过长时才按逗号/分号重新编排。
- 为避免规则继续散落在 JSX，新增纯函数 helper `break-copy-layout.ts`，并用 Vitest 固定住中文和英文的切分行为。

## Governance Notes
- Requirement Brief: 用户指出 break 提示文案现在会被从中间截断，容易造成误解；本任务聚焦 break prompt 的文案分行与排版，不改宿主调度、文案真源内容或 CTA 语义。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: 已冻结为“break prompt 中央文案按整句优先显示，过长句再按分句独立成行”，不改 CTA、倒计时和 break 触发时机。
- Execution Safety Block: service_impact=仅限 `apps/desktop` break prompt 的文案分行 helper、前台排版与对应测试/docs；touches_running_service=no；backup_required=no；backup_plan=依赖 `npm test`、desktop build、browser preview 截图与 workflow docs validator；rollback_plan=回退 `apps/desktop/src/{App.tsx,lib/break-copy-layout.ts}`、`test/desktopBreakCopyLayout.js` 与本任务 docs；destructive_operations=none；operator_approval_required=no；rationale=纯前台 UI 排版修复，不涉及数据、权限、外部副作用或线上服务。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 当前 session 未获用户显式 delegation 授权，遵循上层工具策略不调用 `spawn_agent`；因此在限定于 break prompt 排版、测试、preview 取证与文档闭环的范围内执行单 agent fallback。
- Retention Decision: keep

## Notes
- 代码变更位于 `apps/desktop/src/App.tsx` 与新增的 `apps/desktop/src/lib/break-copy-layout.ts`。
- 现实检查截图位于 `docs/specs/TID-20260411-break-copy-line-layout/evidence/break-copy-preview.png`。
