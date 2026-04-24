# Task-ID: TID-20260424-state-module-split

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 将 `state.rs` 中可独立归类的 settings / persistence / tests 逻辑拆出，降低 runtime 文件的职责混杂度

## Non-Goals
- 不调整 smart / forced 调度策略
- 不拆 `RuntimeState` / `PauzaState` 主状态机
- 不改动前台设置或 Rust host 对外命令面

## Constraints & Assumptions
- `crate::state` 的 public surface 需保持兼容
- 旧 `settings.json` 的 migration 与 `skip_serializing` 兼容边界必须保持
- 先拆“最稳定的边界”，tests 只做文件迁移，不扩展为测试重构

## System Boundaries
- Modules:
  - `state.rs`：runtime scheduler、snapshot、current break lifecycle、status text
  - `state/settings.rs`：`PauzaSettings`、相关 enum、shortcut、默认值与 sanitize helper
  - `state/persistence.rs`：`settings.json` 的 load / save / legacy migration
  - `state/tests.rs`：`state` 模块的 Rust 单测
- Ownership:
  - runtime 逻辑继续归 `state.rs`
  - 设置 schema 与持久化只作为 runtime 依赖，不反向持有 runtime 语义
- Dependency direction:
  - `state.rs` -> `state/settings.rs`
  - `state.rs` -> `state/persistence.rs`
  - `state/persistence.rs` -> `RuntimeState` / `PauzaSettings`
  - `state/settings.rs` 仅依赖 `crate::i18n` 与父模块常量

## API / Contract
- Signatures / Endpoints:
  - 对外仍通过 `crate::state::{PauzaState, PauzaSettings, BreakKind, ReminderMode, ShortcutAction, DesktopSnapshot}` 暴露
  - `load_settings` / `save_settings` 继续只在 `state` 模块内部使用
- Request/Response schema (typed):
  - `PauzaSettings` 的 JSON 结构不变；`idleOpportunitySeconds` 继续只读兼容、不会序列化回写
- Error model (codes, retryability):
  - 维持当前 `Result<_, String>` 模式，不新增错误码

## Data Model / Storage
- `settings.json` 仍由 `PauzaSettings` 驱动；legacy migration 继续在读取路径修正 `reminderMode`、旧音频字段与 `breakIdeasEnabled`

## Invariants
- `crate::state` 外部消费者不需要因文件拆分修改 import
- settings schema 不得重新塞回 `state.rs`
- persistence 不得开始承载 runtime 调度语义
- tests 拆出后覆盖范围与断言语义不变

## Concurrency / Lifecycle / Memory Model
- 本轮不改变 `Arc<Mutex<RuntimeState>>` 生命周期模型；子模块仅参与编译时边界，不引入新共享状态

## Observability Plan (Debug-Driven)
- Logs:
  - 不新增日志；通过现有 `state::tests::*` 回归保证拆分未改变运行时行为
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - N/A

## Security & Privacy Considerations
- 仅涉及本地配置与模块组织，不新增权限、网络或敏感数据面

## Risks & Rollback
- Failure modes:
  - re-export 不完整导致其他 host 模块无法编译
  - persistence 拆分漏掉 legacy migration 造成旧配置加载失败
- Rollback steps:
  - 回退 `state.rs` 与新增子模块
  - 重新跑 Rust / TS / Vitest 验证恢复

## Acceptance Criteria (System)
- `state/settings.rs`、`state/persistence.rs` 与 `state/tests.rs` 承接对应职责
- `state.rs` 不再包含 settings schema / persistence 实现 / inline tests
- 所有现有验证保持通过

## Open Questions / Decision Requests
- N/A
