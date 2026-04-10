# Task-ID: TID-20260410-settings-language-switch-freeze

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 保持设置页自动保存不变，同时避免语言选择在宿主持久化完成前触发整页语言/方向切换。

## Non-Goals
- 不改 Rust host 命令、locale registry 或任何翻译内容。

## Constraints & Assumptions
- 不能新增依赖。
- 当前前台以 `DesktopSnapshot.settings` 作为已持久化真源，`form` 只是草稿层。
- 根因假设是前台生命周期重入，而不是 host 死锁或 locale 文件损坏。

## System Boundaries
- Modules:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/i18n.ts`
  - `test/desktopSettingsControls.js`
- Ownership:
  - 前台显示语言与 effect 依赖边界由 `App.tsx` 管理，语言归一化 helper 由 `i18n.ts` 管理。
- Dependency direction:
  - `DesktopSnapshot.settings.language` -> `resolveUiLanguage()` -> 设置页 labels / `dir/lang`
  - `form.language` -> 语言下拉当前值 -> autosave -> `update_settings`

## API / Contract
- Signatures / Endpoints:
  - `resolveUiLanguage(draftLanguage?: AppLanguage, persistedLanguage?: AppLanguage): AppLanguage`
  - 继续复用 `invoke('get_snapshot')` 与 `invoke('update_settings', { settings: form })`
- Request/Response schema (typed):
  - 无新增 schema；仅调整前台消费 `DesktopSnapshot.settings.language` 的时机
- Error model (codes, retryability):
  - 无新增错误码；加载与保存失败继续沿用前台现有 error UI

## Data Model / Storage
- 无新增字段；`settings.json` 持久化结构保持不变。

## Invariants
- 一旦存在 snapshot，设置页可见语言必须与 `snapshot.settings.language` 保持一致。
- 语言下拉仍显示草稿值，直到保存成功后 snapshot 追上该值。
- snapshot 轮询不应再被草稿语言变化重启。

## Concurrency / Lifecycle / Memory Model
- 语言下拉关闭、autosave 与整页 rerender 不再共享同一个“草稿语言立即生效”触发器。
- `useEffectEvent` 用于在稳定轮询 effect 中读取最新错误文案，避免为语言文案刷新重建轮询定时器。

## Observability Plan (Debug-Driven)
- Logs:
  - 继续依赖前台已有的 load/save error message；不新增日志面
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - N/A

## Security & Privacy Considerations
- 无新增外部输入、权限或隐私数据流。

## Risks & Rollback
- Failure modes:
  - 若 snapshot 一直拿不到新语言，UI 会短暂停留在旧语言，但不会卡死。
- Rollback steps:
  - 回退 `resolveUiLanguage()` 与 `App.tsx` 对 snapshot 语言的依赖，恢复原来的草稿直驱模式。

## Acceptance Criteria (System)
- 不再出现草稿语言直接驱动整页语言翻转。
- 轮询 effect 在语言选择过程中保持稳定。
- 自动化验证通过。

## Open Questions / Decision Requests
- 无。
