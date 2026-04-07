# Task-ID: TID-20260406-settings-inventory-doc

## Test Strategy
- Unit: N/A
- Integration: 校验新文档、CodeMap 与 workflow docs 门禁的一致性。
- E2E (if applicable): N/A

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 人工核对 `docs/SettingsInventory.md` 的四层结构是否完整。
- AC2 -> `rg -n "SettingsInventory\\.md" docs/CodeMap.md`
- AC3 -> 搜索本任务 docs 不保留 `TBD/INIT` 占位。
- AC4 -> `python3 scripts/validate_workflow_docs.py --mode manual`

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
- 把旧版 Electron 候选项误标记成“当前 Tauri 已支持”。
- 把运行时动作误标记成“设置项”。

## Evidence Capture (UI / E2E)
- Required: no   <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260406-settings-inventory-doc/evidence/
- What to capture:
  - Screenshots: N/A
  - Video/trace (optional): N/A
  - HAR/console logs (optional): validator 输出

## Quality Gates (Non-functional)
- a11y: N/A
- perf budget: N/A
- error handling / observability: workflow validator 通过。
- security / privacy: 仅 docs 改动，不新增权限或敏感数据。

## Boundary / Invalid Input Cases
- 不把运行时动作误写成设置。
- 不把旧版候选项误写成“当前 Tauri 已支持”。

## Concurrency / Race Cases (if applicable)
- N/A

## Mocks & Test Data
- N/A

## Commands to Run
- `rg -n "^## " docs/SettingsInventory.md`
- `rg -n "SettingsInventory\\.md" docs/CodeMap.md`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - `docs/SettingsInventory.md` 存在且结构完整。
  - `docs/CodeMap.md` 已登记新文档。
  - workflow docs validator 通过。
- Outputs to keep (10~20 lines snippet):
  - `[OK] Workflow docs validation passed for 2026-04-06 ✅`
