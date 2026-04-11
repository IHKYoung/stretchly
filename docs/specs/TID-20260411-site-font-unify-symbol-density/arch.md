# Task-ID: TID-20260411-site-font-unify-symbol-density

## Goals
- 在不改首页 DOM 结构的前提下，把字体分配与粒子字符池收敛到统一规则。

## Non-Goals
- 不新增依赖、状态层或下载逻辑。
- 不改首页结构和交互入口。

## Constraints & Assumptions
- 站点继续保持纯静态、无依赖。
- 现有 `[data-typewriter-stage]`、`.particle` 与打字机循环继续复用。

## System Boundaries
- Modules:
  - `styles.css`：统一字体分配与粒子样式
  - `script.js`：粒子字符池、数量和尺寸
- Ownership:
  - 仅 `apps/site` 首页
- Dependency direction:
  - `script.js` 继续消费既有 DOM，不新增新的数据入口

## API / Contract
- Signatures / Endpoints:
  - N/A
- Request/Response schema (typed):
  - N/A
- Error model (codes, retryability):
  - 字符池缺失时仍应回退到默认字符，不得抛异常

## Data Model / Storage
- 无新增存储。
- 字体统一依赖现有 `@font-face` 和 `var(--font-display)`。
- 粒子字符池保存在脚本常量中。

## Invariants
- `body / prompt / button / hint / particle` 的 `font-family` 必须统一。
- `.typewriter-copy` 宽度比例仍为 `0.8`。
- 粒子数量提高后仍必须自动回收。

## Concurrency / Lifecycle / Memory Model
- 继续沿用现有 pointer / wheel / click 事件模型。
- 只提高字符粒子的数量和大小，不改变回收路径。

## Observability Plan (Debug-Driven)
- Logs:
  - 浏览器 console 0 error
- Metrics:
  - `font-family`
  - output width ratio
  - particle count / symbol counts
- Traces:
  - N/A
- Debug flags:
  - `prefers-reduced-motion`

## Security & Privacy Considerations
- 不新增网络请求、本地存储或数据采集。

## Risks & Rollback
- Failure modes:
  - 全站单字体后 prompt 失去辨识度
  - 粒子数量太高导致页面发脏
- Rollback steps:
  - 回退 `styles.css` 中的字体统一，再回退 `script.js` 的粒子密度调整

## Acceptance Criteria (System)
- 首页 console 无错误。
- DOM 度量证明整页字体统一、输出区宽度仍为 `0.8`。
- 粒子字符集合命中目标字符池。

## Open Questions / Decision Requests
- 无。
