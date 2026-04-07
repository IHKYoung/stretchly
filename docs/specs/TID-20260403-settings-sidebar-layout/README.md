# Task-ID: TID-20260403-settings-sidebar-layout

## Meta
- Title: Tauri 设置页重构为侧边栏分类布局
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

## Decision Log
- 继续以 `apps/desktop` 作为默认产品设置页入口，不回退到 Electron legacy 页面。
- 保留当前 Tauri/Rust host 已支持的设置字段，不新增新的设置能力或宿主契约。
- 参考旧 `app/preferences.html` 的分类导航思路，将当前 grouped settings 重组为“左侧分类、右侧详情、右侧保存/状态栏”的单页布局。

## Governance Notes
- Requirement Brief: 用户要求设置页在单页内更清楚地展示全部需要配置的内容，优先用侧边栏切换类别；此前开源底座已有设置先保留，不自行扩项。
- Interaction Impact: direct
- Interaction Freeze: 主窗口只改设置页信息架构；`?window=break` 的 break prompt 保持现状不扩项。
- Execution Safety Block: 仅调整 `apps/desktop` 设置前台布局和文案组织，不触碰运行中的服务、数据迁移或新依赖。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: developer policy 限制未获用户显式授权时启用子 agent；因此由单 agent 完成本轮前台重构与验证。
- Retention Decision: keep

## Notes
- 本任务的“保留已有设置”以当前 Tauri `PauzaSettings` 字段和旧底座 `preferences.html` 分类结构为共同边界。
- 浏览器自检截图已归档到 `./evidence/settings-sidebar-layout-desktop.png`。
