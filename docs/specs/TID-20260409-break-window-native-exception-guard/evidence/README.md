# Task-ID: TID-20260409-break-window-native-exception-guard Evidence

## Summary
- 证据目标：证明 macOS break window 的原生 patch 不再因为 Objective-C exception 直接触发 Rust foreign exception abort。
- 采集时间：2026-04-09
- 采集方式：crash report 分析 + 代码路径核对 + Rust/前端构建链 + workflow validator

## Available Evidence
- Crash reports:
  - `~/Library/Logs/DiagnosticReports/pauza-desktop-2026-04-09-224258.ips`
  - `~/Library/Logs/DiagnosticReports/pauza-desktop-2026-04-09-232038.ips`
- 构建验证：
  - `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - `npm --prefix apps/desktop run build`
  - `python3 scripts/validate_workflow_docs.py --mode manual`
  - `python3 scripts/validate_agent_configs.py`
- 代码路径：
  - `apps/desktop/src-tauri/src/shell.rs`

## AC Coverage
- AC1:
  - 两份 crash report 的主线程栈都包含 `_rust_foreign_exception`、`objc2::runtime::message_receiver::msg_send_primitive::send` 和 `msg_send_retained` 路径，支持“ObjC exception 穿透到 Rust event loop 导致 abort”的根因判断。
  - 当前 `shell.rs` 引入 `run_macos_native_break_window_patch()`，将 `ns_window()` 与原生 selector 调用统一包进 exception guard，异常时只输出 `eprintln!` 并返回 `Ok(())`。
- AC2:
  - `show_break_window()` 现在先执行 `window.show()`，再调用 `configure_break_window_native_behavior()` 和 `present_break_window()`。
- AC3:
  - exception 分支统一降级为 no-op；实现层不再允许 native patch 失败直接终止进程。
- AC4:
  - `cargo check` 通过。
  - `cargo test` 通过，关键输出含 `test shell::tests::macos_break_window_native_overlay_policy_uses_expected_levels ... ok` 与 `test result: ok. 22 passed; 0 failed`。
  - `npm --prefix apps/desktop run build` 通过，关键输出含 `✓ 1825 modules transformed.` 与 `✓ built in 13.96s`。
  - workflow docs validator 与 agent config validator 通过。

## Gaps
- 当前没有可稳定编排的 Tauri 原生“强制进入 break”自动化入口，因此还缺少一次新的本机运行时截图或录像。
- 最终现实确认仍需用户在 macOS 本机真实进入一次 break，确认现在表现为“正常进入或降级显示”，而不是直接崩溃。
