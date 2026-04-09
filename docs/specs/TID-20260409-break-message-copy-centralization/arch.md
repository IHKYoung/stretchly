# Task-ID: TID-20260409-break-message-copy-centralization

## Goals
- 切断桌面端 locale registry 对 legacy `app/locales` 与 `app/preferences.html` 的构建依赖。
- 为 break 消息页建立一个明确、可编辑且与共享 locale registry 解耦的文案入口。

## Non-Goals
- 不改 Rust host 的 locale consumer 合同。
- 不迁移或删除 legacy Electron 自己的 locale 资源。

## Constraints & Assumptions
- `registry.generated.json` 的结构保持不变，前端 `i18n.ts` 与 Rust `i18n.rs` 无需改消费协议。
- 当前桌面端可用语言集合继续以 `apps/desktop/src/locales/config/*.json` 为准。
- break 消息页的专属文案目前只需覆盖 `zh-CN` 与 `en` 两种 desktop-ready 语言。

## System Boundaries
- Modules:
  - `scripts/sync_desktop_locales.py`
  - `apps/desktop/src/locales/{messages,config,overrides,registry.generated.json}`
  - `apps/desktop/src/locales/break-message-copy.ts`
  - `apps/desktop/src/{App.tsx,lib/break-prompt.ts}`
- Ownership: 桌面端 locale 资源与 break 消息页文案均归 `apps/desktop/src/locales/**` 管理；legacy `app/` 仅保留给旧壳。
- Dependency direction: desktop runtime/readers -> `registry.generated.json`；registry generator -> desktop locale source files；break UI -> `break-message-copy.ts`。

## API / Contract
- Signatures / Endpoints:
  - `getBreakMessageCopy(language)`
  - `getBreakPrompts(language, kind)`
  - `python3 scripts/sync_desktop_locales.py`
- Request/Response schema (typed): `registry.generated.json` 继续输出 `{ defaultLanguage, languages, bundles }`；`break-message-copy.ts` 输出按语言分组的 break 页面专属文案对象。
- Error model (codes, retryability): locale generator 若发现 config / message / override 不匹配则直接退出并报错，阻止继续生成脏 registry。

## Data Model / Storage
- 桌面端通用 locale 真源：`apps/desktop/src/locales/messages/*.json`、`config/*.json`、`overrides/*.json`
- break 页面专属文案真源：`apps/desktop/src/locales/break-message-copy.ts`
- 共享产物：`apps/desktop/src/locales/registry.generated.json`

## Invariants
- 桌面端 registry 构建不再读取 `app/locales` 或 `app/preferences.html`。
- break 页面文案不再依赖 `ui.break.*` locale override key。
- `normalizeLanguage()` 仍是 break 页面语言回退的唯一入口。

## Concurrency / Lifecycle / Memory Model
- 不涉及并发共享状态；仅有构建期文件生成与前端静态读取。

## Observability Plan (Debug-Driven)
- Logs: `scripts/sync_desktop_locales.py` 现在会在 config / message / override 不匹配时直接报出 language code。
- Metrics: 不涉及。
- Traces: 不涉及。
- Debug flags: 不涉及。

## Security & Privacy Considerations
- 不引入网络请求、不新增权限、不处理额外用户数据。

## Risks & Rollback
- Failure modes:
  - locale 目录存在孤儿文件，导致 registry 生成中断
  - break 页面仍残留旧 `ui.break.*` 读取，导致部分文案缺失
- Rollback steps:
  - 回退 `scripts/sync_desktop_locales.py`、`apps/desktop/src/locales/break-message-copy.ts`、`App.tsx`、`break-prompt.ts` 与相关 docs
  - 重新运行 locale 构建与前端 build 确认恢复

## Acceptance Criteria (System)
- locale generator 仅依赖桌面端 `locales/**` 目录即可生成 registry。
- 前端与 Rust 继续消费同一份 `registry.generated.json`，无需改协议。
- break 页面主文案能从单文件读取并完成语言回退。

## Open Questions / Decision Requests
- 无。
