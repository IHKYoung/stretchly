# Task-ID: TID-20260411-settings-freeze-root-cause-fix

## Goals
- 将设置保存链路从“每次修改都做无差别宿主全量刷新”改为差异驱动。
- 去掉前端 autosave 与 Rust host refresh 叠加造成的冻结风险。

## Non-Goals
- 不改 `PauzaSettings` schema。
- 不改调度状态机、tray 菜单结构或设置页布局。

## Constraints & Assumptions
- 继续沿用当前 `invoke('update_settings') -> DesktopSnapshot` 契约，不新增 command。
- 优化重点是 host refresh 频率与保存并发，而不是引入新的异步存储层。

## System Boundaries
- Modules:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src-tauri/src/commands.rs`
  - `apps/desktop/src-tauri/src/state.rs`
- Ownership:
  - 前端负责串行/合并 autosave
  - Rust host 负责差异判断与最小化刷新
- Dependency direction:
  - `App.tsx` -> `invoke('update_settings')`
  - `commands.rs` -> `state.update_settings()` -> selective `shell::*`

## API / Contract
- Signatures / Endpoints:
  - `update_settings(settings) -> DesktopSnapshot`
- Request/Response schema (typed):
  - 输入仍为完整 `PauzaSettings`
  - 返回仍为完整 `DesktopSnapshot`
- Error model (codes, retryability):
  - 继续使用字符串错误上抛；前端下一轮保存会在后续修改后重试

## Data Model / Storage
- `PauzaSettings` 现在支持 `PartialEq/Eq`，便于 no-op 检测与 host refresh diff。
- `state.update_settings()` 现在会在设置未变化时直接短路，避免重复写盘。

## Invariants
- 保存中最多只有一轮 `update_settings` 在飞。
- 语言变化仍需完整 tray menu rebuild。
- 其它普通设置变化默认只走按需 tray sync，不再无条件整棵 `set_menu()`。

## Concurrency / Lifecycle / Memory Model
- 前端 autosave 通过 `formRevision` / `saveRetryToken` / `saveInFlightRef` 串行化，避免多个宿主保存并发。
- Rust `state.update_settings()` 在持锁状态下完成 no-op 检查和写盘，命令层在解锁后再决定是否刷新 shortcuts/tray。

## Observability Plan (Debug-Driven)
- Logs:
  - 通过 daily logs 记录根因、修复点和验证输出。
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - N/A

## Security & Privacy Considerations
- 不新增权限、依赖、网络访问或外部服务。

## Risks & Rollback
- Failure modes:
  - 差异判断过窄导致某些菜单文案没刷新
  - autosave 串行化逻辑导致保存遗漏
- Rollback steps:
  - 回退 `App.tsx` 的串行 autosave 和 Rust host diff 逻辑

## Acceptance Criteria (System)
- no-op 设置保存直接短路。
- 只有 shortcut 变化才重绑 global shortcuts。
- 只有语言变化才强制 tray menu rebuild，其它设置最多走 `refresh_tray_if_needed()`。

## Open Questions / Decision Requests
- 若后续仍感到“设置一改就卡”，下一步应重点检查 `settings.json` 在包含大尺寸自定义壁纸 data URL 时的写盘成本，必要时再拆分存储。
