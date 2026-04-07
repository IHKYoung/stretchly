# Pauza Desktop

`apps/desktop` 是 Pauza 的新一代桌面端基础骨架，使用 `Tauri 2 + React + TypeScript`。

当前目标不是把旧 Electron 页面原样搬过来，而是先建立新的宿主层：

- 原生 tray / shortcut / notification / autostart
- 面向后续迁移的 Rust 状态与命令边界
- 更适合产品化迭代的前端工作台

## Commands

- `npm install`
- `npm run tauri:dev`
- `npm run tauri:build`
- `npm run typecheck`

## Migration rule

- 旧 `app/**` 只作为领域逻辑参考，不再作为新架构模板。
- break planner、idle、DND、app exclusions 将逐步迁移到 `src-tauri/src/**`。
- 当前 Electron 主体保留一段时间，直到 Tauri 端接管核心能力后再删除。
