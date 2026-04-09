# Task-ID: TID-20260409-settings-runtime-action-clarity

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 在不改变宿主命令语义的前提下，收紧设置页对 runtime action 的表达。

## Non-Goals
- 不改 `resume`、`clear_focus_session`、`reset_breaks` 在 Rust host 内的实现。
- 不新增新的状态字段或持久化配置。

## Constraints & Assumptions
- `resume` 与 `reset_breaks` 已有稳定 command 面，本轮只重新组织前台入口。
- pause / focus / reset 的业务边界以 `state.rs` 当前实现为准。
- 若新增 preview mock，也只能影响无 Tauri runtime 的浏览器 fallback。

## System Boundaries
- Modules:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/locales/overrides/{zh-CN,en}.json`
  - `apps/desktop/src/i18n.ts`（仅读取现有 lookup 能力）
- Ownership:
  - 前台负责动作显隐和说明
  - Rust host 继续负责真正的 pause/resume/focus/reset 执行
- Dependency direction:
  - UI -> Tauri commands
  - UI preview fallback -> local mock only

## API / Contract
- Signatures / Endpoints:
  - 继续使用既有 `resume_breaks`
  - 继续使用既有 `clear_focus_session`
  - 继续使用既有 `reset_breaks`
- Request/Response schema (typed):
  - 输入输出仍为 `DesktopSnapshot`
- Error model (codes, retryability):
  - 沿用当前字符串错误；UI 继续通过设置页顶部 error banner 暴露

## Data Model / Storage
- 无新增持久化字段；仅消费 `pauseUntilMs`、`pausedIndefinitely`、`focusUntilMs`、`currentBreak.strictMode`

## Invariants
- `恢复提醒` 不能在默认运行态常驻可见
- `重置节奏` 不能暗含“恢复暂停”
- focus 态不能再借用 `恢复提醒` 命名

## Concurrency / Lifecycle / Memory Model
- 继续复用现有 `busyAction` 锁，避免设置自动保存与运行时动作同时提交造成双击重复触发

## Observability Plan (Debug-Driven)
- Logs:
- Metrics: N/A
- Traces: N/A
- Debug flags: 可选的 preview query 参数仅用于浏览器 fallback 的状态展示，不进入持久化设置

## Security & Privacy Considerations
- 自定义 preview mock 不应读取系统状态或额外暴露本地路径

## Risks & Rollback
- Failure modes:
  - 运行时动作显隐条件写错，导致 `恢复提醒` 在非暂停态出现
  - `重置节奏` 说明不足，继续被误解为恢复动作
  - preview mock 影响默认浏览器 preview
- Rollback steps:
  - 回退前台动作区与 locale 文案，保留宿主命令不变

## Acceptance Criteria (System)
- 不改 Rust host contract
- 前台新增逻辑只依赖已有 `DesktopSnapshot` 字段
- preview mock 若存在，只在 `hasTauriRuntime() === false` 路径生效

## Open Questions / Decision Requests
- N/A
