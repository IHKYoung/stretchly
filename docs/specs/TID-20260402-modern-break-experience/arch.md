# Task-ID: TID-20260402-modern-break-experience

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 在不推翻现有 Electron 结构的前提下，为 break 窗口引入可配置的打断强度。
- 把“保护心流”的能力放进 tray 与窗口创建链路，而不是额外新系统。

## Non-Goals
- 不重构 `BreaksPlanner` 基础排程算法。
- 不引入新的持久化层或远程配置。

## Constraints & Assumptions
- 必须兼容现有 `fullscreen`、`showBreaksAsRegularWindows`、strict mode、all screens 等配置。
- 新增设置应保持 `electron-store` 简单 key/value 模式。

## System Boundaries
- Modules:
  - `app/main.js`：break 窗口创建、tray menu、settings 保存
  - `app/utils/defaultSettings.js`：新增设置默认值
  - `app/*break*` / `app/*welcome*` / `app/*preferences*`：渲染层与样式
  - `test/**`：新增纯逻辑覆盖
- Ownership: 主进程负责行为与菜单，renderer 负责展现与轻交互
- Dependency direction: settings -> main window policy -> preload/context bridge -> renderer UI

## API / Contract
- Signatures / Endpoints:
  - `settings.get/saveSettings`
  - `send-mini-break-data`
  - `send-long-break-data`
- Request/Response schema (typed):
  - break data payload 在现有数组基础上附加 interruption style 等字段
- Error model (codes, retryability):
  - 无新增错误模型；沿用现有窗口与 IPC 生命周期

## Data Model / Storage
- 新增 `breakPromptStyle` 持久化字段，取值为 `gentle | balanced | immersive`

## Invariants
- strict mode 仍然优先于柔和交互语义
- 现有 pause/resume/reset/skip 命令保持可用
- renderer 不直接决定调度，仅消费主进程下发的 break 状态

## Concurrency / Lifecycle / Memory Model
- break window 仍由主进程统一创建与回收
- tray menu 仅调用现有 `pauseBreaks` / `resumeBreaks` / `skipTo*` 函数，不增加新后台循环

## Observability Plan (Debug-Driven)
- Logs:
  - 新设置变更继续通过统一 settings 变更日志记录
  - focus session / break style 触发路径在主进程留日志
- Metrics: 无
- Traces: 无
- Debug flags: 复用 debug 面板查看当前 breakPlanner 状态

## Security & Privacy Considerations
- 不新增网络请求，不新增敏感数据采集

## Risks & Rollback
- Failure modes:
  - 新窗口尺寸或透明样式在个别平台表现异常
  - 新 IPC payload 字段导致 renderer 解构错误
- Rollback steps:
  - 回滚新增设置与窗口策略逻辑
  - 回滚新样式与新 DOM 结构

## Acceptance Criteria (System)
- 新增设置能被保存、读取并影响窗口行为
- tray focus session 能稳定调用现有 pause 逻辑
- 既有测试集不回退

## Open Questions / Decision Requests
- 当前不需要额外用户决策，先以单 app 内的 gentle 默认推进
