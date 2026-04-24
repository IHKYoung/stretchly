# Task-ID: TID-20260424-structure-cleanup

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 清理 current reminder / break ideas 结构中仍残留的兼容表达，让 settings 模型、host 序列化和调度主流程与当前真实策略重新一致。

## Non-Goals
- 不拆分 `state.rs` 成多文件。
- 不调整 smart / forced 的策略值和用户可见行为。
- 不重做前后端 i18n / locale fallback 架构。

## Constraints & Assumptions
- 旧用户配置里可能仍含 `idleOpportunitySeconds`，载入兼容必须保留。
- 本轮允许清理该字段在新 settings/snapshot 中的继续传播。
- `tick()` 的主流程可读性提升优先于继续增加新的状态抽象。

## System Boundaries
- Modules:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src-tauri/src/state.rs`
  - `docs/{SettingsInventory,Architecture,CHANGELOG}.md`
- Ownership:
  - 前台只关心当前可编辑/可消费的设置模型
  - host 负责旧配置迁移与当前 runtime 主逻辑
- Dependency direction:
  - 前台设置模型 <- host snapshot
  - host 兼容迁移 <- 旧 settings JSON
  - 文档 <- 当前代码行为

## API / Contract
- Signatures / Endpoints:
  - 无新增命令面；沿用现有 `DesktopSnapshot.settings`
- Request/Response schema (typed):
  - `idleOpportunitySeconds` 从前台主模型中移除
  - host 仍可在反序列化时接受该字段
- Error model (codes, retryability):
  - N/A（纯本地结构整理）

## Data Model / Storage
- `PauzaSettings.idle_opportunity_seconds` 保留为兼容字段，但改为 `skip_serializing`，因此：
  - 旧配置文件仍可读
  - 新写回的 settings 文件不再继续保留该字段
  - `DesktopSnapshot.settings` 也不再继续把该字段暴露给前台

## Invariants
- smart reminder 的当前有效阈值仍由 host 内置常量驱动
- 兼容字段只能影响载入，不应重新成为现行设置真源
- due notification 与 due break 的行为不能因为 helper 抽取而改变

## Concurrency / Lifecycle / Memory Model
- 本轮不引入新的并发原语。
- `tick()` 主流程只做 helper 抽取，不改原有锁边界与 runtime 生命周期。

## Observability Plan (Debug-Driven)
- Logs:
  - 继续沿用 `runtime.actions.notificationSent`
  - 继续沿用 `runtime.actions.waitingForOpportunity`
  - 继续沿用 `runtime.actions.breakStarted`
- Metrics: N/A
- Traces: N/A
- Debug flags: N/A

## Security & Privacy Considerations
- 无新增权限、无新外部输入面、无数据外传。

## Risks & Rollback
- Failure modes:
  - 旧配置在保存后丢失兼容字段，但这不应影响当前行为
  - helper 抽取如果边界不对，可能改变 due 时序
- Rollback steps:
  - 回退 `App.tsx` / `state.rs`
  - 重新运行 Rust/Vitest/typecheck
  - 若需要，恢复兼容字段的旧序列化行为

## Acceptance Criteria (System)
- 前台主模型不再继续显式携带 `idleOpportunitySeconds`
- `PauzaSettings` 的序列化结果不再包含 `idleOpportunitySeconds`
- Rust/Vitest/typecheck 全通过，提醒行为无回归

## Open Questions / Decision Requests
- N/A
