# Task-ID: TID-20260404-fullscreen-break-close-fix Evidence

## Summary
- 证据目标：证明 fullscreen break 的关闭链路已经从“仅 hide”修正为“退出 fullscreen + destroy”。
- 采集时间：2026-04-04
- 采集方式：代码路径核对 + 编译链

## Artifacts
- 无新鲜 UI 截图

## AC Coverage
- AC1 / AC2 / AC3：`shell.rs` 中 `close_break_window()` 现已先检查 `is_fullscreen()`，若为 true 则 `set_fullscreen(false)`，随后 `hide()` 并 `destroy()`。
- AC4：`cargo check`、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build` 均通过。
- AC5：本报告明确记录了现实确认缺口。

## Gaps
- 当前环境未完成一次真实 fullscreen break -> 点击 `跳过` 的运行时录像或截图；最终现实确认仍需用户本机手动验证。
