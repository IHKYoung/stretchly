# Task-ID: TID-20260403-repo-cleanup-foundation

## Meta
- Title: 仓库精简第一轮：剔除非核心外围资产
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

## Decision Log
- 第一轮仅删除与产品主体无关、且不依赖最终技术路线的外围资产。
- Docker/Snap 辅助壳、独立调试页与本地生成缓存可直接清理，不保留到二次开发阶段。
- Linux/Windows 分发资产、CI 工作流与构建资源是否继续保留，留待技术路线确定后做第二轮裁剪。

## Governance Notes
- Requirement Brief: 在不触碰核心应用代码主体的前提下，先删除 Docker/Snap 辅助壳、调试页和本地生成缓存，降低仓库噪音，为后续二次开发做准备。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: service_impact=repo hygiene only；touches_running_service=no；backup_required=no；backup_plan=以 git diff 和 VCS 为边界；rollback_plan=按文件级恢复本任务删除的 tracked 文件，并重新生成 `coverage/`；destructive_operations=删除非核心仓库文件与本地生成目录；operator_approval_required=no；rationale=用户已明确要求剔除非必要资产，且本轮不触碰产品核心代码。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 当前 developer policy 明确限制未获显式许可时启用子 agent；本任务由单 agent 在有限写入范围内执行。
- Retention Decision: keep

## Notes
- 本任务不等于平台重构；它只做“无争议的仓库瘦身”。
- 下一轮可在技术路线确定后继续清理 Linux/Windows 分发相关文件与工作流。
