# Task-ID: TID-20260407-finish-break-no-exit

## Goals
- 修复 break CTA 触发的宿主生命周期竞态，避免当前 break webview 在 `invoke` 返回前被同步销毁。
- 保留 `close_break_window()` 对 fullscreen break 的退出 fullscreen + destroy 语义，不回退相邻黑屏修复。

## Non-Goals
- 不改 `PauzaState` 的 break 调度语义。
- 不改 React break prompt 的按钮文案、布局或交互入口。
- 不改 Electron legacy 关闭链路。

## Constraints & Assumptions
- 用户复现场景发生在 Tauri break window 的“完成休息”按钮。
- 当前 bug 更接近窗口生命周期/命令回包时序问题，而不是状态机 `finish_current_break()` 的调度逻辑错误。
- 现有 `close_break_window()` 必须继续负责 fullscreen 退出与窗口 destroy，否则会回退 2026-04-04 的黑屏修复。

## System Boundaries
- Modules:
  - `apps/desktop/src-tauri/src/commands.rs`
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src/App.tsx`
- Ownership:
  - `commands.rs` 负责 break CTA 的命令边界与 snapshot 回包
  - `shell.rs` 负责真实窗口 teardown
  - `state.rs` 继续作为 break 生命周期真源
- Dependency direction:
  - React break CTA -> Tauri command -> `PauzaState` state mutation -> tray refresh / snapshot build -> deferred shell teardown

## API / Contract
- Signatures / Endpoints:
  - `finish_current_break() -> Result<DesktopSnapshot, String>`
  - `skip_current_break() -> Result<DesktopSnapshot, String>`
  - `postpone_current_break() -> Result<DesktopSnapshot, String>`
  - `close_break_window_deferred(AppHandle<R>) -> ()`
- Request/Response schema (typed):
  - 请求参数不变；命令仍返回最新 `DesktopSnapshot` 给前端。
- Error model (codes, retryability):
  - 继续沿用字符串错误；本轮不引入新错误码。

## Data Model / Storage
- N/A；不改 settings schema、持久化文件或运行时数据结构。

## Invariants
- break CTA 命令必须先返回 snapshot，再 teardown 当前 break 窗口。
- 实际窗口关闭仍统一走 `close_break_window()`，保证 fullscreen break 先退出 fullscreen 再 destroy。
- tray 刷新与后续调度计划保持原有顺序，不因窗口关闭策略变化而跳过。

## Concurrency / Lifecycle / Memory Model
- `commands.rs` 在 break CTA 命中后克隆 `AppHandle` 并调用 `close_break_window_deferred()`。
- `close_break_window_deferred()` 在后台线程 sleep 一个很小的延迟后，再回到主线程执行真实 break window teardown。
- 该模型避免“销毁当前调用窗口”和“向该窗口回传 invoke 结果”发生在同一临界区。

## Observability Plan (Debug-Driven)
- Logs:
- 继续依赖 `DesktopSnapshot.lastAction`、tray 状态和构建链输出；本轮不新增宿主日志字段。
- Metrics:
  - N/A
- Traces:
  - `cargo check` / `cargo test` / `typecheck` / `build` 输出作为实现自证。
- Debug flags:
  - N/A

## Security & Privacy Considerations
- 不新增权限、网络、存储或用户数据路径。

## Risks & Rollback
- Failure modes:
- 如果延迟 teardown 仍不足以规避宿主竞态，可能还需要进一步切换为前端事件驱动的自关闭模式。
- 回归风险主要集中在 break prompt 关闭速度和 fullscreen close path。
- Rollback steps:
  - 回退 `commands.rs` / `shell.rs` 到同步 close 版本。
  - 保留本任务 docs 以说明为何回滚。

## Acceptance Criteria (System)
- break CTA 命令不再在 `invoke` 回包前同步 destroy 当前 break webview。
- fullscreen close fix 仍保留。
- Rust 编译/单测和前端构建链通过。

## Open Questions / Decision Requests
- 待用户本机最终确认：点击“完成休息”后应用是否保持后台存活且 tray/menu bar icon 仍在。
