# Task-ID: TID-20260403-tauri-core-parity

## Meta
- Title: Tauri 核心功能补齐并废弃默认 Electron 入口
- Date: 2026-04-03
- Level: complex
- Lane: deep
- Execution Profile: sequential-phases
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
- 将根目录默认开发/构建入口切换到 `apps/desktop` Tauri 2 runtime，Electron 入口降级为 `legacy:*` 脚本保留。
- 不再继续维护“迁移展示页”，Tauri 前台收敛为极简设置页与独立 break prompt 两种界面。
- 核心能力优先迁入 Rust host：设置持久化、调度、pause/focus、pre-break notification、自然休息、DND、应用排除、tray、shortcut、break window 生命周期。
- 交互上保留高级选项，但默认主界面只暴露 `节奏 / 信号 / 通用` 三组，高级项收折叠。
- 根据用户后续反馈，设置页的产品语言继续从“控制台/展示页”收敛到“现代 macOS 小工具偏好页”。
- 证据层本轮仅完成文档化收口；可复用相邻任务 `TID-20260403-tauri-usable-core` 的浏览器结构快照作为旁证，但缺少本任务完成态的独立 tray / strict / manual-awaiting 可视化证据。

## Governance Notes
- Requirement Brief: 将 Tauri 从“可用核心闭环”继续补齐到足以替代默认 Electron 启动入口的状态，同时不删除旧 Electron 代码主体，只把其降为 legacy/source basis。
- Interaction Impact: direct
- Interaction Freeze: 主入口固定为极简设置页，break 交互固定为独立 prompt；不再引入展示型 dashboard 或额外前台页面。
- Execution Safety Block: service_impact=切换默认开发/构建入口到 Tauri 并补齐 Tauri host 核心功能；touches_running_service=no；backup_required=no；backup_plan=保留旧 Electron 代码与 legacy scripts 作为回退边界；rollback_plan=恢复根脚本指向 Electron 并回退 `apps/desktop/**` 与关联 docs；destructive_operations=废弃默认 Electron 入口与移除展示型前台；operator_approval_required=no；rationale=用户已连续批准迁移到 Tauri 并要求把功能迁全。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无实现阻塞；当前剩余缺口主要在交互证据而非代码/测试。
- Retention Decision: keep

## Notes
- 该任务的“废弃 Electron”是指废弃默认入口，而不是删除 `app/**` 参考实现。
- 根目录 `package.json` 现默认走 `desktop:dev` / `desktop:build`；Electron 仍保留 `legacy:start`、`legacy:dev`、`legacy:pack`、`legacy:dist`。
- 文档门禁已补齐，但证据报告明确记录了仍待补采的可视化证明项。
- 当前 UI 定位应理解为：安静的 grouped settings、segmented control、switch、setting-card，以及简洁 break prompt，而不是 dashboard 式工作台。
