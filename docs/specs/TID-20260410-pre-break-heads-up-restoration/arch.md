# Task-ID: TID-20260410-pre-break-heads-up-restoration

## Goals
- 把 pre-break heads-up 从“一次性系统通知副作用”改为状态机里的可见运行态。
- 保持 due 后 smart / forced 的 break 投递行为不漂移。

## Non-Goals
- 不新增新的 Tauri command、通知权限流程或额外提醒表面。
- 不改 break 节奏规划、postpone/skip 规则或 break window 展示逻辑。

## Constraints & Assumptions
- heads-up 的主信号应优先复用已存在的 `DesktopSnapshot.status/status_detail` 与 tray 文本更新链，而不是临时拼接新的前台状态源。
- `next_notification_due_ms` 适合“一次性发送通知”，不适合作为持续可见 heads-up 阶段的唯一真源。
- 现有后台 tick 频率为 1s，本轮只能做常量级派生判断，不能引入更重的轮询或副作用。

## System Boundaries
- Modules:
- `apps/desktop/src-tauri/src/state.rs`
- `apps/desktop/src-tauri/src/engine.rs`
- `apps/desktop/src/locales/messages/*.json`
- Ownership:
- `state.rs` 负责 heads-up 阶段判定与快照状态真源
- `engine.rs` 负责系统通知投递与失败日志
- locale 负责设置与运行态的用户语义
- Dependency direction:
- `engine.rs` 只消费 `state.rs` 产出的 `EngineActions`
- 前台和 tray 只消费 `DesktopSnapshot.status/status_detail`

## API / Contract
- Signatures / Endpoints:
- 无新增外部 API；仅收紧 `DesktopSnapshot.status/status_detail` 的语义
- Request/Response schema (typed):
- `DesktopSnapshot.status` / `status_detail` 在 due 前可返回 `headsUpTitle` / `headsUpDetail|headsUpAdaptiveDetail`
- Error model (codes, retryability):
- 系统通知失败仅输出日志，不阻断 break 调度

## Data Model / Storage
- 不新增持久化字段；继续使用现有 `microbreak_notification_*` / `long_break_notification_*`
- heads-up 阶段由 `next_break_due_ms`、`next_break_kind` 与 lead time 运行时派生

## Invariants
- 仅在 `due > now` 且该 break 的 heads-up 开关开启时，允许进入 heads-up 阶段
- due 后不得继续停留在 heads-up，必须转入 `等待空档` 或 `break active`
- 关闭某类 break 的 heads-up 设置时，该类 break 不得进入 heads-up

## Concurrency / Lifecycle / Memory Model
- 继续沿用 `PauzaState.runtime: Arc<Mutex<RuntimeState>>`
- heads-up 判定是只读派生，不引入新共享字段，因此不扩大并发边界
- 系统通知失败日志发生在后台 tick 线程，不影响主线程窗口生命周期

## Observability Plan (Debug-Driven)
- Logs:
- `engine.rs` 在 `notification().show()` 失败时输出 `failed to show desktop notification: ...`
- Metrics:
- 无
- Traces:
- 无
- Debug flags:
- 无

## Security & Privacy Considerations
- 不新增权限申请、网络上报或新的本地数据采集
- 仅在 stderr 中记录通知失败文本，不包含用户隐私内容

## Risks & Rollback
- Failure modes:
- heads-up 文案可能与 due 后的 waiting 文案过于接近
- tray 属于系统表面，自动化无法完全代表真实感知
- Rollback steps:
- 回退 `heads_up_kind()`、`pre_break_heads_up_detail()` 与 `engine.rs` 日志分支
- 重跑 Rust 测试和 desktop build

## Acceptance Criteria (System)
- heads-up 阶段必须能稳定映射到 `DesktopSnapshot.status/status_detail`
- 系统通知失败不再静默
- due 后 smart / forced 逻辑保持既有语义

## Open Questions / Decision Requests
- 后续若用户仍觉得 heads-up 不够明显，再评估是否增加更强的 in-app 轻提示；本轮先不引入新的打断层。
