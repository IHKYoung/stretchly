# Task-ID: TID-20260403-settings-minimal-ui

## Meta
- Title: 设置页极简重做：仅保留核心真设置
- Date: 2026-04-03
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-03.md
- Log (daily): ../../logs/2026-04-03.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md
- Evidence Report: ./evidence/README.md

## Decision Log
- 保留 `apps/desktop/src-tauri/src/{state,commands,shell}.rs` 现有设置契约与命令面，不回退 Tauri host。
- 设置页只暴露 9 类核心真设置：微休息、长休息、提前提醒、延后、严格模式、智能暂停、打断风格、开机自启动、语言。
- 运行时动作如 pause / resume / focus / skip / reset 不再出现在设置页主结构中。
- 主窗口交互冻结为单一 split view：左侧约 `1/4` 作为导航，右侧约 `3/4` 作为唯一主内容面板。

## Governance Notes
- Requirement Brief: 基于用户确认的“真设置”清单，重写 `apps/desktop` 设置页 IA 与视觉，只保留高频核心设置，并去掉概览/快捷动作/多余卡片结构。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: 设置页只承担配置编辑与保存，不承担运行时快捷动作；主流转固定为左侧分类切换、右侧单面板编辑、顶部保存。
- Execution Safety Block: service_impact=仅改 Tauri 前端设置页与 locale/docs，不改 Rust host 契约；touches_running_service=no；backup_required=no；backup_plan=依赖 VCS、typecheck/build 与截图旁证；rollback_plan=回退 `apps/desktop/src/{App.tsx,styles.css,locales/*,components/ui/card.tsx}` 与本任务 docs；destructive_operations=替换当前设置页结构与视觉；operator_approval_required=no；rationale=用户已明确要求前端推翻重写且本轮无数据/外部副作用风险。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无；本轮边界在用户确认后收敛为单页前端重构。
- Retention Decision: keep

## Notes
- 关键证据：`./evidence/settings-minimal-1440x810.png`、`./evidence/settings-minimal-snapshot-1440x810.md`、`./evidence/browser-console.log`
- 关键验证：`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`、`python3 scripts/validate_workflow_docs.py --mode manual`
