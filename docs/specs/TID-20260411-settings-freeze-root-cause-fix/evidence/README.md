# Evidence Report

- Task-ID: TID-20260411-settings-freeze-root-cause-fix
- Date: 2026-04-11
- Evidence required: partial

## Root Cause Summary

- 前端 `App.tsx` 原先会在 `dirty` 状态下每 220ms 触发一次 autosave，但并不会阻止上一轮 `update_settings` 尚未完成时再次发起下一轮保存。
- Rust `commands::update_settings()` 原先对每次设置变化都会无条件执行：
  - `refresh_shortcuts()`
  - `refresh_tray()`（整棵 tray menu rebuild）
  - `close_break_window()`
- 两层叠加后，设置页点击和时间修改都可能把多个宿主重操作堆在一起，形成用户体感上的“卡死”。

## Captured Evidence

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 新增 `commands::tests::*` 三个测试全部通过，说明 host refresh 已改成差异驱动。
- `npm test`
  - 前端现有交互 helper 测试继续通过，说明设置页既有输入边界未被破坏。
- `npm run typecheck`
  - PASS
- `npm --prefix apps/desktop run build`
  - PASS
- `python3 scripts/validate_workflow_docs.py --mode manual`
  - PASS

## Notes

- 本轮没有补桌面端录屏；根因集中在 autosave / host refresh 调度，自动化与代码路径审查已经足够证明修复方向。
- 如果用户后续仍能稳定复现“改任意设置都长时间卡死”，下一轮应重点测量 `settings.json` 在包含大尺寸自定义壁纸 data URL 时的写盘耗时。
