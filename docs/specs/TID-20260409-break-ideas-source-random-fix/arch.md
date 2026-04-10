# Task-ID: TID-20260409-break-ideas-source-random-fix

## Goals
- 把 break prompt 的文案来源收敛为 `miniBreakIdeas` / `longBreakIdeas`。
- 删除 `ui.breakCopy.prompts.*` 过渡字段与相关 fallback 逻辑。
- 修正“固定节奏总是命中同一条 prompt”的索引缺陷。

## Non-Goals
- 不改 break 页视觉结构、CTA 语义或倒计时逻辑。
- 不改 `breakIdeasEnabled = false` 时默认文案仍由 `ui.breakCopy.defaultPrompt.*` 提供的行为。

## Constraints & Assumptions
- break 页每秒会重渲染，prompt 选择必须在同一 break 内保持稳定，不能每 tick 切一条。
- locale ideas 结构跨语言保持为 `miniBreakIdeas.<id>.text` / `longBreakIdeas.<id>.text`。

## System Boundaries
- Modules:
  - `apps/desktop/src/i18n.ts`
  - `apps/desktop/src/lib/break-ideas.ts`
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/locales/messages/*.json`
- Ownership: 前端 locale lookup、prompt 选择 helper、break 页渲染。
- Dependency direction: `App.tsx` -> `break-ideas.ts` -> `i18n.ts` -> `registry.generated.json`

## API / Contract
- Signatures / Endpoints:
  - `tBreakIdeaList(language, kind) -> string[]`
  - `pickBreakPromptIndex(kind, startedAtMs, sourceLength) -> number`
  - `pickBreakPrompt(language, kind, startedAtMs) -> string`
- Request/Response schema (typed): 无新增 IPC；仅前端 helper contract。
- Error model (codes, retryability): 无显式 error code；当 ideas 列表为空时返回空字符串，由 `App.tsx` 回退到 `ui.breakCopy.defaultPrompt.*`。

## Data Model / Storage
- 保留：
  - `miniBreakIdeas`
  - `longBreakIdeas`
  - `ui.breakCopy.defaultPrompt`
- 删除：
  - `ui.breakCopy.prompts`

## Invariants
- 相同 `startedAtMs` 的 break 在重渲染期间必须得到同一条 prompt。
- 默认 10 分钟微休息 / 30 分钟长休息的固定节奏不应因简单取模而反复命中同一条。

## Concurrency / Lifecycle / Memory Model
- 无共享可变状态；prompt index 由 `kind + startedAtMs` 纯函数计算。
- 由于 break 页会按秒刷新，不能在 render 中直接使用 `Math.random()`。

## Observability Plan (Debug-Driven)
- Logs: 无新增 runtime log。
- Metrics: 无。
- Traces: 无。
- Debug flags: 无。

## Security & Privacy Considerations
- 仅处理本地 locale 文案，无用户隐私或外部输入边界变化。

## Risks & Rollback
- Failure modes:
  - 新 helper 接错 locale 路径，导致 prompt 为空并总是走默认文案。
  - 删除 `ui.breakCopy.prompts` 后仍有残留代码读取旧 key。
- Rollback steps:
  1. 回退 `break-ideas.ts` / `i18n.ts` / `App.tsx`。
  2. 恢复 `messages/{en,zh-CN}.json` 中的 `ui.breakCopy.prompts`。
  3. 重新执行 locale sync、test、typecheck、build。

## Acceptance Criteria (System)
- break prompt 读取 `miniBreakIdeas` / `longBreakIdeas` 的 `.text` 列表。
- `ui.breakCopy.prompts` 不再存在于当前桌面端 locale 真源。
- 固定 10m/30m 节奏下，prompt index 不再与简单取模一样永远锁死。

## Open Questions / Decision Requests
- N/A
