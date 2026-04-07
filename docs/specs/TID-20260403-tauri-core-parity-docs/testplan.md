# Task-ID: TID-20260403-tauri-core-parity-docs

## Test Strategy
- Unit:
  - N/A（docs-only）
- Integration:
  - `python3 scripts/validate_role_file_scope.py --role scribe --paths ...`
  - `python3 scripts/validate_workflow_docs.py --mode manual`
- E2E (if applicable):
  - N/A

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 读取 `docs/specs/TID-20260403-tauri-core-parity/*`，确认不再有占位模板。
- AC2 -> workflow docs validator 检查当日 logs/plans 字段完整性。
- AC3 -> 人工核对 `CHANGELOG`、`CodeMap`、`Architecture`、`UI` 的描述与代码事实一致。
- AC4 -> role scope validator 通过；workflow validator 结果被记录。

## Interaction Contract Coverage
- Interaction impact: none
- Primary flow -> tests/evidence: N/A
- Fallback / secondary flow -> tests/evidence: N/A
- Visible states / transitions -> tests/evidence: N/A

## Governance Gates
- Agent Config Validation: `python3 scripts/validate_agent_configs.py`
- Workflow Docs Validation: `python3 scripts/validate_workflow_docs.py --mode manual`
- Approval Escalation Owner: orchestrator

## False-pass Cases
- 文档任务标记 DONE，但 spec/logs/plans 仍有 `TBD/INIT`。
- role scope 越界写入未被记录。
- workflow validator 失败，但结案时不报告失败原因。

## Evidence Capture (UI / E2E)
- Required: no
- Owner: scribe
- Artifacts path: N/A
- What to capture:
  - Screenshots: N/A
  - Video/trace (optional): N/A
  - HAR/console logs (optional): validator 输出即可

## Quality Gates (Non-functional)
- a11y: N/A
- perf budget: N/A
- error handling / observability: validator 输出应被保留在日志中
- security / privacy: 仅 docs 改动

## Boundary / Invalid Input Cases
- 当日其他 task 留有占位，会导致 workflow docs validator 失败。

## Concurrency / Race Cases (if applicable)
- N/A

## Mocks & Test Data
- N/A

## Commands to Run
- `python3 scripts/validate_role_file_scope.py --role scribe --paths ...`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - role scope validator 通过
  - workflow docs validator 通过或其剩余失败项被准确记录
- Outputs to keep (10~20 lines snippet):
  - validator 关键输出
