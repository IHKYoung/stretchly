# Task-ID: TID-20260409-break-cta-tightening

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 收紧 break CTA 规则：倒计时进行中禁止提前完成，延后只允许在前 10 秒。
- 把同一规则同时落到 host command 与前端按钮显示。

## Non-Goals
- 不改 break 时长、skip 策略或 reminder 调度模型。
- 不改 locale 文案结构。

## Constraints & Assumptions
- 行为限制必须以后端为准，不能只依赖前端隐藏按钮。
- 当前用户只要求收紧 finish / postpone，不要求删除 manual finish 机制本身。

## System Boundaries
- Modules: `apps/desktop/src-tauri/src/state.rs`, `apps/desktop/src/App.tsx`
- Ownership: break CTA 可用性与对应 UI 呈现
- Dependency direction: host 状态判断 -> `DesktopSnapshot.current_break` -> 前端 CTA 显示

## API / Contract
- Signatures / Endpoints: 复用现有 `finish_current_break` / `postpone_current_break`
- Request/Response schema (typed): `CurrentBreakSnapshot { manualAwaiting, canPostpone, canSkip }`
- Error model (codes, retryability): `postpone_current_break` 继续在不可用时返回现有 disabled 错误；`finish_current_break` 不在允许时机内则返回 `false`

## Data Model / Storage
- N/A（无持久化结构变化）

## Invariants
- `finish_current_break` 只有在 `manualAwaiting` 时才允许成功。
- `can_postpone` 只在 `started_at_ms + 10_000` 之前为真。
- 前端普通倒计时阶段不再渲染完成按钮。

## Concurrency / Lifecycle / Memory Model
- 仍在现有 `PauzaState` 锁内完成 CTA 判定，不新增并发状态。

## Observability Plan (Debug-Driven)
- Logs: 沿用现有 `last_action`
- Metrics: N/A
- Traces: N/A
- Debug flags: N/A

## Security & Privacy Considerations
- 无新增权限或数据采集。

## Risks & Rollback
- Failure modes:
  - 前端按钮隐藏了，但后端仍可提前完成
  - 10 秒窗口判断写错，导致延后过早消失或持续过久
- Rollback steps:
  - 回退 `state.rs` / `App.tsx` / 测试与文档
  - 重新跑 Rust/前端验证

## Acceptance Criteria (System)
- 活跃 break 倒计时阶段调用 `finish_current_break` 不会成功。
- `Later` 只在 break 开始后的前 10 秒可用。
- Rust 测试、前端测试和 desktop build 通过。

## Open Questions / Decision Requests
- 若后续还要收紧 skip，应单开任务处理。
