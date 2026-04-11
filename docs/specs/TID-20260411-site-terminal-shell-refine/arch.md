# Task-ID: TID-20260411-site-terminal-shell-refine

## Goals
- 在不改脚本行为的前提下，收敛首页 DOM 结构和样式层次。

## Non-Goals
- 不引入新的状态、依赖或运行时接口。

## Constraints & Assumptions
- 站点继续保持纯静态、无依赖。
- 现有 `script.js` 的交互逻辑和选择器尽量不变。

## System Boundaries
- Modules:
  - `index.html`：调整 prompt 与输出区层级
  - `styles.css`：移除卡片壳并改为终端式对齐
- Ownership:
  - 仅 `apps/site` 首页
- Dependency direction:
  - 不新增依赖方向；`script.js` 继续消费既有 DOM 与文案池

## API / Contract
- Signatures / Endpoints:
  - N/A
- Request/Response schema (typed):
  - N/A
- Error model (codes, retryability):
  - 首页结构调整不应破坏既有脚本选择器和键盘入口

## Data Model / Storage
- 无新增存储或数据模型。

## Invariants
- `[data-typewriter-stage]` 与 `[data-typed-output]` 继续存在。
- 下载按钮仍位于互动层之上。
- 互动层 DOM 不因结构变化失去挂载空间。

## Concurrency / Lifecycle / Memory Model
- 无新增并发或生命周期复杂度；保持现有事件绑定和节点回收路径。

## Observability Plan (Debug-Driven)
- Logs:
  - 浏览器 console 0 error
- Metrics:
  - 舞台宽度与对齐位置
- Traces:
  - N/A
- Debug flags:
  - `prefers-reduced-motion`

## Security & Privacy Considerations
- 不新增网络请求、本地存储或用户数据采集。

## Risks & Rollback
- Failure modes:
  - 去卡片后视觉重心散掉
  - 左上 prompt 影响移动端布局
- Rollback steps:
  - 回退 `index.html` 与 `styles.css` 到上一个 card stage 版本

## Acceptance Criteria (System)
- 现有脚本逻辑无需修改仍可运行。
- 控制台无错误，DOM 关键挂点保持稳定。

## Open Questions / Decision Requests
- 无。
