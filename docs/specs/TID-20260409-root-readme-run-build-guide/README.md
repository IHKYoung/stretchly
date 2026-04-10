# Task-ID: TID-20260409-root-readme-run-build-guide

## Meta
- Title: 在根 README 汇总运行与打包命令
- Date: 2026-04-09
- Level: trivial
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Links
- Plan (daily): ../../plans/2026-04-09.md
- Log (daily): ../../logs/2026-04-09.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 根 `README.md` 重新启用，但只保留“当前有效运行/测试/打包命令”这一层职责，不再恢复旧的展示型内容。
- 命令口径全部以根 `package.json` 与 `apps/desktop/package.json` 当前真实 scripts 为准，避免 README 再次漂移成历史说明。
- README 里同时补充 macOS 当前机器架构打包、universal 打包和产物路径，减少后续反复查找 `target/release/bundle`。

## Governance Notes
- Requirement Brief: 用户要求把当前实际可用的运行、测试与打包命令汇总到根 README；本任务仅重建精简根 README 并同步索引文档，不恢复旧 README 展示内容或历史社区元数据。
- Interaction Impact: none
- Interaction Freeze: N/A
- Execution Safety Block: service_impact=仅限仓库文档入口；touches_running_service=no；backup_required=no；backup_plan=以 Git diff、README grep 与 workflow docs validator 为边界；rollback_plan=回退 README 与相关 docs 索引；destructive_operations=none；operator_approval_required=no；rationale=纯文档任务，不影响运行时行为、数据或外部系统。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: trivial-scribe-only
- Escalation Summary: 无；任务范围稳定，且仅涉及文档与索引同步。
- Retention Decision: keep

## Notes
- 新 README 明确声明 `apps/desktop` 是单一真源，根脚本只是转发层。
- 文档索引同步更新到 `docs/RepositoryGuidelines.md` 与 `docs/CodeMap.md`，避免继续写“根 README 已清理”这种过期口径。
