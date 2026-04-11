# Task-ID: TID-20260411-release-011-archive-commit

## Summary
- Title: 归档当前 0.1.1 工作树并提交
- Date: 2026-04-11
- Level: moderate
- Lane: deep
- Execution Profile: merge-gate
- Status: DONE

## Requirement Brief
- Goal restatement: 将当前所有未提交改动整体视为 `0.1.1` 版本线上的最新归档快照，补齐与当前真实文件树一致的版本文档，并完成一次详细的本地提交。
- In-scope:
  - 当前 worktree 全量差异的归档与提交边界确认
  - `README.md`、`docs/{RepositoryGuidelines,CodeMap,Architecture,UI,CHANGELOG}.md` 的真源对齐
  - 本任务 spec、当天 plans/logs 与 commit message / audit 路径
  - 本地验证命令与 `git commit`
- Out-of-scope:
  - 额外功能开发
  - 版本号提升到 `0.1.2+`
  - 远端 push、tag、release 发布动作
- Assumptions:
  - 当前版本号真源已经统一为 `0.1.1`，不需要再改版本字段
  - 用户要的是“一次完整归档提交”，而不是继续拆小 commit
  - 旧 Electron `app/` 目录的删除是本次快照的一部分，应如实归档
- Risks:
  - 若文档继续把 `app/` 写成现存目录，归档后的仓库索引会与实际文件树脱节
  - 若 staged paths 对应多个 task spec，但 commit message 没按 `TASK-ID-MULTIPLE` 书写，会被 hook 阻断
  - post-commit hook 会继续追加审计记录，需要接受该自动流程
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 将当前工作树收口成一条可追溯的 `0.1.1` 版本归档提交，且提交前后文档、校验与审计链一致。

## Scope
- In-scope:
  - `app/**` 删除、`apps/desktop/src/{App.tsx,lib/break-copy-layout.ts}`、`apps/desktop/src-tauri/src/shell.rs`
  - `apps/site/**` 当前未提交改动
  - `test/desktopBreakCopyLayout.js`
  - `docs/PauzaV1SixStepPlan.md`
  - 当天新增/未跟踪的 task specs
  - `README.md`、`docs/{RepositoryGuidelines,CodeMap,Architecture,UI,CHANGELOG,plans,logs,commits}` 与本任务 spec
- Out-of-scope:
  - 新增依赖、网络安装、部署与推送远端
  - 改写既有已提交历史

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `git status --short`
  - `git diff --stat`
  - `README.md`
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `docs/Architecture.md`
  - `docs/UI.md`
  - `docs/CHANGELOG.md`
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src-tauri/src/shell.rs`
- Related docs/specs/logs reviewed:
  - `docs/plans/2026-04-11.md`
  - `docs/logs/2026-04-11.md`
  - `docs/commits/2026-04-11.md`
  - `docs/specs/TID-20260410-release-011-summary-commit/*`
  - `.githooks/{commit-msg,post-commit}`
  - `scripts/{validate_commit_message_task_id.py,commit_audit_lib.py,scaffold_task.py}`
- Why these are sufficient:
  - 已覆盖当前版本真源、文件树索引、已有 `0.1.1` 收口文档、当天 task 闭环记录与本次 commit hooks 约束，足以定义当前归档边界和提交要求。

## Acceptance Criteria (AC)
- AC1: 当前版本号仍统一为 `0.1.1`，且本次不再额外改写版本字段。
- AC2: `README.md`、`docs/RepositoryGuidelines.md`、`docs/CodeMap.md`、`docs/Architecture.md`、`docs/UI.md` 与实际文件树一致，不再把已删除的 `app/` 目录写成现存入口。
- AC3: 当前 worktree 的关键验证通过，包括测试、类型检查、桌面端构建、Rust 测试、workflow docs validator 与 diff 体检。
- AC4: 形成一条详细中文本地 commit，能够把当前 worktree 作为 `0.1.1` 版本归档快照收口。

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
- Execution Profile: merge-gate
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,architect,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限本地版本归档、文档真源收口与 git commit
- touches_running_service: no
- backup_required: no
- backup_plan: `git diff --stat` + `git diff --check` + `npm test` + `npm run typecheck` + `npm --prefix apps/desktop run build` + `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` + `python3 scripts/validate_workflow_docs.py --mode manual`
- rollback_plan: 如需撤回，以本次 commit 为边界执行后续 revert；提交前以 diff 和 log 复核范围
- destructive_operations: git commit（用户已明确授权）
- operator_approval_required: no
- rationale: 纯本地 merge-gate 收口，不涉及运行中服务、数据迁移、外部副作用、提权或新增成本

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 盘点当前 worktree 变更边界、版本真源与 commit hook 约束
  - DoD: 明确本次归档不再 bump 版本号，并确认 commit message / docs audit 所需格式。
- [x] Task-2: 收口版本文档真源，使索引与实际文件树一致
  - DoD: `README.md`、`RepositoryGuidelines.md`、`CodeMap.md`、`Architecture.md`、`UI.md`、`CHANGELOG.md` 不再把 `app/` 目录描述成现存模块，且 README 的下一阶段口径更新为官网部署/下载链路。
- [x] Task-3: 完成本地验证并执行版本归档 commit
  - DoD: 关键测试与 validators 通过，形成一条详细中文 commit。

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260411-release-011-archive-commit/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading:
  - empty:
  - error:
  - disabled:
  - success:

## Observability / Debug Plan
- Logs: 通过当日 `docs/plans/2026-04-11.md`、`docs/logs/2026-04-11.md` 与 `docs/commits/2026-04-11.md` 追踪归档边界、验证与提交审计
- Error codes: N/A（本任务是本地归档/提交流程，不新增运行时错误码）
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 文档真源若未同步改写，会把已删除目录继续暴露成维护入口
  - 本次 commit 覆盖多个 task spec，commit message 稍有偏差就会被 hook 阻断
  - post-commit hook 会继续自动补记审计，属于预期副作用
- Rollback plan:
  - 若提交前校验失败，先修正文档/commit message 再重试
  - 若提交后需要撤回，基于该 commit hash 执行后续 revert，而不是重写历史

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 盘点当前 worktree、版本真源、已有 task specs 与 commit hook 约束。
  2. 修正文档真源，使 README / docs 索引与实际文件树及当前阶段口径一致。
  3. 运行测试、构建、validator 与 diff 体检。
  4. 暂存当前 worktree，并以详细中文 commit message 完成 `0.1.1` 归档提交。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: 用户已在 2026-04-11 明确要求“将目前的所有修改作为一个 0.1.1 的版本归档和 commit”
