# Task-ID: TID-20260409-apps-locale-single-source-refactor

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 让 `apps/desktop/src/locales/messages/*.json` 成为当前桌面端唯一有效的用户文案真源。
- 删除 `break-message-copy.ts` / `break-message-copy.json` 这条并行读取链路。
- 保持现有 break prompt 行为与文案结果不变，只改变 source of truth 和运行时读取路径。

## Non-Goals
- 本轮不新增 `app/` 物理删除。
- 本轮不新增非 `desktopReady` 语言的 break prompt 人工翻译。
- 本轮不重做设置页或 break prompt 的布局、状态机与交互模型。

## Constraints & Assumptions
- `apps/desktop/src/locales/config/*.json` 只承担语言元信息职责，不能继续塞用户文案。
- 当前 `desktopReady` 语言只有 `zh-CN` 与 `en`；其它语言可以继续通过 fallback 获得 break prompt 文案。
- `registry.generated.json` 的数据结构不变，前端与 Rust host 仍共用这一份 locale registry。

## System Boundaries
- Modules:
  - `apps/desktop/src/{App.tsx,lib/break-prompt.ts,i18n.ts}`
  - `apps/desktop/src/locales/{messages/**,config/**,registry.generated.json}`
  - `test/desktopBreakCopySource.js`
  - `docs/{RepositoryGuidelines,CodeMap,Architecture,UI,CHANGELOG}.md`
  - 本任务 specs / daily plans / daily logs
- Ownership:
  - 所有桌面端可见文案统一归 `apps/desktop/src/locales/messages/*.json` 维护。
  - `config/*.json` 只定义 `code/label/fallback/direction/desktopReady`，不承载任何用户文案。
  - `App.tsx` 与 `break-prompt.ts` 只消费 `i18n.ts` 暴露的 lookup 接口，不再自己维护另一份 break 文案数据。
- Dependency direction:
  - `messages/*.json` + `config/*.json` -> `registry.generated.json`
  - `i18n.ts` -> `registry.generated.json`
  - `App.tsx` / `break-prompt.ts` -> `i18n.ts`
  - 不允许再出现 `App.tsx` / `break-prompt.ts` -> `break-message-copy.*`

## API / Contract
- Signatures / Endpoints:
  - `t(language, key, vars?) -> string`
  - `tList(language, key) -> string[]`
  - `pickBreakPrompt(language, kind, startedAtMs) -> string`
- Request/Response schema (typed):
  - `registry.generated.json = { defaultLanguage, languages, bundles }`
  - `ui.breakCopy = { clearedDetail, manualAwaiting, awaitingFinish, actions, defaultPrompt, prompts }`
- Error model (codes, retryability):
  - 若 `messages/config` 不配对，`scripts/sync_desktop_locales.py` 直接失败退出。
  - 若某语言缺少 `ui.breakCopy.*`，运行时通过 `i18n.ts` fallback 链回退到默认语言文案。

## Data Model / Storage
- break prompt 专属文案存储在 `apps/desktop/src/locales/messages/{en,zh-CN}.json` 的 `ui.breakCopy.*`。
- `config/*.json` 只保存语言元信息，不再包含任何 break 文案覆盖。
- `break-message-copy.ts` / `break-message-copy.json` 在本轮删除，不再作为数据模型的一部分存在。

## Invariants
- 当前桌面端所有可见文案都必须可从 `messages/*.json` 追溯。
- `config/*.json` 不得新增用户文案字段。
- break prompt 运行时不得绕过 `registry.generated.json` 另读独立 JSON。

## Concurrency / Lifecycle / Memory Model
- 本任务不引入新的并发模型；只改 break prompt 文案的读取路径。
- break prompt 的生命周期仍由当前 `currentBreak` / `manualAwaiting` 状态驱动；本轮不改变 timer、sound 或 window 生命周期。

## Observability Plan (Debug-Driven)
- Logs:
  - 用 `rg` 审计是否仍残留 `break-message-copy` 引用。
  - 用 `sync_desktop_locales.py` 的失败输出来暴露 locale 结构缺失。
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - N/A

## Security & Privacy Considerations
- 本轮仅调整本地代码、locale 资源、测试与文档，不涉及新增权限、敏感数据、密钥或网络调用。

## Risks & Rollback
- Failure modes:
  - `App.tsx` 或 `break-prompt.ts` 残留旧 import，会导致 build/typecheck 失败。
  - `ui.breakCopy.*` key 未补齐，会导致 break prompt 在运行时显示 key 或落回错误语言。
  - 文档若仍保留旧口径，会继续诱导维护者新增并行文案入口。
- Rollback steps:
  - 回退 `App.tsx`、`break-prompt.ts`、`messages/{en,zh-CN}.json`、测试与相关 docs。
  - 如需临时恢复旧方案，再单独回滚删除的 `break-message-copy.*` 与相应 import。

## Acceptance Criteria (System)
- AC1: `apps/desktop/src/App.tsx` 与 `apps/desktop/src/lib/break-prompt.ts` 不再 import `break-message-copy.*`，而是只使用 `i18n.ts`。
- AC2: break prompt 专属文案统一落在 `apps/desktop/src/locales/messages/*.json` 的 `ui.breakCopy.*`。
- AC3: `apps/desktop/src/locales/break-message-copy.ts` 与 `apps/desktop/src/locales/break-message-copy.json` 被删除。
- AC4: `apps/desktop/README.md`、`docs/{RepositoryGuidelines,CodeMap,Architecture,UI,CHANGELOG}.md` 与本任务 docs 明确声明 `messages` 是唯一文案真源。
- AC5: locale sync、typecheck、build、test 与 docs validator 全部通过。

## Open Questions / Decision Requests
- 后续若桌面端继续把更多语言切为 `desktopReady`，是否需要同步为 `ui.breakCopy.*` 补齐正式翻译。
