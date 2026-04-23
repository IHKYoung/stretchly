# Task-ID: TID-20260423-release-014-macos-package-commit

## Summary
- Title: 收口 0.1.4 并打包 macOS 安装包
- Date: 2026-04-23
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 把当前未提交的 desktop/docs 变更整理成 `0.1.4`，统一版本真源与发布说明，验证当前 release 边界，打当前机器架构的 macOS 安装包，并完成主提交与审计提交。
- In-scope:
  - 统一 root / desktop / Tauri / Cargo / lockfile 版本真源到 `0.1.4`
  - 更新 `README.md`、`docs/CHANGELOG.md` 与本任务 workflow 文档
  - 纳入 `TID-20260423-smart-reminder-v2-scheduling` 已完成的代码与文档改动
  - 重建 desktop locale registry 并完成 release 边界验证
  - 构建当前机器架构的本地 macOS `.app` / `.dmg`
  - 生成主提交与 audit-only follow-up commit
- Out-of-scope:
  - 上传 GitHub release、更新 site pinned 下载链接或执行远端 push
  - 新增 UI、调度或平台能力
  - 历史重写
- Assumptions:
  - 当前机器可产出无签名 arm64 macOS 安装包
  - 当前工作树里的未提交代码/文档改动都属于 `0.1.4` 正式版本边界
  - `post-commit` hook 会自动追加 `docs/commits/*.md`，需要 follow-up audit-only commit 落盘
- Risks:
  - 版本真源漏改会让 `0.1.4` 版本边界不一致
  - `tauri build` 若走签名路径可能因本地证书环境缺失失败
  - 主提交跨多个 Task-ID，若 commit message 归因不正确会被 hook 阻断
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 将当前有效工作树收口成一条可追溯、可构建、可提交的 `0.1.4` 本地 macOS 版本链。

## Scope
- In-scope:
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src/locales/messages/{zh-CN,en}.json`
  - `apps/desktop/src/locales/registry.generated.json`
  - `package.json`
  - `package-lock.json`
  - `apps/desktop/package.json`
  - `apps/desktop/package-lock.json`
  - `apps/desktop/src-tauri/{Cargo.toml,Cargo.lock,tauri.conf.json}`
  - `README.md`
  - `docs/{CHANGELOG,CodeMap,Architecture,ReminderScheduling,SettingsInventory,UI}.md`
  - `docs/logs/2026-04-23.md`
  - `docs/plans/2026-04-23.md`
  - `docs/specs/TID-20260423-{smart-reminder-v2-scheduling,release-014-macos-package-commit}/**`
- Out-of-scope:
  - `apps/site/**`
  - GitHub release / pinned 下载链接更新
  - 本轮版本之外的新功能

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `package.json`
  - `package-lock.json`
  - `apps/desktop/package.json`
  - `apps/desktop/package-lock.json`
  - `apps/desktop/src-tauri/{Cargo.toml,Cargo.lock,tauri.conf.json}`
  - `README.md`
  - `.githooks/post-commit`
  - `scripts/record_commit_audit.py`
- Related docs/specs/logs reviewed:
  - `docs/{RepositoryGuidelines,CodeMap,CHANGELOG}.md`
  - `docs/logs/2026-04-23.md`
  - `docs/plans/2026-04-23.md`
  - `docs/specs/TID-20260420-release-013-packaging-and-publish/*`
  - `docs/specs/TID-20260423-smart-reminder-v2-scheduling/*`
- Why these are sufficient:
  - 已覆盖当前版本真源、打包入口、commit audit 机制、上一轮 release 经验与本次要纳入的 reminder v2 工作树，足以定义 `0.1.4` 的发布边界、验证路径与提交方式。

## Acceptance Criteria (AC)
- AC1: 根、desktop、Tauri、Cargo 与 lockfile 的版本真源统一为 `0.1.4`，README / CHANGELOG / task docs 无 `TBD/INIT` 占位。
- AC2: `apps/desktop/src/locales/registry.generated.json` 已按当前 locale 真源重建，并且 `npm test`、`npm run typecheck`、`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`npm --prefix apps/desktop run build`、`python3 scripts/validate_workflow_docs.py --mode manual` 全部通过。
- AC3: 本地 macOS 打包成功并产出 `Pauza.app` 与 `Pauza_0.1.4_aarch64.dmg`。
- AC4: 当前仓库形成一条可追溯的 `0.1.4` 主提交，以及一条仅包含 `docs/commits/**` 的审计 follow-up 提交。

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
- service_impact: 仅影响本地版本真源、desktop 打包产物、release 文档与 git 本地提交
- touches_running_service: no
- backup_required: no
- backup_plan: `python3 scripts/sync_desktop_locales.py`、`npm test`、`npm run typecheck`、`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`npm --prefix apps/desktop run build`、`npm --prefix apps/desktop run tauri build -- --bundles app,dmg --no-sign`、`python3 scripts/validate_workflow_docs.py --mode manual`
- rollback_plan: 提交前回退版本真源/文档/打包产物；提交后对错误版本使用新的 `git revert`
- destructive_operations: none
- operator_approval_required: no
- rationale: 用户已明确要求本地打包和提交，本轮不涉及历史重写、线上服务、提权或付费外部动作。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 收口版本真源与 release 文档
  - DoD: `0.1.4` 版本号、README、CHANGELOG 与本任务 workflow docs 全部对齐
- [x] Task-2: 重建 locale registry 并完成 release 边界验证
  - DoD: locale registry 重建成功，测试 / typecheck / build / workflow validator 全部通过
- [x] Task-3: 生成本地 macOS 包并完成提交闭环
  - DoD: 产出 `Pauza.app` 与 `Pauza_0.1.4_aarch64.dmg`，形成主提交与 audit-only follow-up commit

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260423-release-014-macos-package-commit/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading:
  - empty:
  - error:
  - disabled:
  - success:

## Observability / Debug Plan
- Logs:
  - 保留 `sync_desktop_locales`、测试、build、tauri build、validator、`git status` 与 `git log` 输出到当日日志
- Error codes:
  - N/A
- Trace/metrics (optional):
  - N/A
- Debug flags (optional):
  - 打包阶段显式使用 `--no-sign`

## Risks & Rollback
- Risks:
  - 版本真源漏改会让 `0.1.4` 版本边界不一致
  - `tauri build` 产物命名不符合预期，导致 release 文档与实际不匹配
  - 主提交跨多个 Task-ID，若 commit message 未使用 `TASK-ID-MULTIPLE` 会被 hook 阻断
- Rollback plan:
  - 回退版本真源、README、CHANGELOG 与本任务 docs
  - 删除错误的本地产物后重新构建
  - 对错误提交使用新的 `git revert`

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 统一 `0.1.4` 版本真源与 release 文档，明确本轮边界不包含外部 release/push。
  2. 重建 locale registry，并完成测试、typecheck、Rust 测试、desktop build 与 workflow validator。
  3. 生成本地 arm64 macOS `.app` / `.dmg`。
  4. 提交主版本收口，再补 audit-only follow-up commit 落盘 `docs/commits/**`。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: 用户已在 2026-04-23 明确要求“以 0.1.4 做一个 macOS 的编译打包，以及 commit”
