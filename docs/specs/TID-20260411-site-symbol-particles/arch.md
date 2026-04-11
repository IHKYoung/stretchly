# Task-ID: TID-20260411-site-symbol-particles

## Goals
- 在不改首页 DOM 结构的前提下，强化 prompt、收敛输出宽度并替换粒子表现语义。

## Non-Goals
- 不引入新依赖或新事件层。
- 不改文案数据结构和下载路由。

## Constraints & Assumptions
- 站点继续保持纯静态、无依赖。
- 现有 `[data-typewriter-stage]`、`[data-typed-output]`、`.particle` 机制保持兼容。

## System Boundaries
- Modules:
  - `styles.css`：prompt 强化、输出区宽度、粒子字符样式
  - `script.js`：粒子字符池与粒子节点文本
- Ownership:
  - 仅 `apps/site` 首页
- Dependency direction:
  - `script.js` 继续消费既有 DOM；不新增新的数据入口

## API / Contract
- Signatures / Endpoints:
  - N/A
- Request/Response schema (typed):
  - N/A
- Error model (codes, retryability):
  - 粒子池缺失时仍需回退到默认字符，不应抛异常

## Data Model / Storage
- 无新增存储。
- 粒子字符池保存在脚本内常量。

## Invariants
- 主输出区宽度计算必须和 `window.innerWidth` 对齐到 `0.8`。
- 粒子节点仍会在动画结束后自动移除。
- prompt 强化不能影响下载按钮和舞台键盘入口。

## Concurrency / Lifecycle / Memory Model
- 仍沿用现有 pointer / wheel / click 事件模型。
- 只替换粒子节点内容和样式，不改变节点回收路径。

## Observability Plan (Debug-Driven)
- Logs:
  - 浏览器 console 0 error
- Metrics:
  - prompt 样式
  - output width ratio
  - particle symbol sample
- Traces:
  - N/A
- Debug flags:
  - `prefers-reduced-motion`

## Security & Privacy Considerations
- 不新增网络请求、本地存储或数据采集。

## Risks & Rollback
- Failure modes:
  - prompt 过强导致层级失衡
  - 粒子字符池语义跑偏
  - 输出宽度过宽打散文案节奏
- Rollback steps:
  - 回退 `styles.css` 中 prompt / width 改动，再回退 `script.js` 的字符池

## Acceptance Criteria (System)
- 首页无 console error。
- DOM 度量证明输出区宽度为 `0.8`。
- 粒子节点文本包含目标字符集合。

## Open Questions / Decision Requests
- 无。
