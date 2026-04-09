# Task-ID: TID-20260408-break-prompt-purity-pass

## Meta
- Title: 收敛休息界面并清理 break prompt 文案层
- Date: 2026-04-08
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-08.md
- Log (daily): ../../logs/2026-04-08.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 已明确拒绝继续使用左右分栏 + 圆环倒计时的展示型结构，统一改为单列纯净 break prompt。
- 已明确倒计时采用“数字倒计时 + 条形进度”组合；圆环方案视为过强组件感和重复信息。
- 已明确交互语 copy 只能由 locale JSON 管理，不再留在 `break-prompt.ts`。

## Governance Notes
- Requirement Brief: 用户明确要求把休息界面改成纯净 break 界面，移除“微休息”等标签化提示，只保留交互语和倒计时，并要求多语言内容回到 i18n 文件夹管理，同时修掉字段名直接露出的翻译问题。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: 单列纯净 break prompt；主视觉只保留交互语、数字倒计时、条形进度与 CTA；不改 host 调度与 CTA 语义。
- Execution Safety Block: service_impact=仅限本地 Tauri 前台 break prompt 与 locale 层；touches_running_service=no；backup_required=no；backup_plan=依赖 Git diff 与前端 build/typecheck；rollback_plan=回退 `App.tsx`、`i18n.ts`、`break-prompt.ts`、locale 与 docs；destructive_operations=替换 break prompt 布局并删除 prompt 硬编码；operator_approval_required=no；rationale=无数据、权限或外部副作用。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback（当前运行策略禁止未获用户显式授权时调用 `spawn_agent`）
- Fallback Reason Code: N/A（仅 fallback 时填写）
- Escalation Summary: 暂无；当前边界清晰，不涉及高风险动作。
- Retention Decision: keep

## Notes
- 本轮是对 2026-04-08 break prompt parity 补齐的一次“反向做减法”收敛：保留背景 / 音频 / 壁纸等能力，但把 break 主界面压回单句 prompt + 倒计时的高级感结构。
