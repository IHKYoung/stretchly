# Task-ID: TID-20260424-break-ideas-asset-migration

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 将 break ideas 从 UI messages 中拆分为独立 JSON 资产目录。
- 保留 `miniBreakIdeas / longBreakIdeas` 的语义结构，不把内容改写成代码常量。
- 让运行时通过独立 registry 读取 ideas，并用数据标签标记 `official / legacy` 语言。

## Non-Goals
- 本轮不删除任何 legacy 语言 break ideas 文件。
- 本轮不改动 Rust host i18n、提醒策略或 UI 可见交互。
- 本轮不重新设计 break idea 文案内容。

## Constraints & Assumptions
- 必须保留 `miniBreakIdeas / longBreakIdeas.<id>.text` 这组稳定内容结构。
- `official / legacy` 的语言边界必须来自数据文件，而不是 TS 常量。
- 未来允许删除某些 legacy ideas bundle，因此运行时必须支持“无 bundle 时沿 locale fallback 链继续找”。

## System Boundaries
- Modules:
  - `apps/desktop/src/locales/messages/*.json`
  - `apps/desktop/src/locales/break-ideas/{messages/*.json,registry.json,registry.generated.json}`
  - `apps/desktop/src/lib/break-ideas.ts`
  - `scripts/{sync_desktop_locales.py,sync_desktop_break_ideas.py}`
- Ownership:
  - UI 文案：`messages/*.json`
  - break ideas 内容资产：`break-ideas/messages/*.json`
  - break ideas 语言分层元数据：`break-ideas/registry.json`
  - 运行时选择逻辑：`lib/break-ideas.ts`
- Dependency direction:
  - `App.tsx` -> `lib/break-ideas.ts` -> `break-ideas/registry.generated.json`
  - `messages/*.json` 与 `break-ideas/messages/*.json` 分别独立，不再互相嵌套

## API / Contract
- Signatures / Endpoints:
  - `breakIdeaEntries(language, kind) -> BreakIdeaEntry[]`
  - `pickBreakPromptEntry(language, kind, startedAtMs) -> BreakIdeaEntry | null`
- Request/Response schema (typed):
  - `BreakIdeaRegistryLanguage = { code, fallback, tier, available }`
  - `BreakIdeaRegistry = { defaultLanguage, officialLanguages, languages, bundles }`
- Error model (codes, retryability):
  - 本地 JSON 缺失 / 非法时由 sync script 直接失败，不把坏数据放进 generated registry

## Data Model / Storage
- `break-ideas/messages/<code>.json`
  - 顶层仅允许 `miniBreakIdeas` 与 `longBreakIdeas`
  - 每条 idea 保留原有稳定 id
- `break-ideas/registry.json`
  - 只维护 `defaultLanguage` 与 `officialLanguages`
- `break-ideas/registry.generated.json`
  - 聚合 locale config fallback、tier 标签、available 标记和全部 ideas bundles

## Invariants
- `messages/*.json` 不允许再包含 `miniBreakIdeas / longBreakIdeas`
- break ideas 的唯一内容真源只能是 `break-ideas/messages/*.json`
- `lib/break-ideas.ts` 只能读取 registry，不承载内容常量
- `official / legacy` 只能来自 registry 数据，不允许在代码中写死语言列表

## Concurrency / Lifecycle / Memory Model
- 前端在模块初始化时一次性读取 `break-ideas/registry.generated.json`
- 每次取 prompt 时只做纯内存查找与 fallback 遍历，不引入异步 IO

## Observability Plan (Debug-Driven)
- Logs:
  - 无新增 runtime log；排查优先看 generated registry、Vitest 断言和 sync script 输出
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - N/A

## Security & Privacy Considerations
- 全部为本地静态文案资产，无用户隐私或外部权限变更

## Risks & Rollback
- Failure modes:
  - 某些调用路径仍偷偷读取 `messages/*.json` ideas key
  - 某语言未来没有 bundle 时 fallback 链断裂
  - generated registry 与源资产不同步
- Rollback steps:
  1. 回退 `break-ideas/**`、`messages/*.json` 的 ideas 拆分与 `lib/break-ideas.ts`
  2. 重新运行 `scripts/sync_desktop_locales.py`
  3. 恢复旧测试断言与 docs 说明

## Acceptance Criteria (System)
- `messages/*.json` 不再包含 `miniBreakIdeas / longBreakIdeas`
- `break-ideas/messages/*.json` 成为唯一内容真源
- runtime 只从 `break-ideas/registry.generated.json` 读取 ideas
- `official / legacy` 标签来自数据 registry，而不是代码常量

## Open Questions / Decision Requests
- legacy 语言后续是否继续长期维护，留待下一轮内容治理决策
