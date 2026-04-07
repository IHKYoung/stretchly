# Task-ID: TID-20260403-macos-tray-menu-crash

## Goals
- 修复 `apps/desktop` 在 macOS 上右键 tray icon 时的原生菜单闪退。
- 保持 tray action IDs、动作处理逻辑和非 macOS 路径不变。

## Non-Goals
- 不重排菜单信息架构、不改 tray 左键语义。
- 不修改 Electron legacy `app/main.js` 的托盘实现。
- 不调整前台 React/Tailwind UI。

## Constraints & Assumptions
- `muda`/Tauri 在 macOS 顶层 menu 只保证接受 `Submenu`，因此 root context menu 类型必须平台分支处理。
- 原生 tray menu 在显示期间若被 `set_menu()` 替换，用户会看到菜单瞬间消失，即使进程本身并未退出。
- 现有 tray action ids（如 `open`、`pause-30`、`quit`）已经被事件处理和 i18n 文案依赖，不能漂移。
- 修复应尽量局限在 `shell.rs`，避免影响 `state.rs`、`commands.rs` 与 `engine.rs` 的运行时契约。

## System Boundaries
- Modules: `apps/desktop/src-tauri/src/shell.rs`
- Modules: `apps/desktop/src-tauri/src/shell.rs`, `apps/desktop/src-tauri/src/engine.rs`
- Ownership: tray context menu 构造、tray click handler、快捷键刷新保持在 shell 层
- Dependency direction: `shell.rs` 消费 `PauzaState` snapshot 与 i18n，不反向改变 state/engine/commands 契约

## API / Contract
- Signatures / Endpoints:
  - `create_tray_menu` 在 macOS 返回 `Submenu<R>`，在其他平台返回 `Menu<R>`
  - 共享 `populate_tray_menu` 负责填充状态项、skip/focus/pause submenu 与基础动作项
- Request/Response schema (typed):
  - 无外部 API 变化；tray action id 与 `handle_tray_action` 的匹配保持不变
- Error model (codes, retryability):
  - 仍使用 `tauri::Result<()>` / `Result<(), String>` 传播菜单构造与主线程调度错误

## Data Model / Storage
- 无新增存储或数据结构；继续复用 `DesktopSnapshot` 和 `PauzaSettings`

## Invariants
- macOS tray context menu 顶层必须是 `Submenu`
- 非 macOS tray 仍可直接使用 `Menu`
- tray 左键只显示主窗口，右键才展开菜单
- 后台 tick 不应在菜单内容未发生有效变化时重建 tray menu
- 所有菜单项 ID 和本地化 key 保持稳定

## Concurrency / Lifecycle / Memory Model
- 菜单对象仍由 Tauri/muda 在主线程创建与刷新
- tray action 仍在后台线程处理，完成后通过 `refresh_tray` 回主线程刷新菜单
- 本轮不改 break/main window 生命周期和 shortcut 注册流程

## Observability Plan (Debug-Driven)
- Logs: 用 `cargo check` 验证 Rust 编译闭环；用 `npm --prefix apps/desktop run tauri dev` 启动真实 tray runtime
- Metrics: 无
- Traces: 无
- Debug flags: 无；补充记录 `ps` 进程状态与 `osascript` 权限阻塞信息作为现实证据

## Security & Privacy Considerations
- 不新增权限、不读取新数据、不引入网络或系统写入副作用

## Risks & Rollback
- Failure modes:
  - macOS submenu root 仍无法正常弹出
  - tray refresh key 设计不当，导致状态变化后菜单长时间不更新
  - 非 macOS 共享填充函数引入菜单内容回归
- Rollback steps:
  - 回退 `apps/desktop/src-tauri/src/shell.rs` 本轮 tray menu 平台分支和共享填充函数
  - 重新 `cargo check` / `tauri dev` 验证恢复旧行为

## Acceptance Criteria (System)
- macOS 右键 tray icon 不再因为菜单根类型错误导致进程闪退
- macOS 右键 tray icon 不再因后台秒级重建 tray menu 而“一闪即逝”
- tray 菜单动作 ID 和刷新逻辑保持兼容
- Rust host 编译通过，dev runtime 可正常启动

## Open Questions / Decision Requests
- 无；等待用户在本机已运行的 app 上完成最终右键验证
