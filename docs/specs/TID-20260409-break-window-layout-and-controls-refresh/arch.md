# Task-ID: TID-20260409-break-window-layout-and-controls-refresh

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 将 break 页面从展示型双栏布局收口为居中的单列结构，减少与休息本身无关的视觉噪音。
- 把 CTA 暴露面限制在运行态真正允许的动作，不在主界面继续保留 `Skip`。

## Non-Goals
- 不改 `state.rs` / `shell.rs` 的 break 调度、宿主窗口或 CTA 时机判定。
- 不新增 break 设置项，也不重做 locale copy 结构。

## Constraints & Assumptions
- 必须复用现有 `DesktopSnapshot.currentBreak`、`manualAwaiting` 与 `canPostpone` 契约，不新增前后端命令。
- 界面重做只能发生在 `apps/desktop/src/App.tsx` 与 `lib/break-prompt.ts` 的前台渲染层。
- break 页面仍需兼容 windowed / fullscreen 两种宿主场景。

## System Boundaries
- Modules: `apps/desktop/src/App.tsx`, `apps/desktop/src/lib/break-prompt.ts`
- Ownership: break prompt 的视觉层级、排版、CTA 出口与配色映射
- Dependency direction: `DesktopSnapshot.currentBreak` -> `App.tsx` 条件渲染 -> `break-prompt.ts` palette / scene 配置；不反向影响 host state

## API / Contract
- Signatures / Endpoints: 无新增 command；沿用 `finish_current_break` / `postpone_current_break`
- Request/Response schema (typed): 继续使用现有 `DesktopSnapshot` / `CurrentBreakSnapshot`
- Error model (codes, retryability): 沿用现有 `busyAction` / command error handling，本轮只改哪些按钮会被渲染

## Data Model / Storage
- N/A（无持久化结构变化）

## Invariants
- break 页面必须始终只暴露当前状态允许的动作。
- `manualAwaiting` 仍是显示 `Resume work` 的唯一条件。
- 单列主视觉不能破坏现有壁纸 / 主题 / cue 文案读取链路。

## Concurrency / Lifecycle / Memory Model
- 无新增并发模型；只消费现有 snapshot 更新。
- CTA 仍通过既有 `runCommand()` 触发，不新增额外异步状态源。

## Observability Plan (Debug-Driven)
- Logs: 沿用现有 `lastAction` 与 command error 提示
- Metrics: N/A
- Traces: 通过 `App.tsx` 条件渲染路径与 build/test 输出做验证
- Debug flags: 无

## Security & Privacy Considerations
- 无新增权限、网络、日志上报或敏感数据处理。

## Risks & Rollback
- Failure modes:
  - 去掉 `Skip` 后若没有保留正确的替代动作，可能让 break 页面出口过窄
  - 单列收敛若破坏当前壁纸/主题层次，可能导致 break 页面可读性下降
- Rollback steps:
  - 回退 `App.tsx` 与 `break-prompt.ts` 的布局和 CTA 渲染逻辑
  - 重新执行 `npm test`、desktop build 与 docs validator

## Acceptance Criteria (System)
- break 页面采用居中的单列内容区，不再保留旧的展示型双栏结构。
- 倒计时与进度改为数字主视觉 + 细进度条。
- break 页面主界面不再显示 `Skip`，只保留与当前 break 状态一致的 CTA。

## Open Questions / Decision Requests
- 后续是否要把“跳过当前休息”迁移到更隐蔽的次级入口，而不是彻底不提供，可留待单独任务再讨论。
