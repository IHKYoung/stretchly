# Task-ID: TID-20260409-tray-menu-live-status-and-option

## Goals
- 让 tray 菜单里的状态信息项脱离“菜单重建”也能实时刷新。
- 把 `showTimeToBreakInTray` 纳入当前 `apps/desktop` 的设置真源。

## Non-Goals
- 不重构调度状态机。
- 不新增 tray 样式枚举或进度样式。
- 不修改 tray action command 语义。

## Constraints & Assumptions
- 现有 `refresh_tray_if_needed()` 的去抖逻辑必须保留，不能为实时状态重建整个菜单。
- `MenuItem::set_text()` 可用于原地更新禁用信息项。
- settings 仍以 `PauzaSettings` 作为唯一持久化真源。

## System Boundaries
- Modules:
  - `state.rs`：settings schema/default/snapshot
  - `shell.rs`：tray menu / title / tooltip 更新
  - `App.tsx`：设置页开关入口
- Ownership:
  - 本任务只改 tray 展示层和相关设置绑定。
- Dependency direction:
  - `App.tsx -> update_settings -> PauzaSettings -> shell.rs`

## API / Contract
- Signatures / Endpoints:
  - 不新增 tauri command；继续复用 `update_settings`
  - `shell.rs` 内部新增 menu text updater trait / helper
- Request/Response schema (typed):
  - `PauzaSettings` 新增 `showTimeToBreakInTray` / `show_time_to_break_in_tray`
- Error model (codes, retryability):
  - 沿用现有 `Result<(), String>` 宿主错误包装

## Data Model / Storage
- `settings.json` 新增布尔字段 `showTimeToBreakInTray`
- 缺失旧字段时默认值为 `true`

## Invariants
- 顶部 tray title 受 `showTimeToBreakInTray` 控制。
- 右键菜单 `status` / `status-detail` 不受该开关影响，始终反映实时状态。
- tray 菜单刷新仍由 `TrayRefreshKey` 控制，不变成每秒重建。

## Concurrency / Lifecycle / Memory Model
- engine 每秒 tick 调用 `refresh_tray_if_needed()`
- menu item updater 持有当前 menu item 句柄，并在每次重建菜单后被替换
- 不引入新状态机或额外线程

## Observability Plan (Debug-Driven)
- Logs:
  - 不新增日志
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - N/A

## Security & Privacy Considerations
- 仅涉及本地 UI 展示和本地设置持久化，不引入新权限。

## Risks & Rollback
- Failure modes:
  - menu item handle 失效
  - 旧 settings 缺失字段时默认值错误
- Rollback steps:
  - 回退 `PauzaSettings` 字段、前台开关、menu updater 与 locale 改动

## Acceptance Criteria (System)
- 菜单状态项能实时更新
- title 开关持久化可用
- locale sync、typecheck、Rust tests、build、docs validator 均通过

## Open Questions / Decision Requests
- N/A
