# Evidence Report

## Task
- Task-ID: TID-20260423-smart-reminder-v2-scheduling
- Scope: reminder v2 状态机、runtime status 文案、locale、测试与产品文档同步

## Interaction Contract
- Primary flow:
  - smart mode 下 break 到点后先进入 `waiting for opportunity`，若用户已离开至少 `45s` 则进入 `recovery hold`，返回时自动结算。
- Fallback / secondary flow:
  - forced mode 继续到点直接 break。
  - pause/focus/DND/app exclusion 继续作为 delivery blocker，但只冻结投递、不重置节奏。
- Visible states / transitions:
  - `heads-up`
  - `waiting for opportunity`
  - `recovery hold`
  - `recovery microbreak credited`
  - `recovery long break deferred`
  - `natural break full reset`

## Test Evidence
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - PASS: 30 passed
  - 关键覆盖：
    - `active_break_is_not_closed_by_passive_blockers`
    - `pause_and_resume_shift_due_instead_of_resetting_schedule`
    - `dnd_freezes_waiting_timer_without_reset`
    - `microbreak_is_credited_after_user_returns`
    - `long_break_is_deferred_after_user_returns`
    - `long_idle_full_reset_replans_from_now`
    - `snapshot_status_reflects_waiting_for_opportunity`
    - `snapshot_status_reflects_recovery_hold`
- `npm test`
  - PASS: 61 passed
- `npm run typecheck`
  - PASS
- `npm --prefix apps/desktop run build`
  - PASS

## Status Text Evidence
- waiting for opportunity:
  - `runtime.break.status.waitingOpportunityDetail`
  - 当前文案会明确“最多再等多久”，避免看起来像随机失效。
- recovery hold:
  - `runtime.break.status.recoveryTitle`
  - `runtime.break.status.recoveryDetail`
  - 当前文案会明确“检测到你已离开约多久，回来后会自动结算”。
- credited / deferred:
  - `runtime.actions.recoveryMicrobreakCredited`
  - `runtime.actions.recoveryLongBreakDeferred`
  - 当前文案会把离开时长和顺延时长写入 `last_action`。

## Conclusion
- AC1: PASS
- AC2: PASS
- AC3: PASS
- AC4: PASS
