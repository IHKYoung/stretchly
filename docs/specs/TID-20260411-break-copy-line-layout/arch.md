# Task-ID: TID-20260411-break-copy-line-layout

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 让 break prompt 的长文案不再被浏览器从任意字位截断，而是按整句或分句稳定换行。
- 把分行规则抽成纯函数 helper，并补自动化测试，避免以后再因样式或字体调整回归。

## Non-Goals
- 不修改 locale 真源内容本身。
- 不调整 break prompt 的 CTA、倒计时、调度时机或宿主窗口策略。
- 不新增依赖。

## Constraints & Assumptions
- 当前问题主要发生在中文等无空格语言里，普通段落流会在任意字之间自动断行。
- 对英文等有空格语言，也应优先保持“一句一行”；只有超长句才退化为按分句换行。
- 这类排版规则应保持在前台 helper 层，而不是把逻辑散落在 JSX / CSS 中。

## System Boundaries
- Modules:
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/lib/break-copy-layout.ts`
- `test/desktopBreakCopyLayout.js`
- Ownership:
- break prompt 文案切行策略由前台 helper 管理，`App.tsx` 只负责渲染 line list。
- Dependency direction:
- locale prompt text -> `resolveBreakPromptCopy()` -> `splitBreakPromptLines()` -> break prompt line render

## API / Contract
- Signatures / Endpoints:
- `splitBreakPromptLines(text, language, variant): string[]`
- Request/Response schema (typed):
- 无新增外部 schema；仅新增前台内部 helper API
- Error model (codes, retryability):
- 无新增错误码；空字符串直接回退为空数组

## Data Model / Storage
- 无新增持久化字段或设置项；继续复用既有 `BreakPromptCopy` 与 locale 文案。

## Invariants
- 句末或分句标点必须和当前行文本一起保留，不能在换行时丢失。
- 多句文案优先按整句独立成行。
- 只有当整句超出当前版式阈值时，才允许按逗号/分号再次拆分。

## Concurrency / Lifecycle / Memory Model
- 纯前台同步 helper，无额外状态、effect 或异步依赖。
- 不引入新的 React 生命周期风险，只是把已有字符串映射为 line array。

## Observability Plan (Debug-Driven)
- Logs:
- 不新增 runtime logs；优先用单测和 browser preview 截图验证
- Metrics:
- N/A
- Traces:
- N/A
- Debug flags:
- N/A

## Security & Privacy Considerations
- 不新增外部输入、权限或网络数据流。

## Risks & Rollback
- Failure modes:
- 句宽阈值若过于激进，会把文案切得太碎。
- 阈值若过于宽松，中文仍可能在个别句子里被浏览器二次断开。
- Rollback steps:
- 回退 `break-copy-layout.ts`、`App.tsx` 与对应测试/文档。

## Acceptance Criteria (System)
- 中文长文案会按整句或分句稳定换行，不再出现从任意字位折断的阅读体验。
- 英文多句文案至少保持“一句一行”；超长单句在需要时按分句换行。
- `npm test` 与 `npm --prefix apps/desktop run build` 通过。

## Open Questions / Decision Requests
- 无。
