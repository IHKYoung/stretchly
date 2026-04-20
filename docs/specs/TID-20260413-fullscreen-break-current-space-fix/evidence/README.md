# Evidence Report

## Task
- Task-ID: TID-20260413-fullscreen-break-current-space-fix
- Title: 修复全屏工作区下 break 未覆盖当前界面

## What Was Verified
- 已核对 `apps/desktop/src-tauri/src/shell.rs` 的 macOS native overlay policy：break window 重新保留 `CanJoinAllSpaces | MoveToActiveSpace | FullScreenAuxiliary` 组合，并继续沿用 `native patch before show -> present -> focus/activate` 链路。
- 已补齐 Rust 回归测试，确保 `shell::tests::macos_break_window_native_overlay_policy_uses_expected_levels` 会守住上述 collection behavior bits。
- 已运行 Rust 测试、desktop build 与 workflow docs validator 所需的前置文档完善。

## Commands
```text
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npm --prefix apps/desktop run build
python3 scripts/validate_workflow_docs.py --mode manual
```

## Results
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - PASS
  - 关键输出：
```text
running 23 tests
test shell::tests::macos_break_window_native_overlay_policy_uses_expected_levels ... ok
test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
- `npm --prefix apps/desktop run build`
  - PASS
  - 关键输出：
```text
✓ 1826 modules transformed.
✓ built in 1.97s
```
- `python3 scripts/validate_workflow_docs.py --mode manual`
  - PASS
  - 关键输出：
```text
[OK] Workflow docs validation passed for 2026-04-13 ✅
```

## Interaction Contract Check
- Primary flow:
  - 到点后，当前 fullscreen Space 应直接看到 Pauza break prompt，而不是只在别的屏幕或后台 Space 自己开始。
- Fallback / secondary flow:
  - 非全屏场景下，现有 windowed/fullscreen break 的 show/present/focus/activate 路径保持不变。
- Visible states / transitions:
  - due -> native patch -> current Space visible overlay -> break close / finish teardown

## Reality Gap
- 当前终端环境无法自动把系统切到另一应用的真实 fullscreen Space，因此没有新增截图或录屏。
- 本轮证据以代码路径、单测与构建通过为主；最终视觉现实检查仍需用户在本机实际进入浏览器/编辑器 fullscreen 场景后再 spot-check 一次。
