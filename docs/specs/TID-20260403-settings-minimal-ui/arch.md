# Task-ID: TID-20260403-settings-minimal-ui

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 在不改 Rust host 契约的前提下，重写前端设置页 IA。
- 保留完整 `PauzaSettings` round-trip 行为，避免隐藏字段丢失。

## Non-Goals
- 不新增或删除 `PauzaSettings` 字段。
- 不改 `get_snapshot`、`update_settings`、`toggle_autostart` 的命令签名。
- 不改调度状态机、tray、shortcut 或 break window 运行逻辑。

## Constraints & Assumptions
- 现有 `state.rs` 是设置真源，前端只能决定“展示哪些字段”，不能改 schema。
- `toggle_autostart` 继续是独立命令，不并入 `update_settings`。
- 前端需要保留 `?window=break` 路径。

## System Boundaries
- Modules:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/styles.css`
  - `apps/desktop/src/components/ui/card.tsx`
  - `apps/desktop/src/locales/{zh-CN,en}.json`
  - `apps/desktop/src-tauri/src/{state,commands}.rs`（只读依赖）
- Ownership:
  - 本任务只拥有 React/Tailwind 前端显示层与文案层。
- Dependency direction:
  - React UI 依赖 `DesktopSnapshot` / `PauzaSettings` / command API。
  - Rust host 不依赖本次 UI 结构。

## API / Contract
- Signatures / Endpoints:
  - `get_snapshot() -> DesktopSnapshot`
  - `update_settings({ settings: PauzaSettings }) -> DesktopSnapshot`
  - `toggle_autostart() -> DesktopSnapshot`
- Request/Response schema (typed):
  - `form` 继续持有完整 `PauzaSettings`，即使 UI 只显示其中一部分字段。
- Error model (codes, retryability):
  - 无新增错误码；前端继续将命令失败直接展示在顶部错误条。

## Data Model / Storage
- 数据模型不变；设置仍由 Rust host 持久化到现有 settings 存储。
- 本轮只改变前端可见字段集合，不改变存储结构。

## Invariants
- 未暴露的设置字段在保存时必须原样带回 `update_settings`。
- `dirty` 状态存在时，轮询返回的新 `snapshot.settings` 不覆盖用户本地表单。
- `autostartEnabled` 继续走 snapshot 独立字段，不与 `form.language` 等一起等待保存。

## Concurrency / Lifecycle / Memory Model
- `snapshot` 每秒轮询一次；若 `dirty=false`，表单从 snapshot 同步；若 `dirty=true`，保留本地编辑。
- `runCommand` 在 preview 模式下走本地 transform，在 Tauri runtime 下走真实 command。
- `breakMode` 与 settings mode 共用一个 `App.tsx`，但路径分支互斥。

## Observability Plan (Debug-Driven)
- Logs: 无新增日志；继续依赖 host action 文案和前端错误条。
- Metrics: 无。
- Traces: 无。
- Debug flags: 无。

## Security & Privacy Considerations
- 不新增网络请求、不新增权限、不引入新的本地敏感数据。
- app exclusion 文本仍为用户本地配置内容，不额外上传或处理。

## Risks & Rollback
- Failure modes:
  - 文案键漏配导致 UI 回落为 key 文本。
  - `App.tsx` 重写时误删保存路径或 break window 分支。
- Rollback steps:
  - 回退上述前端文件与 docs，即可恢复到上一个 fixed split 版本。

## Acceptance Criteria (System)
- `update_settings` / `toggle_autostart` 的调用路径未变化。
- 构建和类型检查通过。
- 无 host 侧文件变更。

## Open Questions / Decision Requests
- 是否为低频设置单独提供二级入口，后续视用户决策再展开。
