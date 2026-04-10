# Task-ID: TID-20260410-release-011-summary-commit

## Test Strategy
- Unit:
  - N/A（版本与文档任务）
- Integration:
  - `npm test`
  - `npm --prefix apps/desktop run typecheck`
  - `npm --prefix apps/desktop run build`
  - `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - `python3 scripts/validate_workflow_docs.py --mode manual`
- E2E (if applicable):
  - N/A

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `node -p "require('./package.json').version"`、`node -p "require('./package-lock.json').version"`、`node -p "require('./apps/desktop/package.json').version"`、`rg -n '^version = \"0\\.1\\.1\"$' apps/desktop/src-tauri/Cargo.toml`
- AC2 -> 人工检查 `README.md` 与 `docs/CHANGELOG.md`
- AC3 -> `npm test`、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`、`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`python3 scripts/validate_workflow_docs.py --mode manual`
- AC4 -> `git log -1 --stat`

## Interaction Contract Coverage
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: N/A
- Fallback / secondary flow -> tests/evidence: N/A
- Visible states / transitions -> tests/evidence: N/A
- Validator expectation: 当 `interaction_impact != none` 时，本节三项与 Evidence Capture 的 `Required` 不得继续保留 `N/A/no/TBD`

## Governance Gates
- Agent Config Validation: `python3 scripts/validate_agent_configs.py`
- Workflow Docs Validation: `python3 scripts/validate_workflow_docs.py --mode manual`
- Approval Escalation Owner: orchestrator

## False-pass Cases
- `DONE` 任务对应的 spec 仍保留 `TBD/INIT` 占位。
- `orchestrator` 未显式使用 `sandbox_mode = "danger-full-access"` 与 `approval_policy = "never"`，却仍宣称当前仓库运行在 aggressive 基线。
- 代码变更前未记录 `Source Basis`，导致实现依据不可追溯。
- 子 agent 未显式 `approval_policy = "never"`。
- `moderate/complex` 任务通过缩小 `Required Roles` 伪装为 trivial fallback。
- 已使用 `single-agent-fallback`，但 logs/plans 没有单独记录 `Execution Mode` / `Fallback Scope` / `Fallback Reason Code`。
- 需要受角色边界约束的文件系统写命令没有经过 `run_role_guard.py`，只在结案时补跑范围校验。
- `git add -- <explicit paths...>` 仍被包进 wrapper / helper script，导致运行时看不到裸命令前缀。
- `interaction_impact != none`，但 plan/testplan/ui spec 没有定义 primary flow / fallback flow / visible states / evidence coverage。

## Evidence Capture (UI / E2E)
- Required: no   <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260410-release-011-summary-commit/evidence/
- What to capture:
  - Screenshots:
  - Video/trace (optional):
  - HAR/console logs (optional):

## Quality Gates (Non-functional)
- a11y:
  - N/A
- perf budget:
  - N/A
- error handling / observability:
  - docs / commit message / audit trace 完整
- security / privacy:
  - 不新增依赖、密钥或网络安装

## Boundary / Invalid Input Cases
- 版本号不能只改 root package，必须覆盖 desktop / tauri / cargo
- 历史日志里的 `0.1.0` 记录不属于本轮需要回写的历史事实

## Concurrency / Race Cases (if applicable)
- `post-commit` hook 会自动追加 `docs/commits/YYYY-MM-DD.md`；若该文件在 commit 后变脏，视为仓库审计副产物，不代表本版提交失败

## Mocks & Test Data
- N/A

## Commands to Run
- `npm test`
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - 关键测试与构建通过
  - 版本号统一为 `0.1.1`
  - 形成详细中文 commit
- Outputs to keep (10~20 lines snippet):
  - `Tests  59 passed (59)` 或更新后的通过统计
  - `✓ built in ...`
  - `test result: ok. ... passed`
