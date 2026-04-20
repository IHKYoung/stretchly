# Task-ID: TID-20260420-release-013-packaging-and-publish

## Summary
- Title: 整理未提交改动并发布 0.1.3 macOS 版本
- Date: 2026-04-20
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 把当前未提交的 desktop / site / docs 变更整理成 `0.1.3`，统一版本真源、补齐发布文档、打本地 macOS 包，并完成 GitHub release / commit 的发布动作。
- In-scope:
  - 统一 root / desktop / Tauri / Cargo 版本真源到 `0.1.3`
  - 更新 `README.md`、`docs/CHANGELOG.md`、`docs/CodeMap.md`、`docs/Architecture.md` 与当日任务文档
  - 纳入 `apps/site` 的 release workflow mirror、playbook 与 pinned 下载链接更新
  - 重建 desktop locale registry 并验证 50 个 locale 的运行时真源一致性
  - 构建当前机器架构的 macOS dmg，上传 GitHub `v0.1.3` release，提交并 push 代码
- Out-of-scope:
  - 清理或提交根目录翻译 scratch 文件
  - 改变代码仓库与 release 仓库的长期拓扑
  - 新增 UI 或调度功能
- Assumptions:
  - release 资产继续发布到 `IHKYoung/Pauza`
  - 代码 commit/push 先按当前代码仓库远端执行
  - 当前机器可产出无签名 macOS dmg
- Risks:
  - 代码仓库与 release 仓库分离，可能导致版本追踪歧义
  - locale registry 未同步会让新增文案无法生效
  - dmg 构建或 GitHub 上传失败会阻断 release
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 将当前有效工作树收口成一条可追溯、可构建、可下载的 `0.1.3` 发布链。

## Scope
- In-scope:
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src/locales/messages/*.json`
  - `apps/desktop/src/locales/registry.generated.json`
  - `apps/site/{README.md,index.html,script.js,download/targets.js}`
  - `apps/site/{docs,scripts,.githooks}/**`
  - `README.md`
  - `docs/**`
  - 版本真源与 macOS 打包产物
- Out-of-scope:
  - 根目录翻译 scratch 文件
  - release 之外的新功能开发
  - 历史重写或仓库迁移

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src/locales/messages/*.json`
  - `apps/desktop/src/locales/registry.generated.json`
  - `apps/site/{README.md,index.html,script.js,download/targets.js}`
  - `apps/site/scripts/publish_site_release.py`
- Related docs/specs/logs reviewed:
  - `README.md`
  - `docs/{CHANGELOG,Architecture,CodeMap,RepositoryGuidelines}.md`
  - `apps/site/docs/ReleasePlaybook.md`
  - `docs/specs/TID-20260413-fullscreen-break-current-space-fix/*`
  - `gh release list --repo IHKYoung/Pauza`
- Why these are sufficient:
  - 已覆盖本次版本真源、release 脚本、站点下载真源、关键桌面端修复、运行时 locale 真源和既有发布状态，足以定义 `0.1.3` 的发布边界与验证路径。

## Acceptance Criteria (AC)
- AC1: 根、desktop、Tauri、Cargo 与 site pinned 链接的版本真源统一为 `0.1.3`，README / CHANGELOG / task docs 无 `TBD/INIT` 占位。
- AC2: `apps/desktop/src/locales/registry.generated.json` 已根据 50 个 locale 消息文件重建，新增 break 文案能进入运行时真源。
- AC3: 本地 macOS 打包成功并产出 `Pauza_0.1.3_aarch64.dmg`。
- AC4: GitHub `IHKYoung/Pauza` 上存在 `v0.1.3` release，且带有 `Pauza_0.1.3_aarch64.dmg` 资产。
- AC5: 当前代码仓库形成可追溯的 `0.1.3` commit，并完成 push。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: N/A
- Primary flow: N/A
- Fallback / secondary flow: N/A
- Interaction authority / ownership boundary: N/A
- Visible entrypoints / handoff cues: N/A
- In-scope interactions: N/A
- Out-of-scope interactions: N/A
- Interaction acceptance criteria: N/A
- Validator expectation: 当 `interaction_impact != none` 时，本节与 Requirement Brief 中的交互字段不得继续保留 `N/A/TBD`

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,architect,coder,tester,scribe,reality_checker
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅影响本地桌面端打包产物、站点 pinned 下载链接、release 资产与 GitHub 代码推送
- touches_running_service: no
- backup_required: no
- backup_plan: `npm test` + `npm run typecheck` + `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` + `npm --prefix apps/desktop run build` + `npm --prefix apps/desktop run tauri build -- --bundles dmg --no-sign` + `python3 scripts/validate_workflow_docs.py --mode manual`
- rollback_plan: 回退版本真源/文档/站点 pinned URL，删除或覆盖错误的 GitHub release 资产，并对错误 push 使用后续 `git revert`
- destructive_operations: none
- operator_approval_required: no
- rationale: 用户已明确要求发布 `0.1.3`，本轮不涉及历史重写、生产服务写入或提权操作

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 收口版本真源、发布文档与 site release workflow
  - DoD: `0.1.3` 版本号、README、CHANGELOG、CodeMap、Architecture、site playbook 与 task docs 全部对齐
- [x] Task-2: 重建 locale registry 并完成本地验证
  - DoD: locale registry 已重建，测试/validator/build 通过
- [x] Task-3: 打本地 macOS 包并发布 GitHub release
  - DoD: 生成 `Pauza_0.1.3_aarch64.dmg`，`v0.1.3` release 存在且资产可见
- [ ] Task-4: 生成并推送代码提交
  - DoD: commit / audit / push 完成，剩余工作树不包含本轮正式版本文件

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260420-release-013-packaging-and-publish/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading:
  - empty:
  - error:
  - disabled:
  - success:

## Observability / Debug Plan
- Logs:
  - 保留 build、tauri build、validators、`gh release view` 与 `git status` 输出到当日日志
- Error codes:
  - N/A
- Trace/metrics (optional):
  - N/A
- Debug flags (optional):
  - 打包阶段使用 `--no-sign` 以减小签名环境不确定性

## Risks & Rollback
- Risks:
  - 代码仓库与 release 仓库分离，可能导致版本追踪歧义
  - macOS dmg 构建失败或产物命名不符合脚本约定
  - GitHub release 上传成功但 site pinned URL 未同步
- Rollback plan:
  - 回退版本真源、文档和 site pinned URL
  - 对错误 release 重新上传正确资产或删除错误 tag
  - 对错误代码推送使用新的 `git revert` 提交回滚

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 统一版本真源与 release 文档，明确 `0.1.3` 边界并排除根目录翻译 scratch 文件。
  2. 重建 locale registry，纳入 `apps/site` release workflow mirror，补齐当天 plans/logs/spec。
  3. 运行测试、typecheck、Rust 测试、build 和 workflow validator。
  4. 生成本地 macOS dmg，并用 site release 脚本更新 pinned URL + 创建/上传 GitHub release。
  5. 提交并 push 当前代码仓库的 `0.1.3` 版本收口。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: 用户已在 2026-04-20 明确要求“作为 0.1.3 版本，打包本地的 macOS 并上传 GitHub 的 release 和 commit”
