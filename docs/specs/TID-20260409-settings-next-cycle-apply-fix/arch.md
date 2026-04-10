# Task-ID: TID-20260409-settings-next-cycle-apply-fix

## Goals
- 将设置更新的生效边界改为“从下一轮休息开始”，而不是直接重置当前状态机。

## Non-Goals
- 不修改显式 `reset_breaks`、`pause`、`focus` 等动作的语义。

## Constraints & Assumptions
- 当前 break 的时长、标题和 strict/manual 状态在开始时已经固化在 `CurrentBreak` 中。

## System Boundaries
- Modules:
  - `apps/desktop/src-tauri/src/state.rs`
- Ownership:
  - `update_settings()`、运行中 schedule 状态、对应单测

## Invariants
- 正在进行中的 break 不应因设置保存被清空。
- 已经排队的下一次 break 不应因设置保存被改写 due time 或 kind。
- 新 settings 仍需在下一轮 `schedule_next_slot()` 中生效。

## Risks & Rollback
- Failure modes:
  - settings 更新后没有 future schedule 可用
- Rollback steps:
  - 回退 `update_settings()` 与新增测试

## Acceptance Criteria (System)
- 调整设置时当前 break 持续运行。
- 调整设置时已排队 next break 保持原计划。
