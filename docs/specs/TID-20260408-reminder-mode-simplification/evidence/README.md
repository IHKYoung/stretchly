# Evidence Report

- Task-ID: `TID-20260408-reminder-mode-simplification`
- Evidence type: 代码路径 + 自动化验证

## Covered Interaction Contract
- Primary flow:
  - 设置页只暴露 `智能提醒 / 强制提醒 / 自然休息`
  - `智能提醒` 到点后等待空档
  - `强制提醒` 到点直接严格开始 break
- Fallback / secondary flow:
  - `自然休息` 继续只做长时间离开后的重置
  - DND / app exclusion / focus / pause 阻塞逻辑保持优先级
- Visible states:
  - `WaitingForOpportunity`
  - `BreakActive`
  - `NaturalBreak`

## Evidence
- Rust tests:
  - `forced_mode_starts_break_immediately`
  - `forced_mode_disables_postpone_and_skip`
  - `due_break_waits_for_idle_opportunity`
  - `pending_break_starts_when_idle_opportunity_appears`
  - `legacy_strict_settings_migrate_to_forced_mode`
  - `platform::tests::idle_signal_is_available_even_without_natural_breaks`
- Frontend build:
  - `npm --prefix apps/desktop run build`

## Notes
- 本轮没有采集截图或录屏；证据以 host 状态机单测、前端类型/构建与 workflow docs validator 为主。
