# Task-ID: TID-20260409-smart-reminder-wait-guard

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 将 smart reminder 从“只看 idle 阈值”的单一条件，收紧为“短空档阈值 + 有界等待上限”的投递策略。
- 保持 forced mode、pause/focus/DND/app exclusion 等既有状态机边界不变。

## Non-Goals
- 不重新拆分 microbreak / long break 的 reminder mode。
- 不新增新的持久化 schema 或迁移版本号。
- 不重做 tray / notification / break window 的 UI 结构。

## Constraints & Assumptions
- `idle_opportunity_seconds` 已存在于 `PauzaSettings`，但此前没有前台入口。
- 当前 1s tick 仍是 break 调度的唯一驱动，不额外引入更高频率的状态机循环。
- 用户真正感知到的问题是默认阈值偏长与无限等待，而不是 host 无法识别 idle signal。

## System Boundaries
- Modules:
  - `state.rs`：smart reminder 的 due / waiting / start 判定真源
  - `App.tsx`：设置页智能暂停阈值的前台输入入口
  - `messages/{en,zh-CN}.json`：相关设置与运行时文案
- Ownership:
  - Rust host 负责真正的等待阈值和 wait cap 判定
  - React 前台只负责展示和修改 `idleOpportunitySeconds`
- Dependency direction:
  - 前台设置 -> `PauzaSettings` -> host 状态机
  - runtime status -> `DesktopSnapshot` -> 前台展示

## API / Contract
- Signatures / Endpoints:
  - `PauzaState::update_settings(PauzaSettings)`
  - `RuntimeState::should_wait_for_opportunity(BreakKind, now)`
- Request/Response schema (typed):
  - 设置结构不新增字段，只调整 `idle_opportunity_seconds` 默认值与有效语义
- Error model (codes, retryability):
  - 无新增错误模型

## Data Model / Storage
- 继续沿用现有 `idleOpportunitySeconds` 持久化字段，不引入新配置字段。
- bounded wait cap 为运行时派生规则，不单独入库。

## Invariants
- Smart mode 到点后只有两种路径：等到短空档，或等到 bounded wait cap 后自动开始。
- Forced mode 仍然立即开始，不进入等待空档分支。
- 当前 break / queued break 的既有不被设置更新打断的规则保持不变。

## Concurrency / Lifecycle / Memory Model
- 状态机仍由 `engine.rs` 的 1s tick 驱动。
- `next_break_wait_started_ms` 继续作为“已 due 且正在等待空档”的运行时标记，直到 break 真正开始或等待计划被清空。

## Observability Plan (Debug-Driven)
- Logs:
  - 无新增结构化日志；调试优先观察 `next_break_wait_started_ms`、`idle_ms`、`idle_opportunity_seconds` 与 `smart_wait_cap_ms(kind)` 的关系。
- Metrics:
  - 无
- Traces:
  - 无
- Debug flags:
  - 无

## Security & Privacy Considerations
- 仅使用既有 idle / DND / app exclusion 信号，不新增权限、外部调用或用户数据采集。

## Risks & Rollback
- Failure modes:
  - waiting 状态在 snapshot 中过早消失或过久残留
  - 设置页控件与 host 默认值不一致
- Rollback steps:
  - 回退 `state.rs`、`App.tsx`、locale JSON、测试与 docs
  - 重新生成 locale registry，并重跑 test/typecheck/build

## Acceptance Criteria (System)
- Smart mode 默认 idle 阈值收紧到 `6s`。
- Smart mode 在持续输入下不再无限等待；达到 bounded wait cap 后会自动开始 break。
- Forced mode 现有立即开始行为不变。

## Open Questions / Decision Requests
- 无
