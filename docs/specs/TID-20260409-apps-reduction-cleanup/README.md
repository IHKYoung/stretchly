# Task-ID: TID-20260409-apps-reduction-cleanup

## Meta
- Title: 清理 apps 未使用迁移层并保留 app 参考目录
- Date: 2026-04-09
- Level: complex  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-09.md
- Log (daily): ../../logs/2026-04-09.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 2026-04-09：`apps/desktop/legacy-utils` 与依赖它的根级旧测试不再属于当前产品默认链路，直接删除，不再保留迁移兼容层。
- 2026-04-09：`coverage/`、`examples/`、`output/` 与根目录旧 `README` 展示素材都不属于主体开发资产，统一清理，只保留可重建的图标真源与原版 `app/` 参考代码。
- 2026-04-09：根级 `README.md`、`CONTRIBUTING.md`、`CODE_OF_CONDUCT.md`、`LICENSE`、Linux `desktop/metainfo` 也不再保留在主体开发路径；有用信息抽取到 `docs/RootMetadataArchive.md`。

## Governance Notes
- Requirement Brief: 用户要求继续对仓库做减法，删除影响主体开发判断的无用目录、迁移兼容层、旧测试、README 展示素材以及根级社区/发布元数据文件；保留 `apps/desktop` 作为唯一有效实现，保留 `app/` 作为原版参考代码目录，并把未来可能还要用的信息沉淀到 `docs/`。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: service_impact=仅限仓库结构、测试入口、README 与开发索引文档；touches_running_service=no；backup_required=no；backup_plan=依赖 Git diff、`rg` 审计、typecheck/build/test 与 workflow docs gate；rollback_plan=从 VCS 恢复被删除目录、测试、素材与 README/docs；destructive_operations=删除非运行时目录和 tracked 素材；operator_approval_required=no；rationale=用户已明确要求清理，且不涉及线上服务、权限、付费成本或数据迁移。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 当前无升级；若清理过程中发现仍有 build/runtime 引用这些目录或素材，则转为 BLOCKED 再调整范围。
- Retention Decision: keep

## Notes
- `app/` 继续作为原版参考目录保留。
- `build/`、`graphics/` 与 `apps/desktop/src-tauri/icons/*` 继续保留，避免误删当前打包与图标链路。
- 根级社区治理、法律与 Linux 发布元数据改由 `docs/RootMetadataArchive.md` 承接，不再保留在仓库根目录。
