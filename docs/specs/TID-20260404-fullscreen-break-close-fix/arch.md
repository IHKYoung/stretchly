# Task-ID: TID-20260404-fullscreen-break-close-fix

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 修复 Tauri break window fullscreen 关闭链路的生命周期 bug。

## Non-Goals
- 不修改 scheduler、state schema 或 React 组件。

## Constraints & Assumptions
- 旧 Electron 的 `closeWindows()` 采用 `hide() + destroy()`，这是本轮主要 source basis。
- Tauri `WebviewWindow::destroy()` 可以绕过 `CloseRequested`，适合清理残留 break 窗口实例。

## System Boundaries
- Modules:
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src-tauri/src/commands.rs`（只读）
  - `apps/desktop/src-tauri/src/state.rs`（只读）
- Ownership:
  - 本任务只拥有 break window 宿主关闭策略。
- Dependency direction:
  - break CTA / tray / engine -> `close_break_window()` -> native window teardown

## API / Contract
- Signatures / Endpoints:
  - `close_break_window(app)`
- Request/Response schema (typed):
  - 无新 API；仅改变内部 teardown 顺序。
- Error model (codes, retryability):
  - 无新增错误码；close path 仍尽量忽略单窗 teardown 错误并继续。

## Data Model / Storage
- 无数据模型变更。

## Invariants
- fullscreen break 关闭前必须先退出 fullscreen。
- break windows 关闭后不应保留可复用但已损坏的隐藏实例。
- 非 fullscreen break 关闭路径仍然可用。

## Concurrency / Lifecycle / Memory Model
- `close_break_window()` 运行在主线程。
- `destroy()` 绕过 `CloseRequested`，避免重新走用户关闭拦截逻辑。

## Observability Plan (Debug-Driven)
- Logs: 无新增日志；依赖用户现实复核
- Metrics: 无
- Traces: 无
- Debug flags: 无

## Security & Privacy Considerations
- 不新增权限、网络或用户数据路径。

## Risks & Rollback
- Failure modes:
  - 某些平台上 fullscreen 退出和 destroy 的时序仍可能需要更细调。
  - destroy 导致后续 break 窗口每次重建，若有副作用需继续修正。
- Rollback steps:
  - 回退 `shell.rs` 与 docs。

## Acceptance Criteria (System)
- `close_break_window()` 先 `set_fullscreen(false)`，后 `destroy()`
- Rust host 编译通过
- 不触碰 state schema

## Open Questions / Decision Requests
- 如果用户本机仍能复现黑屏，下一步需要做真实 runtime 证据采集
