# Task-ID: TID-20260727-settings-simplification-015

## Meta
- Title: 收口 0.1.5 设置体验改版
- Date: 2026-07-27
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-07-27.md
- Log (daily): ../../logs/2026-07-27.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 设置首页采用“快捷设置 + 单层主题详情”，移除独立的“详细设置”中转页；所有主题页直接返回首页。
- 三档节奏统一为约每小时一次 `5m` 完整休息，微休息分别为 `30m/20s`、`20m/20s`、`10m/30s`；均衡档是新安装默认值。
- 已有持久化节奏不自动迁移；不匹配三档时显示为“自定义”。
- long break 中文术语统一为“完整休息”；完整休息提示语完整呈现后停留 `60s` 再切换。
- 本地版本提升到 `0.1.5`；本任务不打包、不打 tag、不 push、不创建 GitHub Release，官网继续指向公开版 `0.1.4`。

## Governance Notes
- Requirement Brief: 根据用户反馈减少设置项决策负担，修正详情页返回路径，增加自定义节奏入口，用可解释依据校准三档节奏，统一“完整休息”术语与提示语轮换时间，并收口为本地 `0.1.5` commit。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: 设置首页直接进入五个主题页并直接返回；微休息单条提示不轮换，完整休息在文案完整呈现后停留 60 秒。
- Execution Safety Block: service_impact=none；touches_running_service=no；backup_required=no；rollback_plan=后续 `git revert`；destructive_operations=none；operator_approval_required=no；外部发布动作明确排除。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 上层策略只允许用户明确要求时启动子 agent，本任务由主 agent 在限定范围内完成；浏览器后端不可用作为显性证据缺口保留。
- Retention Decision: keep（设置导航契约、节奏依据、兼容边界与验证记录可用于后续版本维护）

## Notes
- 前端设置路由是展示状态事实源；Rust `PauzaSettings` 仍是持久化设置与调度默认值事实源。
- 本地 Dev 服务返回 HTTP 200，但浏览器能力返回空后端列表，未采集自动化截图或真实等待 60 秒的视频证据。
