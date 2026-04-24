# Task-ID: TID-20260424-structure-cleanup

## Test Strategy
- Unit:
  - Rust 单测验证 `idle_opportunity_seconds` 不再被序列化
  - Rust 单测回归现有 reminder helper 行为
- Integration:
  - `npm run typecheck`
  - `npm test`
- E2E (if applicable):
  - N/A

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `state::tests::legacy_idle_opportunity_is_not_serialized_back_out` + `npm run typecheck`
- AC2 -> `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- AC3 -> 文档人工校对 + `python3 scripts/validate_workflow_docs.py --mode manual`

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
- Artifacts path: docs/specs/TID-20260424-structure-cleanup/evidence/
- What to capture:
  - Screenshots:
  - Video/trace (optional):
  - HAR/console logs (optional):

## Quality Gates (Non-functional)
- a11y: N/A
- perf budget: 不引入新运行时开销；仅 helper 抽取与序列化边界整理
- error handling / observability: 现有 runtime action 文案保持可用
- security / privacy: 无新增权限、网络或敏感数据处理

## Boundary / Invalid Input Cases
- 旧 settings 文件仍包含 `idleOpportunitySeconds`
- 旧 settings 文件同时缺少该字段与更早期 per-break opportunity 字段

## Concurrency / Race Cases (if applicable)
- 不新增并发路径；重点回归 `tick()` 抽 helper 后的等价行为

## Mocks & Test Data
- 使用现有 Rust 单测里的默认 settings 与临时文件

## Commands to Run
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm run typecheck`
- `npm test`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - Rust 测试全部通过
  - Vitest 全部通过
  - TypeScript 无报错
  - workflow docs 校验通过
- Outputs to keep (10~20 lines snippet):
  - `28 passed`
  - `113 passed`
  - `[OK] Workflow docs validation passed`
