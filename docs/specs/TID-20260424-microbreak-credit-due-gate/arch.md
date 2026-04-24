# Task-ID: TID-20260424-microbreak-credit-due-gate

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 修复 recovery credit 的生命周期边界，使其只作用于 overdue pending break。

## Non-Goals
- 不重构 reminder v2 状态机。
- 不改变 smart wait 阈值、natural break 阈值或 UI 设置结构。

## Constraints & Assumptions
- `idle_ms` 仍是唯一可用活动信号。
- recovery credit 属于投递层补偿，不应影响未到点的节奏层计划。

## System Boundaries
- Modules:
  - `apps/desktop/src-tauri/src/state.rs`
- Ownership:
  - 仅 desktop host reminder runtime。
- Dependency direction:
  - `engine.rs` 读取 `tick()` 结果投递通知/窗口；本轮只改 `state.rs` 的状态判定。

## API / Contract
- Signatures / Endpoints:
  - 保持 `PauzaState::tick(now, idle_ms, dnd_active, app_exclusion_match)` 签名不变。
- Request/Response schema (typed):
  - 保持 `EngineActions` 与 `DesktopSnapshot` 结构不变。
- Error model (codes, retryability):
  - N/A

## Data Model / Storage
- 不新增持久化字段；复用 `next_break_kind`、`next_break_due_ms`、`idle_ms`、`next_break_wait_started_ms`。

## Invariants
- recovery credit 只允许在 `pending_due_kind(now).is_some()` 时执行。
- 未到点的 pending break 不得因为 idle return 被提前 credit / defer / full reset。

## Concurrency / Lifecycle / Memory Model
- `tick()` 单线程持锁更新 `RuntimeState`；本轮只在现有生命周期判定中增加 due gate，不改变锁粒度。

## Observability Plan (Debug-Driven)
- Logs:
  - 通过 `last_action` 验证恢复结算文案只在 overdue 场景出现。
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - N/A

## Security & Privacy Considerations
- 无新增权限、无外部 IO、无隐私面扩展。

## Risks & Rollback
- Failure modes:
  - due gate 放错位置，导致 overdue return 不再 credit。
  - 误伤 natural break full reset。
- Rollback steps:
  - 回退 `state.rs` 中 recovery credit 条件与新增测试。

## Acceptance Criteria (System)
- overdue pending break 返回后仍可按设计执行 credit / defer。
- not-yet-due pending break 返回后不再被提前结算。

## Open Questions / Decision Requests
- N/A
