# Task-ID: TID-20260408-reminder-state-machine-doc

## Test Strategy
- Unit: N/A（纯文档任务）
- Integration: 通过 workflow docs validator 检查 spec / plans / logs / 索引同步完整性。
- E2E (if applicable): N/A

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 人工检查 `docs/ReminderScheduling.md` 是否定义三个用户概念及其职责边界，并明确“强制提醒吸收严格模式”。
- AC2 -> 人工检查文档是否包含 Mermaid 状态图与调度流程图。
- AC3 -> 人工检查文档是否明确写出“`breakPromptStyle` 不进入调度、第一版不含 `soft nudge`”。
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
- Artifacts path: docs/specs/TID-20260408-reminder-state-machine-doc/evidence/
- What to capture:
  - Screenshots: N/A
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y: N/A（纯文档任务）
- perf budget: N/A
- error handling / observability: 文档需明确写出调度优先级与运行时边界，避免后续实现时产生隐式状态。
- security / privacy: 仅使用已有本地输入空闲信号概念，不引入新的内容级输入采集。

## Boundary / Invalid Input Cases
- 文档不能把自然休息写成第三种提醒方式。
- 文档不能再把严格模式与强制提醒并列为两个用户设置。
- 文档不能沿用当前实验性 adaptive 版本的 soft nudge 作为第一版核心流程。

## Concurrency / Race Cases (if applicable)
- blocker、natural break 与 due 同时出现时，文档需明确优先级顺序。

## Mocks & Test Data
- N/A

## Commands to Run
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - `docs/ReminderScheduling.md` 已落盘。
  - 本任务 spec / daily plans / daily logs / CodeMap 不再保留 `TBD/INIT` 占位进入 DONE。
  - validator 输出 `[OK] Workflow docs validation passed for 2026-04-08`。
- Outputs to keep (10~20 lines snippet):
  - `python3 scripts/validate_workflow_docs.py --mode manual` 的成功输出。
