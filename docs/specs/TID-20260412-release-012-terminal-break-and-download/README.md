# Task-ID: TID-20260412-release-012-terminal-break-and-download

## Meta
- Title: 发布 0.1.2 终端式休息页与下载直链整理
- Date: 2026-04-12
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: merge-gate  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-12.md
- Log (daily): ../../logs/2026-04-12.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 休息页沿用官网打字机语言，但布局收口为居中的终端输出：保留 `Pauza>` prompt，同时让正文与时间、倒计时、按钮保持同一中轴。
- 微休息只展示一条稳定提示；长休息则在整句打完后停留 `30s` 再切到下一条，避免上一版切换过快。
- 官网下载入口回到单页直链：首页按钮前端先解析 GitHub `latest release` 的 Apple Silicon DMG，失败时回退到固定稳定链接；由于截至 `2026-04-12` GitHub 最新已发布 release 仍是 `v0.1.1`，固定回退链接暂不改到 `0.1.2`。
- 本次本地版本线统一提升到 `0.1.2`，提交只覆盖 break UI、官网下载入口和对应版本元数据，不夹带用户在 `src-tauri/src/{commands,state}.rs` 等未完成改动。

## Governance Notes
- Requirement Brief: 用户确认采用“居中的终端打字效果”，并要求对当前这轮改动做一次总结提交；同时说明本地版本线已升级为 `0.1.2`。本任务的目标是在不触碰未完成用户改动的前提下，将 break 终端化展示、官网下载直链整理和版本 bump 收口为一条本地提交。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: service_impact=仅限本地代码/文档收口、版本号提升与 git commit；touches_running_service=no；backup_required=no；backup_plan=`git diff --check` + `npm --prefix apps/desktop run typecheck` + `npm test -- test/desktopBreakCopyLayout.js test/desktopBreakIdeas.js` + `python3 scripts/validate_workflow_docs.py --mode manual`；rollback_plan=如需撤回，后续以本次 commit 为边界执行 `git revert`，提交前通过 staged diff 复核范围；destructive_operations=git commit（用户已明确授权）；operator_approval_required=no；rationale=纯本地 merge-gate 收口，不涉及运行中服务、数据迁移、外部付费副作用或提权。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked（当前 session 未获用户显式 delegation 授权，遵循上层工具策略不调用 `spawn_agent`）
- Escalation Summary: 无；范围已明确，且风险仅限本地收口与提交。
- Retention Decision: keep

## Notes
- 按仓库门禁，本任务仍声明 `architect,coder,tester,scribe` 为最小 Required Roles；但受当前 session 的上层工具策略约束，只能走 `single-agent-fallback`。
- commit 会只暂存本次相关路径；`apps/desktop/src-tauri/src/{commands,state}.rs`、既有 `2026-04-11` 文档改动和其他未跟踪任务目录保持不入本次提交。
