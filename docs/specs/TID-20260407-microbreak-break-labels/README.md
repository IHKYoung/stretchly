# Task-ID: TID-20260407-microbreak-break-labels

## Meta
- Title: 统一微休息与休息命名
- Date: 2026-04-07
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-07.md
- Log (daily): ../../logs/2026-04-07.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 2026-04-07：当前产品 UI 术语统一收敛为“微休息 / 休息”，英文对应 `Microbreak / Break`。
- 2026-04-07：内部兼容优先于名词洁癖，本轮明确保留 `miniBreak*` / `longBreak*` / `BreakKind.longBreak` 等既有标识，不触碰设置 schema、迁移和命令名。
- 2026-04-07：历史 spec / logs / evidence 视为审计切片，不做批量改写；只更新当前活跃 locale、现行文档与必要元数据。
- 2026-04-07：浏览器 preview 的设置页快照与控制台摘要已归档到 `./evidence/`，用于证明主设置面的术语已经切换。

## Governance Notes
- Requirement Brief: 用户明确要求把产品里的对应命名改成“微休息 / 休息”；本轮只改当前用户可见文案、现行说明文档和最小元数据，不改内部字段名和行为逻辑。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: 已冻结为“现行用户可见术语替换”，不扩展为 schema rename、命令 rename 或历史证据重写。
- Execution Safety Block: service_impact=仅限当前 UI / locale / 文档 / 元数据文案；touches_running_service=no；backup_required=no；backup_plan=以 Git diff、浏览器 preview、typecheck/build 和 workflow docs 为回滚边界；rollback_plan=回退 locale、README/docs、metainfo 与本任务 evidence；destructive_operations=替换现有可见文案；operator_approval_required=no；rationale=不涉及数据、权限、外部副作用或提权。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 当前 session 未获用户显式 delegation，遵循上层工具约束不 `spawn_agent`；由单 agent 在限定范围内完成规划、文案替换、验证与文档结案。
- Retention Decision: keep

## Notes
- 证据目录：`docs/specs/TID-20260407-microbreak-break-labels/evidence/`
- 当前主设置面、break runtime 文案、tray skip 文案、legacy app locale、README 和元数据已统一使用新术语；内部兼容字段仍保留旧命名。
