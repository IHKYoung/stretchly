# Task-ID: TID-20260409-smart-reminder-ui-simplification

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 收回前台对 `idleOpportunitySeconds` 的直接暴露，让 smart reminder 的策略参数继续留在 host 内部。

## Non-Goals
- 不修改 `state.rs` 的 `6s + bounded wait cap` 逻辑。
- 不新增新的 reminder mode 或 schema。

## Constraints & Assumptions
- 当前用户反馈的重点是“参数太多”，不是 host 默认策略本身错误。
- 前台减少控件后，仍需保留 `idleOpportunitySeconds` 字段以兼容现有设置读写结构。

## System Boundaries
- Modules:
  - `App.tsx`：撤回 smart reminder 阈值控件
  - `messages/{en,zh-CN}.json`：删除对应设置文案
- Ownership:
  - React 前台负责减少可见设置项
  - Rust host 继续维护实际策略
- Dependency direction:
  - 前台不再主动编辑该参数
  - host 继续读取现有设置结构和默认值

## API / Contract
- Signatures / Endpoints:
  - 无新增 API
- Request/Response schema (typed):
  - 设置 schema 不变，仅前台不再暴露该字段
- Error model (codes, retryability):
  - 无新增错误模型

## Data Model / Storage
- `idleOpportunitySeconds` 继续保留在持久化设置里，但回到 hidden setting。

## Invariants
- 用户侧不再看到 smart reminder 的阈值控件。
- host 侧 smart reminder 策略保持不变。

## Concurrency / Lifecycle / Memory Model
- 无新增并发或生命周期变化。

## Observability Plan (Debug-Driven)
- Logs:
  - 无新增
- Metrics:
  - 无
- Traces:
  - 无
- Debug flags:
  - 无

## Security & Privacy Considerations
- 无新增权限、外部调用或数据采集。

## Risks & Rollback
- Failure modes:
  - 文档口径仍写成前台可调
  - locale registry 残留已删除 key
- Rollback steps:
  - 回退 `App.tsx`、locale、测试与 docs
  - 重新同步 registry 后复验

## Acceptance Criteria (System)
- 设置页中不再出现 smart reminder 阈值控件。
- host 默认策略保持不变。

## Open Questions / Decision Requests
- 无
