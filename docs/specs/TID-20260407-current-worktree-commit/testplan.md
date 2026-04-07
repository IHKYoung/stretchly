# Task-ID: TID-20260407-current-worktree-commit

## Test Strategy
- Unit: N/A（本任务不新增业务逻辑单测）
- Integration: 通过 `git diff --cached --stat`、commit hook 规则、workflow docs gate 和多语言/构建验证确认提交边界正确。
- E2E (if applicable): N/A

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 检查 `git diff --cached --name-only` 不再包含 `.playwright-mcp/`、`output/playwright/`、`scripts/__pycache__/` 和空白 `Untitled`；同时搜索 docs 仅引用 `docs/specs/**/evidence/`。
- AC2 -> 检查本任务和 `TID-20260407-rhythm-chip-presets` 的 spec / plans / logs 不再保留 `INIT/TBD`。
- AC3 -> 运行 `npm test`、`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`、`python3 scripts/validate_workflow_docs.py --mode manual`，并完成主提交与 audit commit。

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
- Owner: N/A
- Artifacts path: docs/specs/TID-20260407-current-worktree-commit/evidence/
- What to capture:
  - Screenshots: N/A
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y: N/A
- perf budget: 不新增依赖，不扩大最终提交的缓存噪音。
- error handling / observability: commit hook 与 workflow docs validator 可机械阻断错误归因或缺失字段。
- security / privacy: 不引入外部请求、密钥或权限调整。

## Boundary / Invalid Input Cases
- commit message 若少写 `TASK-ID-MULTIPLE` 或 `Related Task-IDs` 集合不全，应被 hook 直接拒绝。
- audit-only follow-up commit 不应再次触发 commit audit 递归追加。

## Concurrency / Race Cases (if applicable)
- 不适用；本任务是本地单用户 Git / 文档整理流程。

## Mocks & Test Data
- 使用当前实际 staged diff、Git hooks 与仓库脚本，无需额外 mock。

## Commands to Run
- `git diff --cached --stat`
- `npm test`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_agent_configs.py`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - staged diff 不含原始缓存；
  - workflow docs gate 通过；
  - 主提交与 audit-only 提交完成后工作区干净。
- Outputs to keep (10~20 lines snippet):
  - `npm test` 的 `18 passed / 319 passed`
  - `cargo test` 的 `3 passed; 0 failed`
  - `vite build` 的 `1820 modules transformed / built in 1.39s`
