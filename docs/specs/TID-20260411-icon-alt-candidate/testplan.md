# Task-ID: TID-20260411-icon-alt-candidate

## Test Strategy
- Unit:
  - N/A（静态素材）
- Integration:
  - `sips -g pixelWidth -g pixelHeight -g hasAlpha apps/desktop/src-tauri/icons/icon-alt-nw45.png`
  - `magick identify -format '%f %wx%h %[channels]\n' apps/desktop/src-tauri/icons/icon-alt-nw45.png`
  - `python3 scripts/validate_workflow_docs.py --mode manual`
- E2E (if applicable):
  - N/A

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 检查 `apps/site/favicon.svg` diff，确认背景矩形已移除
- AC2 -> 检查 `apps/desktop/src-tauri/icons/icon-alt-nw45.svg` 与 `.png` 存在
- AC3 -> `sips` + `identify` 校验 `1024x1024` 与 `hasAlpha: yes`
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
- Artifacts path: docs/specs/TID-20260411-icon-alt-candidate/evidence/
- What to capture:
  - Screenshots: N/A
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y: N/A（静态素材）
- perf budget: N/A
- error handling / observability: PNG 必须保留 alpha
- security / privacy: 无敏感数据

## Boundary / Invalid Input Cases
- 若 PNG 导出失败，则保留 SVG 候选文件并记录失败原因

## Concurrency / Race Cases (if applicable)
- N/A

## Mocks & Test Data
- `icon-alt-nw45.svg`

## Commands to Run
- `sips -g pixelWidth -g pixelHeight -g hasAlpha apps/desktop/src-tauri/icons/icon-alt-nw45.png`
- `magick identify -format '%f %wx%h %[channels]\n' apps/desktop/src-tauri/icons/icon-alt-nw45.png`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - 候选 PNG 为 `1024x1024`
  - `hasAlpha: yes`
  - docs validator 通过
- Outputs to keep (10~20 lines snippet):
  - `pixelWidth: 1024`
  - `pixelHeight: 1024`
  - `hasAlpha: yes`
  - `icon-alt-nw45.png 1024x1024 srgba 4.0`

## Observed Results
- `sips` 校验通过：`pixelWidth: 1024`、`pixelHeight: 1024`、`hasAlpha: yes`
- `magick identify` 校验通过：`icon-alt-nw45.png 1024x1024 srgba 4.0`
- workflow docs validator 通过
