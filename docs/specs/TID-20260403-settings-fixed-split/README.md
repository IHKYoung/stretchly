# Task-ID: TID-20260403-settings-fixed-split

## Meta
- Title: 设置页固定 1:3 分栏与 16:9 最小窗口
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
- 现有“侧边栏 + 主体 + save rail”三栏布局是本轮别扭感的主要来源，优先回退为稳定的双栏 split view。
- save/status dock 保留，但必须并回主体列；否则主体持续被压缩。
- 通过提高 Tauri 主窗口最小尺寸到 `1280x720`，将窄屏堆叠从桌面交付态中移除。
- 用户进一步指出“圆角太多、介绍太多”，因此同步收紧 desktop 前台 primitives 的圆角，并裁掉 hero / section / sidebar 中的冗长介绍。

## Governance Notes
- Requirement Brief: 修正设置页的全局分栏比例与主窗口尺寸约束，使 sidebar/main 回到稳定的 `1/4 + 3/4` split view，并收紧圆角与介绍文案。
- Interaction Impact: direct
- Interaction Freeze: `打开设置页 -> 左侧选分类 -> 右侧主体查看与保存`，不再允许全局第三列压缩主体。
- Execution Safety Block: 仅前台布局、desktop primitives 与窗口尺寸调整；不改 settings schema、host 命令或运行中服务。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked（developer policy 未获用户显式授权前不得 spawn_agent）
- Escalation Summary: 无
- Retention Decision: keep

## Notes
- 证据产物位于 `evidence/`，包括 `1440x810` 视口截图、DOM snapshot 与 console log。
