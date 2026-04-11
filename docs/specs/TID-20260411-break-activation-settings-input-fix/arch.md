# Task-ID: TID-20260411-break-activation-settings-input-fix

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 在不改 `PauzaSettings`、调度状态机和 command 契约的前提下，修复 macOS break 浮出链路与设置页数字输入交互。

## Non-Goals
- 不重写 `PauzaState::tick()` 或 next break planner
- 不新增持久化字段、命令或 locale 资源
- 不重做 settings 视觉层或 break prompt 文案

## Constraints & Assumptions
- 不能新增依赖。
- 现有 Tauri host 仍以 `update_settings(settings: PauzaSettings)` 为唯一设置保存入口。
- `CompactNumber` 当前问题来自前台受控输入生命周期，而不是 `settings.json` 写盘失败。
- macOS fullscreen Space 可见性问题更接近宿主激活缺失，而不是 `open_break_window` 没有触发。

## System Boundaries
- Modules:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src-tauri/src/{commands.rs,state.rs,engine.rs}`（只读校验链路）
- Ownership:
  - 前台数字输入草稿/提交边界由 `App.tsx` 负责
  - break window 宿主显示/激活边界由 `shell.rs` 负责
- Dependency direction:
  - `CompactNumber draft` -> `updateForm()` -> autosave effect -> `invoke('update_settings')`
  - `engine.tick()` -> `actions.open_break_window` -> `shell::show_break_window()` -> macOS native present/activate path

## API / Contract
- Signatures / Endpoints:
  - 继续复用 `update_settings(settings: PauzaSettings) -> DesktopSnapshot`
  - 不新增 Tauri command；仍由 `engine.rs` 调 `show_break_window()`
- Request/Response schema (typed):
  - `PauzaSettings` 与 `DesktopSnapshot` schema 不变
- Error model (codes, retryability):
  - 无新增错误码；现有 `String` 错误上抛与前台 error UI 保持不变

## Data Model / Storage
- 无新增字段；`settings.json` 结构保持不变。

## Invariants
- 数字输入在提交前只影响本地 draft，不应触发最终保存值回弹。
- blur / Enter / step button 提交后，值仍必须 obey `min/max` clamp。
- macOS break window 即使在 non-focusable/windowed 路径，也必须显式把应用带到当前 Space。
- 现有 tray refresh workaround、break close/fullscreen helper 语义不应被回退。

## Concurrency / Lifecycle / Memory Model
- `CompactNumber` 的 draft 状态与上层 `form` 分离，避免每击键触发 autosave effect。
- break window 激活补丁只挂在 `show_break_window()` 成功显示后，不修改 `PauzaState` runtime mutex 或 tick 顺序。
- 与现有 `run_on_main_thread()`、`run_macos_native_break_window_patch()` 生命周期保持一致。

## Observability Plan (Debug-Driven)
- Logs:
  - 继续依赖 `runtime.last_action`、tray status、前台 save/load error
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - N/A

## Security & Privacy Considerations
- 无新增权限、文件范围、网络通信或敏感数据流。

## Risks & Rollback
- Failure modes:
  - 应用激活补丁若过强，可能让普通 windowed break 更容易抢前台
  - 输入草稿与按钮交互若处理不当，可能出现 blur/click 顺序问题
- Rollback steps:
  - 回退 `App.tsx` 的 draft 输入逻辑与 `shell.rs` 的 macOS activation helper，再重跑构建/测试

## Acceptance Criteria (System)
- 宿主层在 macOS fullscreen Space 中可靠展示 break prompt。
- 前台数字输入不再每击键直写最终值。
- 构建、测试和 docs validator 通过。

## Open Questions / Decision Requests
- 无。
