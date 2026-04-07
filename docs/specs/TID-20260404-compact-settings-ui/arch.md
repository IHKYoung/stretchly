# Task-ID: TID-20260404-compact-settings-ui

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 在不改动设置数据契约的前提下，收紧 Tauri 设置窗口的视觉层与窗口层约束。

## Non-Goals
- 不调整 Rust 侧 `PauzaSettings` 字段。
- 不修改任何 break 调度、tray、shortcut 或 notification 后端逻辑。

## Constraints & Assumptions
- 必须保持 `update_settings`、`toggle_autostart` 与前端 `PauzaSettings` 类型兼容。
- 本轮只允许改前台表达和主窗口尺寸，不引入新的状态来源。
- 用户已经明确要求前端朝“小巧工具窗口”方向收紧。

## System Boundaries
- Modules:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/components/ui/{button,select,segmented-control}.tsx`
  - `apps/desktop/src/styles.css`
  - `apps/desktop/src-tauri/tauri.conf.json`
- Ownership: 前端布局与控件样式由 React/Tailwind 层负责；窗口尺寸边界由 Tauri config 负责。
- Dependency direction: `tauri.conf.json` 定义窗口外壳，`App.tsx` 消费既有 snapshot/settings，基础 UI 组件为其提供统一控件样式。

## API / Contract
- Signatures / Endpoints:
  - `invoke('get_snapshot')`
  - `invoke('update_settings', { settings })`
  - `invoke('toggle_autostart')`
- Request/Response schema (typed): 前端 `PauzaSettings` 与 `DesktopSnapshot` 类型未改动。
- Error model (codes, retryability): 沿用当前前端 `runCommand` 的字符串错误提示，不新增错误码。

## Data Model / Storage
- N/A：未新增持久化字段，`settings.json` 结构不变。

## Invariants
- 四个核心分类仍映射到同一份 `PauzaSettings`。
- 保存、撤销、自动刷新 snapshot 的行为保持不变。
- 主窗口仍为单窗口入口，只是尺寸与内部布局收紧。

## Concurrency / Lifecycle / Memory Model
- 不新增并发来源；仍沿用前端每秒轮询 snapshot 的现有生命周期。
- 窗口尺寸调整发生在 Tauri 启动时，不引入新的运行时资源生命周期。

## Observability Plan (Debug-Driven)
- Logs: 依赖浏览器控制台与构建输出；不新增专门日志。
- Metrics: N/A
- Traces: N/A
- Debug flags: N/A

## Security & Privacy Considerations
- 不新增权限、不引入外部请求、不改变本地设置持久化范围。

## Risks & Rollback
- Failure modes:
  - 低分辨率下内容仍可能显得拥挤。
  - 基础控件样式收平后，若某些 break 页面继续复用这些控件，视觉会一起改变。
- Rollback steps:
  - 回退 `App.tsx`、基础 UI 组件、`styles.css` 与 `tauri.conf.json`。
  - 重新运行 typecheck/build。

## Acceptance Criteria (System)
- 不修改 `PauzaSettings` 类型和命令契约。
- `npm --prefix apps/desktop run typecheck` 与 `npm --prefix apps/desktop run build` 通过。
- 主窗口配置与前台布局变更可在 docs/evidence 中追溯。

## Open Questions / Decision Requests
- 是否需要在下一轮继续下调最小窗口尺寸，还是保持 `960x540` 作为安全下限。
