# Task-ID: TID-20260423-smart-reminder-v2-scheduling

## Test Strategy
- Unit:
  - `state.rs` 单测覆盖 blocker freeze、recovery credit、waiting budget、active break priority。
- Integration:
  - 通过 `cargo test` 验证 pause/focus/DND/app exclusion 与 reminder state machine 的组合路径。
- E2E (if applicable):
  - 本轮不新增 UI 自动化；以 snapshot/status 与 locale 文案、build、自证 evidence 为主。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `active_break_is_not_closed_by_passive_blockers`
- AC2 -> `pause_and_resume_shift_due_instead_of_resetting_schedule`、`dnd_freezes_waiting_timer_without_reset`
- AC3 -> `microbreak_is_credited_after_user_returns`、`long_break_is_deferred_after_user_returns`、`long_idle_full_reset_replans_from_now`
- AC4 -> `snapshot_status_reflects_waiting_for_opportunity`、`snapshot_status_reflects_recovery_hold`

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->

### Primary flow -> tests/evidence
`cargo test` 覆盖 waiting for opportunity、recovery hold、microbreak credited、long break deferred、full reset；evidence README 记录对应状态文本与测试结果。

### Fallback / secondary flow -> tests/evidence
forced mode 既有立即开 break 语义保持不变；pause/focus/DND/app exclusion freeze 路径由新增单测覆盖，验证解除阻塞后是平移恢复而不是整轮 reset。

### Visible states / transitions -> tests/evidence
`snapshot_status_reflects_waiting_for_opportunity` 与 `snapshot_status_reflects_recovery_hold` 覆盖 waiting budget、recovery hold 两个主要用户可见态；evidence README 额外对照 last_action 中的 credited/deferred/reset 说明。

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
- Required: partial   <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260423-smart-reminder-v2-scheduling/evidence/
- What to capture:
  - Screenshots: 非必须；如需可补 settings runtime status 文案截图
  - Video/trace (optional): N/A
  - HAR/console logs (optional): 记录关键测试命令与 snapshot/status 证据摘录

## Quality Gates (Non-functional)
- a11y:
  - 不新增交互控件，不改变键盘路径
- perf budget:
  - 不新增高频探测、额外线程或新依赖
- error handling / observability:
  - `last_action` / `status_detail` 必须覆盖 waiting / recovery / blocker 解释
- security / privacy:
  - 继续只使用本地 idle / DND / app exclusion 信号，不新增权限

## Boundary / Invalid Input Cases
- blocker 重叠（例如 pause + DND）
- due 已过但仍在等待时进入 / 离开 blocker
- idle gap 既满足 recovery credit 又接近 natural break full reset
- forced mode 下 recovery credit 不得改变既有立即开 break 语义

## Concurrency / Race Cases (if applicable)
- 命令路径（pause/resume/focus）与后台 tick 共享同一 `RuntimeState`；due 平移不能重复执行。
- wait timer 进入 blocker 后恢复，blocked duration 不得计入 smart wait deadline。

## Mocks & Test Data
- 直接使用 `PauzaSettings::default()` 与手动构造的 `RuntimeState`/`CurrentBreak`。
- 通过固定 `now` / `idle_ms` 模拟 due、block、resume 与 recovery gap。

## Commands to Run
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm test`
- `npm run typecheck`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`
- `python3 scripts/validate_agent_configs.py`

## Expected Results
- PASS criteria:
  - Rust reminder 状态机相关单测全部通过
  - 前端测试 / typecheck / build 与 workflow docs validator 通过
  - evidence README 能回溯 AC 对应的关键状态文本与测试结果
- Outputs to keep (10~20 lines snippet):
  - `cargo test` 中与 reminder v2 相关测试的 PASS 摘要
  - workflow docs validator 的 `[OK]`
