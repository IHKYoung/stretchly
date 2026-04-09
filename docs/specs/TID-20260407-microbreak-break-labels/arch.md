# Task-ID: TID-20260407-microbreak-break-labels

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 在不改动设置 schema、命令和内部类型的前提下，统一当前用户可见术语。

## Non-Goals
- 不重命名 `longBreak*` / `miniBreak*` 字段、类型、命令、迁移或测试描述。
- 不修改调度、tray 结构、break 行为和保存逻辑。
- 不回写历史 spec / logs / evidence。

## Constraints & Assumptions
- 共享 locale 已是当前前台 UI、tray 与 runtime 文案的主要来源。
- legacy app 仍带有历史内部命名，需要兼容性优先。
- 这次改动只能触碰现行文案层，不能引入 schema 漂移。

## System Boundaries
- Modules:
  - `apps/desktop/src/locales/{zh-CN,en}.json`
  - `app/locales/{zh-CN,en}.json`
  - 当前 README / docs / metainfo
- Ownership:
  - locale 与说明文档负责当前用户可见命名；内部字段与命令保持原有所有权。
- Dependency direction:
  - UI / tray / runtime surface -> locale key -> 现有内部 `longBreak*` / `miniBreak*` 数据字段。

## API / Contract
- Signatures / Endpoints:
  - 不新增 API，不改 Tauri / Electron 命令签名。
- Request/Response schema (typed):
  - `PauzaSettings`、`BreakKind` 和历史 preference key 保持不变。
- Error model (codes, retryability):
  - 本轮无新增错误模型；沿用现有构建/运行时错误输出。

## Data Model / Storage
- 无数据结构、迁移或存储 key 变化。

## Invariants
- `BreakKind` 仍保持 `'microbreak' | 'longBreak'`。
- `longBreak*` / `miniBreak*` / `break*` 等内部兼容标识不被重命名。
- locale key 到用户可见术语是一对一映射，不应出现“设置页改了，tray/runtime 没改”的漂移。

## Concurrency / Lifecycle / Memory Model
- 不适用；本轮不引入新的状态流、线程模型或宿主生命周期变更。

## Observability Plan (Debug-Driven)
- Logs: 依赖 `git diff`、浏览器 preview 控制台和构建输出定位遗漏文案。
- Metrics: N/A
- Traces: N/A
- Debug flags: N/A

## Security & Privacy Considerations
- 无新权限、无新外部请求、无敏感数据路径变化。

## Risks & Rollback
- Failure modes:
  - 仍有未走 locale 的硬编码文案残留，导致新旧术语混用。
  - 文档更新不完整，误导后续维护者去改 schema。
- Rollback steps:
  - 直接回退本轮 locale / docs / metadata 修改。
  - 重跑测试与构建，确认行为未受影响。

## Acceptance Criteria (System)
- 内部数据结构和命令契约保持不变。
- 当前共享 locale 输出的新术语覆盖主设置页、tray skip 和 runtime break 文案。
- 现行说明文档明确“用户术语已改、内部兼容标识保留”。

## Open Questions / Decision Requests
- 无。
