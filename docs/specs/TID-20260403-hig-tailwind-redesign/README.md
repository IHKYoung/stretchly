# Task-ID: TID-20260403-hig-tailwind-redesign

## Meta
- Title: Tauri 前台改版：Tailwind + Shadcn + Apple HIG
- Date: 2026-04-03
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Links
- Plan (daily): ../../plans/2026-04-03.md
- Log (daily): ../../logs/2026-04-03.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md
- Evidence: ./evidence/README.md

## Decision Log
- 将 `apps/desktop` 前台从手写 CSS 语义类重构为 Tailwind CSS v4 + shadcn 风格组件基座，并补 `components.json`、`src/components/ui/*` 与 `src/lib/utils.ts`。
- 设置页改为更符合 Apple HIG 的 grouped settings：顶部 overview hero + live overview，主体按 `节奏 / 信号 / 通用 / 高级` 分组，保存动作下沉到独立 rail。
- break prompt 保持单卡片模型，但用环形倒计时、玻璃材质和更清晰的主次按钮层级替代旧版普通卡片。
- 因浏览器 preview 默认不会进入 active break，交互证据通过 Playwright 注入 mocked `__TAURI_INTERNALS__` + `DesktopSnapshot` 采集。

## Governance Notes
- Requirement Brief: 仅重做 Tauri React 前台视觉层与组件层，不改 Rust host 命令契约，用 Tailwind + shadcn + Apple HIG 收敛主设置页与 break prompt。
- Interaction Impact: direct
- Interaction Freeze: 已冻结为 `主设置页` 与 `?window=break` 两条可见链路；本轮不改 tray、Rust host 调度语义或 Electron legacy UI。
- Execution Safety Block: service_impact=仅 Tauri 前台视觉/交互层；touches_running_service=no；backup_required=no；backup_plan=依赖 VCS + build/screenshot 旁证；rollback_plan=回退 `apps/desktop/**` 本轮前台改版与 docs；destructive_operations=替换原 React 前台样式与组件结构；operator_approval_required=no；rationale=用户已明确要求采用 Tailwind + Shadcn 重设计 UI，且无运行中服务/数据风险。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无；实现、截图和微文案问题均在单轮闭环内完成。
- Retention Decision: keep

## Notes
- 新的 UI 基座位于 `apps/desktop/components.json`、`apps/desktop/src/components/ui/*`、`apps/desktop/src/lib/utils.ts`。
- 证据位于 `docs/specs/TID-20260403-hig-tailwind-redesign/evidence/*`。
