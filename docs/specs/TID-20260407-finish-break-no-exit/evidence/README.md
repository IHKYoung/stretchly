# Task-ID: TID-20260407-finish-break-no-exit Evidence

## Summary
- 证据目标：证明 break CTA 已从“命令执行中同步 destroy 当前窗口”改为“命令回包后 deferred teardown”，从而避免点击“完成休息”时应用直接退出。
- 采集时间：2026-04-07
- 采集方式：代码路径核对 + Rust/前端构建链 + workflow docs validator

## Artifacts
- 无新鲜原生 UI 截图

## AC Coverage
- AC1 / AC3：`commands.rs` 中 `finish_current_break`、`skip_current_break`、`postpone_current_break` 现统一调用 `close_break_window_deferred(app.clone())`。
- AC2：`shell.rs` 新增 `close_break_window_deferred()`，先等待极小延迟，再回到 `close_break_window()` 做真实 teardown。
- AC4：`close_break_window()` 仍保留 `is_fullscreen() -> set_fullscreen(false) -> hide() -> destroy()`。
- AC5：`cargo check`、`cargo test`、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build` 与 `python3 scripts/validate_workflow_docs.py --mode manual` 均已通过。

## Gaps
- 当前环境未录制一次真实原生 break prompt 点击“完成休息”的操作；最终现实确认仍需用户本机手动验证应用未退出且 tray/menu bar icon 仍在。
