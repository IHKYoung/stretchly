# Task-ID: TID-20260408-adaptive-reminder-state-machine

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 用显式运行时状态机表达“break 已到点但当前不宜打断”的中间态，替换当前只有 `next_break_due_ms -> start_current_break()` 的直线触发。
- 保持现有 pause / focus / DND / natural break / app exclusion / postpone / skip / manual finish 契约不变。

## Non-Goals
- 不新增全局键盘监听、鼠标 hook 或外部遥测。
- 不重构 `commands.rs`、`shell.rs` 的命令面和窗口层。
- 不回填 Electron legacy 的同等功能。

## Constraints & Assumptions
- 默认入口是 Tauri host，因此必须在 `apps/desktop/src-tauri/src/state.rs` 落地。
- 当前可用信号只有 `idle_ms`、DND、app exclusion、focus/pause/natural break；“是否正在打字”只能由 idle gap 近似判断。
- 不新增依赖，不改变现有用户可见设置页结构。

## System Boundaries
- Modules:
  - `apps/desktop/src-tauri/src/state.rs`：调度真源、hidden adaptive settings、pending-delivery 状态机、snapshot/status 文案入口。
  - `apps/desktop/src-tauri/src/engine.rs`：每秒 tick，消费 `EngineActions`。
  - `apps/desktop/src-tauri/src/shell.rs`：tray/status 读取 snapshot，不新增命令。
  - `apps/desktop/src/locales/{zh-CN,en}.json`：等待空档/soft nudge 文案。
- Ownership:
  - host 状态机与 snapshot 契约由 `state.rs` 负责。
  - UI 仅消费状态文本，不自行推断 waiting 状态。
- Dependency direction:
  - `engine -> state -> i18n/settings`
  - `shell -> state snapshot`
  - 前台 `App.tsx` 只读 snapshot，不直接操纵状态机内部节点。

## API / Contract
- Signatures / Endpoints:
  - `PauzaState::tick(now, idle_ms, dnd_active, app_exclusion_match) -> EngineActions`
  - `PauzaState::snapshot(...) -> DesktopSnapshot`
  - `PauzaState::update_settings(settings)` 继续接收 `PauzaSettings`
- Request/Response schema (typed):
  - `PauzaSettings` 追加 hidden adaptive delivery 阈值字段，用于机会阈值与 soft nudge 阈值。
  - `RuntimeState` 追加 pending-delivery 内部字段，用于记录 waiting started / nudged at。
  - `DesktopSnapshot` 契约保持稳定，主要通过 `status/statusDetail/last_action` 表达新状态。
- Error model (codes, retryability):
  - 无新增错误码；不改变现有 command error surface。

## Data Model / Storage
- 持久化：
  - `adaptive_breaks_enabled`
  - `microbreak_idle_opportunity_seconds`
  - `long_break_idle_opportunity_seconds`
  - `microbreak_soft_nudge_seconds`
  - `long_break_soft_nudge_seconds`
- 运行时：
  - `next_break_wait_started_ms`
  - `next_break_nudged_at_ms`

## Invariants
- 任一时刻最多只有一个 pending break 和一个 current break。
- pending break 进入 waiting-for-opportunity 后，不得在 `idle_ms` 仍低于阈值时直接打开 break window。
- 同一待投递 break 最多只发送一次 soft nudge。
- 任一 blocker（pause/focus/DND/app exclusion/natural break）生效时，不应残留 waiting-for-opportunity 状态。

## Concurrency / Lifecycle / Memory Model
- `RuntimeState` 仍通过单一 `Mutex` 串行更新；新状态字段必须在 `reset_schedule / clear_waiting_schedule / schedule_specific_break / start_current_break` 中统一清理或重置。
- `tick()`、命令调用和 tray snapshot 都在同一状态真源上收敛，避免 UI 和调度层对 waiting 状态产生双真源。

## Observability Plan (Debug-Driven)
- Logs:
  - 复用 `last_action` 写出“开始等待空档”“已发送温和提醒”“break 已开始”等状态切换文案。
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - 以 hidden settings 阈值代替单独 debug flag，便于以后调优。

## Security & Privacy Considerations
- 不收集或上传输入内容，不新增 OS 级事件监听。
- 只消费当前已有的 idle/DND/app exclusion 信号，不扩大权限面。

## Risks & Rollback
- Failure modes:
  - idle opportunity 阈值过短导致仍有感知打断。
  - soft nudge 阈值过长导致用户以为提醒失效。
  - pending 状态清理不全导致 break 被卡住或重复投递。
- Rollback steps:
  - 回退 `state.rs` 与 locale 文案新增字段。
  - 重新运行 Rust tests / cargo check / workflow validator，确认恢复旧逻辑。

## Acceptance Criteria (System)
- `tick()` 能在 due 但活跃输入时保持 pending，不开窗。
- idle gap 达标后才能开始 break。
- soft nudge 最多触发一次，且不会替代 blocker 逻辑。
- snapshot/status 能稳定反映 waiting 状态。

## Open Questions / Decision Requests
- 本轮先采用 hidden settings + 默认值；若后续需要把阈值显式暴露到设置页，再单独做信息架构任务。
