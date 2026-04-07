# Task-ID: TID-20260403-break-window-scale-fix

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 修正 break 页的前台容器层级与 host 默认尺寸，使 break window 尺度回到合理状态。

## Non-Goals
- 不改状态机、通知或 action command。
- 不新增新设置字段。

## Constraints & Assumptions
- 宿主 break window 已经独立存在，前台不应再人为缩成一张窄卡片。
- 浏览器 preview 需要一个 mock current break 才能稳定校验 break UI。

## System Boundaries
- Modules:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src-tauri/src/shell.rs`
- Ownership:
  - 本任务同时拥有 break 页 React 布局与 host 默认尺寸。
- Dependency direction:
  - `shell.rs` 决定宿主窗口大小
  - `App.tsx` 决定 break 页如何使用这些空间

## API / Contract
- Signatures / Endpoints:
  - 不新增命令接口，仍复用现有 break action commands。
- Request/Response schema (typed):
  - `previewSnapshot()` 在 break route 下增加 mock current break，仅用于浏览器预览。
- Error model (codes, retryability):
  - 无新增错误模型。

## Data Model / Storage
- 无新增持久化字段。

## Invariants
- break 页 CTA 语义不变。
- 真正的 runtime break 数据仍来自 `DesktopSnapshot.currentBreak`。
- mock current break 只在 `?window=break` 且无 Tauri runtime 时启用。

## Concurrency / Lifecycle / Memory Model
- 不新增并发状态；仍通过现有 `runCommand` 处理按钮动作。
- preview mock 不写回持久化状态。

## Observability Plan (Debug-Driven)
- Logs: 依赖 build/cargo/preview console。
- Metrics: 无
- Traces: 无
- Debug flags: 无

## Security & Privacy Considerations
- 不新增权限、网络或存储路径。

## Risks & Rollback
- Failure modes:
  - break 页填满窗口后，在极小尺寸下可能仍需二次压缩。
  - preview mock 逻辑若泄漏到非 break 路径会造成误导。
- Rollback steps:
  - 回退 `App.tsx`、`shell.rs` 和 docs。

## Acceptance Criteria (System)
- `shell.rs` 默认尺寸调整已编译通过。
- `App.tsx` 的 break route 结构已切换为整页布局。
- 不修改 state schema。

## Open Questions / Decision Requests
- 若后续仍嫌微休息小，下一轮可能要把 gentle 微休息从角落小窗进一步改成更明确的中心窗口。
