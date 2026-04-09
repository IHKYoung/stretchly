# Evidence Report · TID-20260408-adaptive-reminder-state-machine

## Scope
- 本轮改动是 Tauri host 的提醒投递状态机，不涉及新的设置页布局或 break prompt 视觉调整。
- 因此证据以 Rust 单测、编译结果和状态文案覆盖为主，未采集截图。

## Interaction Contract Mapping
- Primary flow:
  - `due_break_waits_for_idle_opportunity`
  - 证明 break 到点但 `idle_ms` 仍低时，不会立刻开 break window。
- Fallback / secondary flow:
  - `pending_break_starts_when_idle_opportunity_appears`
  - 证明出现短暂停顿后，pending break 会开始。
- Visible states / transitions:
  - `soft_nudge_is_emitted_only_once_while_waiting`
  - `snapshot_status_reflects_waiting_for_opportunity`
  - 证明 waiting / soft nudge 状态可见，且不会重复提醒。

## Verification
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - PASS：`7 passed; 0 failed`
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - PASS：`Finished 'dev' profile [unoptimized + debuginfo] target(s) in 1.07s`
- `python3 scripts/validate_workflow_docs.py --mode manual`
  - PASS：`[OK] Workflow docs validation passed for 2026-04-08`

## Notes
- `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml` 在当前环境下无法执行，因为本机缺少 `rustfmt` 组件；本轮未通过网络安装额外组件。
