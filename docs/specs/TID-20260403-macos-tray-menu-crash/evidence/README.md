# Task-ID: TID-20260403-macos-tray-menu-crash

## Evidence Summary
- 类型：partial
- 原因：本轮修复涉及 macOS 原生 tray 菜单与后台刷新策略；当前终端缺少 assistive access，无法自动替代用户执行菜单栏右键，因此先保留运行态、源码与权限阻塞证据，最终右键确认由用户完成。

## Captured Evidence
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`：PASS
- `npm --prefix apps/desktop run tauri dev`：已启动真实 dev runtime，并运行 `target/debug/pauza-desktop`
- `rg -n "refresh_tray_if_needed|refresh_tray\\(" apps/desktop/src-tauri/src/{engine,shell}.rs`：
  - `engine.rs` 已从秒级 `refresh_tray()` 改为 `refresh_tray_if_needed()`
  - `shell.rs` 新增 tray refresh key，仅在菜单内容变化时重建原生 tray menu
- `pgrep -fl 'target/debug/pauza-desktop|node .*tauri dev|vite --host 127.0.0.1 --port 43179'`：
  - `3481 node /Users/changkunyang/CKProjects/Pauza/apps/desktop/node_modules/.bin/tauri dev`
  - `3775 node /Users/changkunyang/CKProjects/Pauza/apps/desktop/node_modules/.bin/vite --host 127.0.0.1 --port 43179`
  - `3870 target/debug/pauza-desktop`
- `osascript -e 'tell application "System Events" to tell process "pauza-desktop" to get count of menu bars'`：
  - `execution error: osascript is not allowed assistive access. (-25211)`

## AC Mapping
- AC1：代码层已将 macOS tray 根菜单改为 `Submenu`，并移除后台秒级 tray menu 重建；最终展开验证待用户右键确认
- AC2：`handle_tray_action` 与左键 reveal handler 未改，菜单动作 ID 与交互语义保持原样
- AC3：`engine.rs` 已改为 `refresh_tray_if_needed()`，后台不再每秒强制换 menu
- AC4：编译通过，dev runtime 已实际运行

## Manual Check Pending
- 请在当前已运行的 Pauza 应用上对顶部菜单栏图标执行一次右键，确认菜单稳定展开且应用不退出。
