# Task-ID: TID-20260408-tauri-window-fullscreen-fix

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 把主窗口尺寸边界和 break fullscreen 进入/退出收敛到单一可维护宿主层实现。

## Non-Goals
- 不重做 break 页面前端结构
- 不改变 settings schema、调度状态机或 tray 行为

## Constraints & Assumptions
- 不新增依赖，不改外部契约
- 所有窗口操作继续在 `run_on_main_thread()` 包裹下执行
- macOS 顶部空白更可能来自 native fullscreen/titlebar 语义，而不是 React 布局

## System Boundaries
- Modules:
  - `apps/desktop/src-tauri/tauri.conf.json`：主窗口默认/最小尺寸真源
  - `apps/desktop/src-tauri/src/shell.rs`：break window 生命周期与 fullscreen 宿主策略
- Ownership:
  - host shell only
- Dependency direction:
  - `tauri.conf.json` -> 主窗口边界
  - `shell.rs` -> break window fullscreen helper

## API / Contract
- Signatures / Endpoints:
  - `configure_break_window()`
  - `close_break_window()`
  - `set_break_window_fullscreen()`
- Request/Response schema (typed):
  - 无新增对外 API；仅调整内部 helper
- Error model (codes, retryability):
  - 沿用 `Result<(), String>` 宿主错误模型，无新增 error code

## Data Model / Storage
- N/A（无数据结构或持久化变更）

## Invariants
- 主设置窗口不能小于 `800x600`
- break fullscreen 进入/退出必须经由同一个 helper
- macOS fullscreen 优先使用 simple fullscreen，close path 同时兜底 simple/native fullscreen

## Concurrency / Lifecycle / Memory Model
- 窗口状态变更继续只在主线程执行
- break close path 仍允许 deferred teardown；本轮不改变其线程模型

## Observability Plan (Debug-Driven)
- Logs:
  - 本轮未新增 logger；后续若再次出现 fullscreen 问题，应优先在 `set_break_window_fullscreen()` 打点
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - N/A

## Security & Privacy Considerations
- 不新增权限、提权、外部连接或敏感数据处理

## Risks & Rollback
- Failure modes:
  - macOS simple fullscreen 退出时仍可能有系统动画
  - 若 helper 未来被绕开，native/simple fullscreen 语义会再次漂移
- Rollback steps:
  - 回退 `apps/desktop/src-tauri/{tauri.conf.json,src/shell.rs}`

## Acceptance Criteria (System)
- fullscreen 宿主逻辑只有一个入口 helper
- close path 不再依赖 `is_fullscreen()` 单独判断
- 编译链通过

## Open Questions / Decision Requests
- 后续是否需要补一条 macOS 专用运行时日志，用于区分 simple fullscreen / native fullscreen 进入路径
