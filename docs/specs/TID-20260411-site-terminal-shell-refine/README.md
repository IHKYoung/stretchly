# Task-ID: TID-20260411-site-terminal-shell-refine

## Meta
- Title: 官网去卡片化并改为终端式提示头
- Date: 2026-04-11
- Level: trivial  <!-- trivial | moderate | complex -->
- Lane: fast    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-11.md
- Log (daily): ../../logs/2026-04-11.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md
- Evidence Report: ./evidence/README.md

## Decision Log
- 去掉中央舞台卡片壳，回到更干净的纸面终端感。
- `Pauza>` 作为单独提示头放到舞台区域左上角，主文案继续居中。

## Governance Notes
- Requirement Brief: 用户要求去掉首页背景卡片，并把 `Pauza>` 放到左上角，模拟终端输出感；现有打字机、下载按钮和互动层保持不变。
- Interaction Impact: direct
- Interaction Freeze: 已冻结为 `左上角终端提示头 + 中央输出区 + 无卡片纸面结构`；本轮不改动脚本逻辑、下载路由和文案池。
- Execution Safety Block: service_impact=仅限 `apps/site` 首页结构样式与相关 docs/evidence；touches_running_service=no；backup_required=no；backup_plan=依赖本地静态服务器、浏览器现实检查、截图证据与 workflow docs validator；rollback_plan=回退 `apps/site/{index.html,styles.css}` 与相关 docs；destructive_operations=none；operator_approval_required=no；rationale=纯静态站点视觉结构微调，不涉及线上服务、权限、数据或外部副作用。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无；本轮只收敛首页视觉结构，不动行为逻辑。
- Retention Decision: keep

## Notes
- 浏览器现实检查会重点确认：去卡片化后结构是否稳定、`Pauza>` 是否处于左上角、主文案是否仍保持居中。
- 证据见 `./evidence/README.md`。
