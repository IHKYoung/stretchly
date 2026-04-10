# Task-ID: TID-20260409-smart-reminder-decay-thresholds

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 将 smart reminder 从单一机会阈值改成 per-kind 递减阈值曲线。
- 在不增加用户设置复杂度的前提下，让提醒既尽量礼貌，又保持最终确定性。

## Non-Goals
- 不重新把阈值暴露到设置页。
- 不新增第三种 reminder mode。
- 不改 pause / focus / DND / natural break 的阻塞边界。

## Constraints & Assumptions
- 当前产品优先级是“少配置、强默认”。
- 1s tick 的 host 状态机足以支撑阶段式阈值放宽。
- `idle_ms` 仍然只是输入/鼠标活动的近似代理，不做更复杂的专注识别。

## System Boundaries
- Modules:
  - `apps/desktop/src-tauri/src/state.rs`：smart reminder 的 due / waiting / start 判定真源
  - `apps/desktop/src/locales/messages/{en,zh-CN}.json`：相关状态与提示文案
  - `docs/ReminderScheduling.md`：长期设计说明
- Ownership:
  - Rust host 负责阈值曲线与最终 deadline
  - 前台只展示结果，不参与阶段判定
- Dependency direction:
  - `idle_ms` / due state -> host 曲线策略 -> `DesktopSnapshot.status`

## API / Contract
- Signatures / Endpoints:
  - `RuntimeState::should_wait_for_opportunity(BreakKind, now)`
  - `RuntimeState::current_smart_wait_idle_requirement_ms(BreakKind, now)`
- Request/Response schema (typed):
  - 无新增外部 API；仅调整 host 内部策略
- Error model (codes, retryability):
  - 无新增错误模型

## Data Model / Storage
- 不新增持久化字段。
- `idle_opportunity_seconds` 继续只作为兼容字段保留，不再映射为当前 runtime 的唯一阈值。

## Invariants
- Smart mode 只要 break 已 due，就只会发生三类结果：命中当前阶段空档、进入下一阶段、到最终 deadline 后直接开始。
- 微休息和休息使用不同曲线，但都保证不会无限等待。
- Forced mode 仍然立即开始，不进入等待分支。

## Concurrency / Lifecycle / Memory Model
- 状态机继续由 `engine.rs` 的 1s tick 驱动。
- `next_break_wait_started_ms` 仍是唯一“waiting for opportunity”运行时标记。

## Observability Plan (Debug-Driven)
- Logs:
  - 无新增结构化日志；排查优先看 `next_break_wait_started_ms`、`idle_ms` 和当前曲线阶段。
- Metrics:
  - 无
- Traces:
  - 无
- Debug flags:
  - 无

## Security & Privacy Considerations
- 无新增权限、外部调用或额外数据采集。

## Risks & Rollback
- Failure modes:
  - 曲线切换时机写错，导致太早或太晚开始
  - locale 文案和当前策略口径不一致
- Rollback steps:
  - 回退 `state.rs`、locale、文档与测试
  - 重新同步 registry，并重跑 Rust/前端验证

## Acceptance Criteria (System)
- 微休息使用 `6s -> 3s -> 1s -> 45s deadline`。
- 休息使用 `8s -> 4s -> 1s -> 90s deadline`。
- smart reminder 不再依赖单一 `opportunity_threshold`。

## Open Questions / Decision Requests
- 无
