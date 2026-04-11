# Task-ID: TID-20260411-jump-to-next-break-freeze

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 修复 tray 顶部菜单点击“跳到下一次休息”后宿主卡死的问题。
- 保持 skip 调度语义、tray 文案和快捷键路径不变，只调整 tray 菜单动作后的刷新时机。

## Non-Goals
- 不修改 `state.rs` 的 break 计划、skip 语义或任何 settings 持久化字段。
- 不改 React 前台、break prompt 布局或多语言内容。
- 不重新设计 tray 菜单层级。

## Constraints & Assumptions
- 不能新增依赖。
- 根因更接近 tray 菜单动作后的主线程刷新重入，而不是 `PauzaState::skip_to_*` 的调度死循环。
- `TID-20260403-macos-tray-menu-crash` 已证明 macOS 原生 tray 菜单对 `set_menu()` 生命周期较敏感，本次问题很可能是同类回归。

## System Boundaries
- Modules:
- `apps/desktop/src-tauri/src/shell.rs`
- `apps/desktop/src-tauri/src/state.rs`
- `apps/desktop/src-tauri/src/engine.rs`
- Ownership:
- tray 菜单事件分发、菜单重建和主线程桥接均由 `shell.rs` 管理。
- Dependency direction:
- `tray on_menu_event` -> `handle_tray_action()` -> `PauzaState::skip_to_*` -> tray refresh helper
- `engine.rs` 继续只负责秒级 tick 和 `refresh_tray_if_needed()`，不重新接管 tray action 生命周期

## API / Contract
- Signatures / Endpoints:
- `handle_tray_action(app, action)` 继续消费既有 tray action id
- `refresh_tray()` / `refresh_tray_if_needed()` 继续作为 tray 刷新入口
- Request/Response schema (typed):
- 无新增命令或 schema；仅调整 tray action 后触发刷新入口的时机
- Error model (codes, retryability):
- 继续沿用 `Result<(), String>` 传播 tray 主线程调度错误；不新增错误码

## Data Model / Storage
- 无新增持久化字段；`PauzaSettings`、`RuntimeState` 与 `DesktopSnapshot` 结构保持不变。

## Invariants
- tray 菜单动作 ID 与现有本地化 key 必须保持不变。
- shortcut / settings 页面触发的 tray 刷新语义不能被 tray 菜单专属修复误伤。
- tray 菜单状态更新仍应最终收敛到最新 snapshot，只是不再与菜单点击处于同一宿主临界区。

## Concurrency / Lifecycle / Memory Model
- `tray.set_menu()` 与 `MenuItem::set_text()` 均通过 Tauri 的主线程阻塞桥接执行。
- 本次修复优先拆开“菜单项点击”和“菜单重建/文本同步”的时间重叠，避免在 native menu 关闭前再次进入主线程菜单更新。
- 不改 `PauzaState` 锁模型，只减少 tray action 路径上的主线程竞争。

## Observability Plan (Debug-Driven)
- Logs:
- 沿用当前 `last_action` 与 tray snapshot；不新增高频日志
- Metrics:
- N/A
- Traces:
- N/A
- Debug flags:
- N/A

## Security & Privacy Considerations
- 不新增权限、不读取新数据、不引入网络或外部进程副作用。

## Risks & Rollback
- Failure modes:
- 如果根因并非 tray 刷新时机而是 break window 打开路径，用户仍可能复现卡死。
- 若延迟刷新过长，tray 状态文案会有轻微滞后。
- Rollback steps:
- 回退 `apps/desktop/src-tauri/src/shell.rs` 中 tray 菜单动作后的刷新时机调整。
- 重新执行 Rust/前端构建与 docs validator 验证旧行为恢复。

## Acceptance Criteria (System)
- 选择 tray 顶部菜单的 skip 动作时，不再在点击路径上立刻整棵 `set_menu()` 重建。
- skip 调度语义与 tray/shortcut 既有 action id 保持不变。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`npm --prefix apps/desktop run build` 与 `python3 scripts/validate_workflow_docs.py --mode manual` 通过。

## Open Questions / Decision Requests
- 无；若本地自动化无法覆盖 macOS 菜单栏点击，最终现实检查由用户手工确认。
