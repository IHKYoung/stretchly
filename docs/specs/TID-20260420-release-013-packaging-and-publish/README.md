# Task-ID: TID-20260420-release-013-packaging-and-publish

## Meta
- Title: 整理未提交改动并发布 0.1.3 macOS 版本
- Date: 2026-04-20
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-20.md
- Log (daily): ../../logs/2026-04-20.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 将 `0.1.3` 的正式版本边界限定为当前已存在的有效产品改动、发布脚本、站点 pinned 链接与 workflow 文档，不把根目录翻译 scratch 文件混入仓库版本。
- 当前 GitHub 上同时存在代码仓库与 release 仓库；本轮默认按现有站点脚本把 release 资产发布到 `IHKYoung/Pauza`，代码 commit/push 则按当前代码仓库远端执行。
- locale 消息文件已批量扩充，而前端/宿主运行时依赖 `registry.generated.json`；因此 `sync_desktop_locales.py` 是本次 release 的硬门禁，不可省略。

## Governance Notes
- Requirement Brief: 用户要求把当前没有提交的部分做一次文档整理并总结，作为 `0.1.3` 版本，随后打包本地 macOS 并上传 GitHub release 和 commit；本任务据此只收口有效产品改动与发布资产，不把根目录翻译 scratch 文件混入正式版本。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: service_impact=仅影响本地桌面端打包产物、站点 pinned 下载链接、release 资产与 GitHub 代码推送；touches_running_service=no；backup_required=no；backup_plan=`npm test` + `npm run typecheck` + `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` + `npm --prefix apps/desktop run build` + `npm --prefix apps/desktop run tauri build -- --bundles dmg --no-sign` + `python3 scripts/validate_workflow_docs.py --mode manual`；rollback_plan=回退版本真源/文档/站点 pinned URL，删除或覆盖错误的 GitHub release 资产，并对错误 push 使用后续 `git revert`；destructive_operations=none；operator_approval_required=no；rationale=用户已明确要求发布 `0.1.3`，本轮不涉及历史重写、生产服务写入或提权操作。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 当前 session 未获用户显式授权调用 `spawn_agent`，且上层工具策略要求仅在用户明确要求子 agent 时才可 delegation；因此在限定范围内采用单 agent 执行本轮 `0.1.3` 收口与发布。
- Retention Decision: keep

## Notes
- 根目录当前存在一批未追踪的翻译生成脚本与中间 JSON/TXT；本轮不会把这些文件纳入正式版本，而是通过 `.gitignore` 明确排除。
- 当前 `apps/site` 目录内的 workflow mirror 是正式版本资产的一部分，会一并纳入 `0.1.3`。
- 已验证本地产物 `Pauza_0.1.3_aarch64.dmg` 存在，并已发布到 `https://github.com/IHKYoung/Pauza/releases/tag/v0.1.3`。
