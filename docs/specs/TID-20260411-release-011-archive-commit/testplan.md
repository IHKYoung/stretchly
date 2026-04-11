# Task-ID: TID-20260411-release-011-archive-commit

## Test Strategy
- Unit: 复用当前已有 Vitest 用例，确认 `App.tsx` / `break-copy-layout.ts` 相关改动未引入回归。
- Integration: 通过 `npm run typecheck`、`npm --prefix apps/desktop run build` 与 `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` 验证前台、桌面宿主与 Rust 侧闭环。
- E2E (if applicable): 本任务不额外新增 E2E；官网与 break prompt 的现实检查沿用当天既有 task 日志与 evidence。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `node -p "require('./package.json').version"`、`node -p "require('./apps/desktop/package.json').version"`、`rg -n '^version = \"0\\.1\\.1\"$' apps/desktop/src-tauri/Cargo.toml`、`rg -n '\"version\"\\s*:\\s*\"0\\.1\\.1\"' apps/desktop/src-tauri/tauri.conf.json`
- AC2 -> `rg -n '旧 Electron 壳已从当前工作树移除|仅维护两块内容|官网部署与下载链路' README.md docs/RepositoryGuidelines.md docs/CodeMap.md docs/Architecture.md docs/UI.md`
- AC3 -> `git diff --check`、`npm test`、`npm run typecheck`、`npm --prefix apps/desktop run build`、`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`python3 scripts/validate_workflow_docs.py --mode manual`
- AC4 -> `git commit`（commit-msg / post-commit hooks 应通过）、`git log -1 --stat`

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
- Artifacts path: docs/specs/TID-20260411-release-011-archive-commit/evidence/
- What to capture:
  - Screenshots: N/A
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y: N/A（本任务不新增 UI 交互）
- perf budget: 不应引入新的构建错误或明显扩大现有 desktop build 告警范围
- error handling / observability: docs / commit / audit 链路需可追溯；workflow docs validator 必须通过
- security / privacy: 不新增依赖、不执行网络安装、不推送远端

## Boundary / Invalid Input Cases
- `README` / `CodeMap` / `Architecture` 若仍保留“`app/` 目录存在”描述，视为归档失败
- commit message 若未使用 `Task-ID: TASK-ID-MULTIPLE` 与完整 `Related Task-IDs`，视为 hook 拦截的预期失败
- staged changes 若遗漏 `docs/CHANGELOG.md` 或新增 task specs，视为提交边界不完整

## Concurrency / Race Cases (if applicable)
- N/A（本任务是本地单次归档提交流程）

## Mocks & Test Data
- N/A

## Commands to Run
- `git diff --check`
- `node -p "require('./package.json').version"`
- `node -p "require('./apps/desktop/package.json').version"`
- `rg -n '^version = \"0\\.1\\.1\"$' apps/desktop/src-tauri/Cargo.toml`
- `rg -n '\"version\"\\s*:\\s*\"0\\.1\\.1\"' apps/desktop/src-tauri/tauri.conf.json`
- `rg -n '旧 Electron 壳已从当前工作树移除|仅维护两块内容|官网部署与下载链路' README.md docs/RepositoryGuidelines.md docs/CodeMap.md docs/Architecture.md docs/UI.md`
- `npm test`
- `npm run typecheck`
- `npm --prefix apps/desktop run build`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `python3 scripts/validate_workflow_docs.py --mode manual`
- `git log -1 --stat`

## Expected Results
- PASS criteria:
  - 版本真源仍统一输出 `0.1.1`
  - 关键测试、构建和 validator 全部通过
  - 最终生成一条详细中文 commit，能够作为当前 `0.1.1` worktree 的归档快照
- Outputs to keep (10~20 lines snippet):
  - `npm test` 的通过统计
  - `cargo test` 的通过统计
  - `python3 scripts/validate_workflow_docs.py --mode manual` 的 `[OK]`
