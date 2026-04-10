# Task-ID: TID-20260409-version-unify-010

## Test Strategy
- Unit: 无新增单元测试；以版本字段审计为主。
- Integration: 通过 `npm test`、`npm --prefix apps/desktop run build` 验证版本元数据变更没有破坏当前默认链路。
- E2E (if applicable): N/A。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `rg -n '^  \"version\": \"0\\.1\\.0\"' package.json package-lock.json`
- AC2 -> `rg -n '0\\.1\\.0' apps/desktop/package.json apps/desktop/src-tauri/tauri.conf.json apps/desktop/src-tauri/Cargo.toml` + `rg -n '1\\.20\\.0' package.json package-lock.json apps/desktop/package.json apps/desktop/src-tauri/tauri.conf.json apps/desktop/src-tauri/Cargo.toml`
- AC3 -> `npm test`、`npm --prefix apps/desktop run build`、`python3 scripts/validate_workflow_docs.py --mode manual`

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
- Artifacts path: docs/specs/TID-20260409-version-unify-010/evidence/
- What to capture:
  - Screenshots: N/A
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y: N/A
- perf budget: 构建通过即可
- error handling / observability: 版本 grep 与验证命令结果可追溯
- security / privacy: 不新增权限或外部调用

## Boundary / Invalid Input Cases
- 根 `package.json` 与 `package-lock.json` 不能出现一处更新、一处遗漏。
- 历史 release / archive 文本中的旧版本号不计入本任务失败条件。

## Concurrency / Race Cases (if applicable)
- N/A

## Mocks & Test Data
- N/A

## Commands to Run
- `rg -n '^  \"version\": \"0\\.1\\.0\"' package.json package-lock.json`
- `rg -n '0\\.1\\.0' apps/desktop/package.json apps/desktop/src-tauri/tauri.conf.json apps/desktop/src-tauri/Cargo.toml`
- `rg -n '1\\.20\\.0' package.json package-lock.json apps/desktop/package.json apps/desktop/src-tauri/tauri.conf.json apps/desktop/src-tauri/Cargo.toml`
- `npm test`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - 当前有效版本真源全部统一为 `0.1.0`
  - 默认测试/构建/文档 gate 均通过
- Outputs to keep (10~20 lines snippet):
  - `npm test` 摘要
  - build 摘要
  - docs validator PASS 摘要
