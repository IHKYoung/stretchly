# Task-ID: TID-20260410-pre-break-heads-up-restoration

## Evidence Summary
- 结论：提前提示并非断线，而是原实现只保留一次性系统通知，缺少持续可见的运行态 cue，因此用户体感近似“没有用到”。本轮已将其恢复为显式 heads-up 状态，并保留系统通知作为辅助通道。

## Source Evidence
- 链路核查：
  - `apps/desktop/src/App.tsx` 仍暴露 `ui.preBreakNotifications`
  - `apps/desktop/src-tauri/src/state.rs` 仍保留 `microbreak_notification_*` / `long_break_notification_*`、`next_notification_due_ms`
  - `apps/desktop/src-tauri/src/engine.rs` 仍会调用 `notification().show()`
- 关键修正：
  - `state.rs` 新增 `heads_up_kind()` 和 `pre_break_heads_up_detail()`
  - `status()` 现在可在 due 前返回 `headsUpTitle` / `headsUpDetail|headsUpAdaptiveDetail`
  - `engine.rs` 在通知失败时输出 `failed to show desktop notification: ...`

## Verification Commands
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - PASS
  - 关键输出：
    - `test state::tests::snapshot_status_reflects_pre_break_heads_up ... ok`
    - `test state::tests::snapshot_status_reflects_waiting_for_opportunity ... ok`
    - `test result: ok. 23 passed; 0 failed`
- `python3 scripts/sync_desktop_locales.py`
  - PASS
  - 关键输出：
    - `[OK] Built desktop locale registry: 50 languages, registry -> apps/desktop/src/locales/registry.generated.json`
- `npm --prefix apps/desktop run build`
  - PASS
  - 关键输出：
    - `✓ 1825 modules transformed.`
    - `✓ built in 1.82s`

## Gaps / Reality Check
- tray 和系统通知属于操作系统表面，当前没有稳定的自动化采集入口。
- 因此本任务的最终现实确认仍建议由用户本机观察一次：
  - due 前是否看到 `即将开始 / Up next`
  - 若系统允许通知，是否仍收到一次辅助通知
  - due 后 smart 模式是否正确切到 `等待空档`
