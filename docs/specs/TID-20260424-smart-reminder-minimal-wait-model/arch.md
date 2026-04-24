# Task-ID: TID-20260424-smart-reminder-minimal-wait-model

## Goals
- 将 smart reminder 收缩为固定阈值 + 最长等待的最小模型。
- 保留 `BreakActive` 优先级与 passive blocker freeze，不回退这两项已经修正的实现语义。

## Non-Goals
- 不引入新的 interruptibility score、activity model 或 machine learning 信号。
- 不新增设置项或更改现有前端交互结构。
- 不改 break planner 的节奏来源，只改“已到点 break 的投递时机”。

## Constraints & Assumptions
- 调度仍只依赖 `idle_ms`。
- `Smart` 与 `Forced` 的主要差异只体现在 `due_at` 之后的投递决策。
- `Natural breaks` 继续表示“长时间离开后整轮重排”，不是第三种提醒模式。

## System Boundaries
- Modules:
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src/locales/messages/*.json`
  - `docs/ReminderScheduling.md`
- Ownership:
  - 本轮只动 desktop host reminder 调度与对应文案/文档。
- Dependency direction:
  - 节奏层继续产出 `next_break_kind / next_break_due_ms`
  - 调度层消费这些字段并决定是否开始 break

## API / Contract
- Signatures / Endpoints:
  - 无新增外部 API；仅调整 `RuntimeState::should_wait_for_opportunity()`、tick 路径与相关状态展示。
- Request/Response schema (typed):
  - N/A
- Error model (codes, retryability):
  - N/A

## Data Model / Storage
- 不新增持久化字段。
- `idle_opportunity_seconds` 保留为兼容字段，但当前等待逻辑改由内置固定阈值驱动。

## Invariants
- `due_at` 之前绝不开始 break。
- `Forced` 到点即开始，不进入等待空档状态。
- `Smart` 只在“idle 达标”或“已等满 max wait”两种条件下开始。
- passive blocker 只冻结投递，不重置节奏。
- `NaturalBreak` 返回后从当前时间重排。

## Concurrency / Lifecycle / Memory Model
- `tick()` 继续在单一 runtime state 上串行推进。
- 当前 break 生命周期优先于 blocker / waiting / natural break 判定。
- pending schedule 的时间平移仍由现有 freeze/resume 逻辑处理。

## Observability Plan (Debug-Driven)
- Logs:
  - 复用 `last_action`，重点看 `waitingForOpportunity`、`breakStarted`、`naturalBreakFinished`、`dndEnded`、`appExclusionCleared`。
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - N/A

## Security & Privacy Considerations
- 本轮不新增权限、不接入网络、不新增外部依赖。

## Risks & Rollback
- Failure modes:
  - 固定阈值过高会导致等待过久。
  - 最大等待判定若与 due 时间线脱节，可能导致永远不开始或过早开始。
- Rollback steps:
  - 回退 `state.rs`、locale 与本任务 docs 到变更前版本。

## Acceptance Criteria (System)
- `Smart` 不再进入 recovery hold，也不再执行 recovery credit / defer。
- `Smart` 的等待条件固定为微休息 `8s / 90s`、长休息 `12s / 180s`。
- `Forced` 到点立即开始；`NaturalBreak` 与 passive blocker 语义保持现状。

## Open Questions / Decision Requests
- 无；阈值与最长等待已在当前会话中由用户确认方向后直接实现。
