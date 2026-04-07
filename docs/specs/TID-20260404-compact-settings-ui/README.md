# Task-ID: TID-20260404-compact-settings-ui

## Meta
- Title: 压缩设置窗口与重做紧凑型设置界面
- Date: 2026-04-04
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-04.md
- Log (daily): ../../logs/2026-04-04.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 保留 `节奏 / 提醒与打断 / 智能暂停 / 通用` 四个分类，但把导航压成窄侧栏，避免再回到三栏展示页。
- 维持现有 `PauzaSettings` / `update_settings` / `toggle_autostart` 契约不变，只收紧 UI 表达与主窗口尺寸。
- 证据采用浏览器结构快照与控制台日志；PNG 截图仍受字体加载超时影响，因此在 evidence 中明确记录该限制。

## Governance Notes
- Requirement Brief: 用户明确要求把当前设置页从偏展示型的大窗口 UI 收回为“小巧、精简、工具化”的偏好窗口；本轮仅重做设置页结构、基础控件样式与主窗口尺寸，不改底层设置能力和 break 运行时逻辑。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（交互流程未变，本轮仅收紧视觉结构与窗口尺寸）
- Execution Safety Block: service_impact=仅调整桌面主窗口尺寸与设置页视觉结构；touches_running_service=no；backup_required=no；backup_plan=依赖 VCS、typecheck/build 与浏览器快照；rollback_plan=回退 `App.tsx`、基础 UI 组件、`styles.css`、`tauri.conf.json` 与相关 docs；destructive_operations=替换现有前台布局；operator_approval_required=no；rationale=用户已明确要求重做前台 UI，本轮不涉及数据迁移或外部副作用。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked（developer policy 限制：未获用户显式授权不得 spawn_agent）
- Escalation Summary: 无；本轮在单 agent 下完成实现、验证与证据落盘。
- Retention Decision: keep

## Notes
- 代码改动集中在 `apps/desktop/src/App.tsx`、`apps/desktop/src/styles.css`、`apps/desktop/src/components/ui/{button,select,segmented-control}.tsx` 与 `apps/desktop/src-tauri/tauri.conf.json`。
- 证据文件位于 `docs/specs/TID-20260404-compact-settings-ui/evidence/`。
