# Evidence Report

- Task-ID: TID-20260411-break-activation-settings-input-fix
- Date: 2026-04-11
- Scope: AC1 / AC2 / AC3 / AC4

## Evidence Summary
- AC1（macOS 全屏 Space break 可见性）：
  - 代码路径已补到 `apps/desktop/src-tauri/src/shell.rs` 的 `show_break_window()`：break window `present` 后，focusable 路径继续走 `window.set_focus()`；non-focusable/windowed 路径现在额外调用 `activate_break_application()`，通过 `NSApplication.activateIgnoringOtherApps(true)` 显式激活应用，补齐此前只 front window、但未把 app 带到当前 Space 的缺口。
- AC2（设置页数字输入不再卡住）：
  - `apps/desktop/src/App.tsx` 的 `CompactNumber` 已改为本地 draft 编辑：允许临时空值与多位数输入，只在 `blur / Enter / 加减按钮` 时提交 clamp 后的值；`Escape` 恢复当前持久化值，且按钮点击通过 `onMouseDown(preventDefault)` 避免 blur/click 竞态。
- AC3（不回退现有 break/tray 行为）：
  - `shell.rs` 现有 break close/fullscreen helper 与 tray refresh workaround 保持原状；本轮只追加 non-focusable break 的 macOS app activation 调用点。
- AC4（构建/测试/validator）：
  - 相关命令均已通过，见下方命令记录。

## Commands
- `npm test` -> PASS
  - `Test Files  5 passed (5)`
  - `Tests  62 passed (62)`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` -> PASS
  - `test result: ok. 23 passed; 0 failed`
- `npm --prefix apps/desktop run typecheck` -> PASS
- `npm --prefix apps/desktop run build` -> PASS
  - `✓ 1826 modules transformed.`
  - `✓ built in 1.74s`
  - 保留既有 chunk size warning
- `python3 scripts/validate_workflow_docs.py --mode manual` -> PASS
  - `[OK] Workflow docs validation passed for 2026-04-11 ✅`

## Interaction Coverage
- Primary flow:
  - `due -> show_break_window() -> present_break_window() -> activate_break_application()` 的 macOS 路径已落地并通过构建/测试验证。
- Fallback / secondary flow:
  - `CompactNumber` 已覆盖 `empty/partial draft -> blur|Enter commit -> Escape restore -> +/- step commit` 的交互边界。
- Visible states:
  - 当前值、草稿值、禁用按钮状态与 `blur/click` 顺序都由前台组件内部处理，不再每击键直写 autosave。

## Gaps / Reality Note
- 本轮没有补一段真实 macOS 全屏 Space 手工录屏；AC1 当前证据以宿主代码路径审查 + 前后端构建/测试为主。
- 如果后续需要更强现实证据，可在真实全屏应用场景下补一轮手工 spot-check 或录屏。
