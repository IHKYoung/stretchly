# Task-ID: TID-20260409-break-window-macos-fullscreen-coverage

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 让 break 窗口在 macOS 全屏工作区里也能覆盖当前工作屏幕。
- 把修复限制在 Tauri host 的原生窗口展示层，不扩散到前端页面和调度状态机。

## Non-Goals
- 不重做多屏目标选择逻辑。
- 不调整 break 页面布局、文案或互动流程。

## Constraints & Assumptions
- 不能新增依赖；优先复用 Tauri 已暴露的 `ns_window()` 能力和现有 `objc2`。
- 修复必须在非 macOS 平台上保持 no-op。
- 根因假设是 macOS Space / window level 行为不足，而不是 React 前台样式或休息调度错误。

## System Boundaries
- Modules: `apps/desktop/src-tauri/src/shell.rs`
- Ownership: break 宿主窗口的创建、定位、显示与平台特化行为
- Dependency direction: `show_break_window()` / `configure_break_window()` -> macOS native helper；不改 `state.rs` / `commands.rs` 对外契约

## API / Contract
- Signatures / Endpoints: 无新增对外 command；仅扩展 `shell.rs` 内部 helper
- Request/Response schema (typed): N/A
- Error model (codes, retryability): native patch 失败时沿用现有 `Result<(), String>` 上抛；非 macOS 直接 no-op

## Data Model / Storage
- N/A（无持久化结构变化）

## Invariants
- 非 macOS 平台的 break window 行为不变。
- 现有 `fullscreen` / `window` 模式语义不变，只增强 macOS 的原生展示能力。
- 即便原生 `NSWindow` 句柄不可用，break window 也仍按现有 Tauri 抽象层路径显示。

## Concurrency / Lifecycle / Memory Model
- 所有窗口原生 patch 继续发生在 `run_on_main_thread()` 包裹的主线程上下文。
- 不引入新的后台线程或共享状态。

## Observability Plan (Debug-Driven)
- Logs: 暂不新增日志；通过测试和构建保证 helper 接线稳定
- Metrics: N/A
- Traces: N/A
- Debug flags: N/A

## Security & Privacy Considerations
- 无新增权限、系统 API 探测或数据采集。

## Risks & Rollback
- Failure modes:
  - 窗口层级过高，导致非全屏场景下过于强势
  - macOS 原生 selector 调用错误，造成 break window 无法显示
- Rollback steps:
  - 回退 `shell.rs` 中的 macOS native helper 与对应测试
  - 重跑 `cargo test` / `npm --prefix apps/desktop run build`

## Acceptance Criteria (System)
- macOS break 窗口会显式带上 `CanJoinAllSpaces | FullScreenAuxiliary`。
- macOS fullscreen / windowed break 在显示时会使用更高的原生窗口层级，并主动 front 到当前 Space。
- `cargo test` 与 desktop build 通过。

## Open Questions / Decision Requests
- 若后续还要根据“当前实际工作屏”自动改写 monitor 选择，应单开任务处理。
