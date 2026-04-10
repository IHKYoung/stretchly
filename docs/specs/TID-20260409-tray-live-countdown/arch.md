# Task-ID: TID-20260409-tray-live-countdown

## Goals
- 将 tray 的“实时变化文本”与“较重的菜单重建”拆开处理。
- 保持现有状态机和菜单动作不变，只修展示层刷新粒度。

## Non-Goals
- 不修改 break 调度逻辑。
- 不新增 tray 设置项。
- 不重画 tray icon。

## Constraints & Assumptions
- 后台 engine tick 已经按 1s 运行，可复用该节奏。
- Windows 不支持 tray title，Linux 不支持 tooltip，需要平台分支兜底。
- tray 菜单每秒重建会带来体验回归，不能采用粗暴刷新。

## System Boundaries
- Modules:
  - `engine.rs`：每秒 tick，驱动 tray 刷新入口。
  - `shell.rs`：tray menu/title/tooltip 的宿主层实现。
  - `state.rs`：提供 `DesktopSnapshot` 真源。
- Ownership:
  - 本任务只改 `shell.rs` 的展示同步逻辑。
- Dependency direction:
  - `shell.rs` 读取 `PauzaState -> DesktopSnapshot`，不反向写状态机。

## API / Contract
- Signatures / Endpoints:
  - 内部新增/扩展 `sync_tray_text()`、`tray_title()`、`tray_countdown_ms()` 等 helper。
- Request/Response schema (typed):
  - 继续使用现有 `DesktopSnapshot`，不新增对外契约。
- Error model (codes, retryability):
  - tray title/tooltip 设置失败时返回 `Result<(), String>`，沿用现有宿主错误包装。

## Data Model / Storage
- 不新增持久化字段。
- 倒计时文本完全由运行时 `DesktopSnapshot` 推导。

## Invariants
- tray 菜单刷新 key 仍保持状态级/分钟级，不为秒级倒计时扩张。
- 当前 break 的倒计时优先于下一次 scheduled break。
- 阻塞态不展示误导性的 scheduled countdown。

## Concurrency / Lifecycle / Memory Model
- engine 在线程中每秒调用 `refresh_tray_if_needed()`。
- tray title/tooltip 更新必须对后台线程安全，因此通过 Tauri tray API 内部切回主线程。
- 不引入新的共享状态或长期缓存。

## Observability Plan (Debug-Driven)
- Logs:
  - 不新增日志，避免 tray 每秒刷新带来噪声。
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - N/A

## Security & Privacy Considerations
- 仅显示本地调度剩余时间，不引入新权限或新数据面。

## Risks & Rollback
- Failure modes:
  - 平台不支持 title/tooltip 时触发宿主错误。
  - title 与菜单语义不同步。
- Rollback steps:
  - 回退 `shell.rs` 的 live title/tooltip helper 与对应测试。

## Acceptance Criteria (System)
- 菜单不每秒重建。
- title 在有时间语义的状态下按秒更新。
- 现有 Rust 测试与 desktop build 通过。

## Open Questions / Decision Requests
- N/A
