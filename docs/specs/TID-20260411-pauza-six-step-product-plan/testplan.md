# Task-ID: TID-20260411-pauza-six-step-product-plan

## Test Strategy
- Unit: N/A（纯文档任务）
- Integration: 文档内容审查，确认六个步骤都包含目标、修改内容、涉及模块与完成标志
- E2E (if applicable): `python3 scripts/validate_workflow_docs.py --mode manual`

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 审查 `docs/PauzaV1SixStepPlan.md` 的 6 个 `Step` 章节
- AC2 -> 审查每个 `Step` 中的“具体修改内容 / 涉及模块 / 完成标志”
- AC3 -> `python3 scripts/validate_workflow_docs.py --mode manual`

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
- Artifacts path: docs/specs/TID-20260411-pauza-six-step-product-plan/evidence/
- What to capture:
  - Screenshots: N/A
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y: 文档标题层级清晰，便于后续直接引用
- perf budget: N/A
- error handling / observability: 不新增运行时代码；daily docs 可追溯
- security / privacy: 不引入外部数据或敏感信息

## Boundary / Invalid Input Cases
- 文档不得只给原则，不给具体模块和修改内容
- 文档不得把“六步”写成含糊的时间预估而失去执行边界

## Concurrency / Race Cases (if applicable)
- 不适用；纯文档任务

## Mocks & Test Data
- 使用当前仓库中的官网、桌面端、locale 与 docs 真源作为分析输入

## Commands to Run
- `rg -n "^## Step [1-6]" docs/PauzaV1SixStepPlan.md`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - `docs/PauzaV1SixStepPlan.md` 中检测到 6 个步骤标题
  - workflow docs validator 通过
- Outputs to keep (10~20 lines snippet):
  - `docs/PauzaV1SixStepPlan.md:## Step 1 ... ## Step 6`
  - `[OK] Workflow docs validation passed for 2026-04-11 ✅`
