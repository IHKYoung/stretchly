# Task-ID: TID-20260407-rhythm-chip-presets

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 在不改动 `PauzaSettings` 契约的前提下，只重构节奏页前端输入控件。

## Non-Goals
- 不新增 settings schema 字段。
- 不改 Rust host / Tauri command / Electron legacy。
- 不引入新的组件依赖。

## Constraints & Assumptions
- 现有四个数值字段仍然直接写回 `PauzaSettings`。
- preset 选项集固定，由前端静态定义即可满足本轮需求。
- 自动保存与快照刷新机制已经存在，不需要重做数据流。

## System Boundaries
- Modules:
  - `apps/desktop/src/App.tsx`
  - 现有 `CompactNumber` stepper
  - 现有 `Switch`
- Ownership:
  - 仅前端设置页负责控件呈现和 `updateForm()` 写值。
- Dependency direction:
  - preset 芯片 -> `updateForm()` -> 现有自动保存 effect -> Tauri `update_settings`。

## API / Contract
- Signatures / Endpoints:
  - 不新增 API；继续使用 `invoke('update_settings', { settings: form })`。
- Request/Response schema (typed):
  - `PauzaSettings` 保持不变，更新的仍是现有数值字段：`microbreakIntervalMinutes`、`microbreakDurationSeconds`、`longBreakEvery`、`longBreakDurationMinutes`。
- Error model (codes, retryability):
  - 沿用现有设置页保存失败提示；本轮不增加新的错误码。

## Data Model / Storage
- 无 schema 变化；仅改变四个既有字段在前端的赋值方式。

## Invariants
- 任何 preset 点击都必须写入合法数值，不再允许输入非 preset 值。
- 次要设置分区继续使用 `CompactNumber`，避免整个节奏页交互同时漂移。
- 自动保存、快照同步和开关字段行为保持原样。

## Concurrency / Lifecycle / Memory Model
- 无新增并发模型；仍受现有 React 状态和自动保存节流 effect 管理。
- preset 点击只是同步更新本地 form 状态，不引入额外定时器或副作用。

## Observability Plan (Debug-Driven)
- Logs: 不新增 runtime logs；以浏览器控制台、typecheck/build 输出和错误提示条为主要排查入口。
- Metrics: N/A
- Traces: N/A
- Debug flags: N/A

## Security & Privacy Considerations
- 本轮不引入新权限、网络请求或敏感数据处理。

## Risks & Rollback
- Failure modes:
  - preset 行布局过宽导致换行过多，影响节奏页整体密度。
  - 选中态不够明显，用户难以判断当前值。
- Rollback steps:
  - 直接回退 `App.tsx` 中新增的 preset 组件与布局。
  - 重新运行前端构建和预览验证恢复到 stepper 版本。

## Acceptance Criteria (System)
- 不改动 `PauzaSettings` 类型和 Tauri `update_settings` 契约。
- 通过一次点击即可把 preset 值写回对应字段并触发现有自动保存流程。
- 下方 stepper 分区和其他设置分类不出现行为回归。

## Open Questions / Decision Requests
- 无。
