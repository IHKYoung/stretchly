# Task-ID: TID-20260411-root-scripts-cleanup

## Meta
- Title: 整理根 package.json 重复脚本
- Date: 2026-04-11
- Level: trivial
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Links
- Plan (daily): ../../plans/2026-04-11.md
- Log (daily): ../../logs/2026-04-11.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 根 `package.json` 只保留一套 repo 级短入口（`dev/build/typecheck`），其余使用显式命名空间命令，避免同义别名持续扩散。
- 删除 `start/pack/dist/test-single/prepublishOnly/postpublish`；其中前四者仅是重复或低价值入口，后两者还依赖当前仓库未安装的 `pinst`。
- 测试相关命令统一改为 `test` / `test:coverage` / `test:watch`，并同步更新 CI 与当前有效文档。

## Governance Notes
- Requirement Brief: 用户认为根 `package.json` 的 scripts 重复过多，希望整理成更清晰的一层 repo 入口；本次仅收敛根脚本、CI 命令名和当前有效文档，不改 `apps/desktop/package.json` 的子应用脚本语义。
- Interaction Impact: none
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: service_impact=仅限根 npm scripts、CI 命令名与文档命令说明；touches_running_service=no；backup_required=no；backup_plan=通过 package.json parse、脚本 grep、`npm test`、`npm run typecheck` 与 workflow docs validator 验证；rollback_plan=回退 `package.json`、`.github/workflows/tests.yml` 与相关 docs；destructive_operations=none；operator_approval_required=no；rationale=不涉及线上服务、数据迁移、权限或外部付费动作。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无；范围限定在 repo 级命令整理与文档收口。
- Retention Decision: keep

## Notes
- 相关依据已同步到 `README.md`、`docs/RepositoryGuidelines.md`、`docs/CodeMap.md` 与 `docs/CHANGELOG.md`。
