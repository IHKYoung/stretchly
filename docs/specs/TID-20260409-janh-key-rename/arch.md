# Task-ID: TID-20260409-janh-key-rename

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 把开发者姓名相关 locale key 从 `preferences.about.janH` 统一重命名为 `preferences.about.clarkeY`。
- 让 desktop locale、archived locale、legacy HTML 引用与生成后的 registry 保持一致。

## Non-Goals
- 不修改 `Clarke Young` 这个显示值本身。
- 不调整 `package.json`、README、metainfo 或 `LICENSE`。
- 不改变任何前台交互或运行时逻辑。

## Constraints & Assumptions
- 当前桌面端和 archived locale 都使用 JSON 资源文件，适合做批量 key 重命名。
- `app/preferences.html` 仍保留旧 `data-i18next` 引用，需要一起改，否则 legacy grep 会残留旧 key。
- 本轮不需要为 key rename 额外引入迁移兼容层。

## System Boundaries
- Modules:
  - `apps/desktop/src/locales/messages/*.json`
  - `app/locales/*.json`
  - `apps/desktop/src/locales/registry.generated.json`
  - `app/preferences.html`
  - `docs/specs/TID-20260409-developer-name-unification/*`
  - `docs/specs/TID-20260409-janh-key-rename/*`
  - `docs/{plans,logs}/2026-04-09.md`
- Ownership:
  - locale key 名与 legacy HTML i18n 引用归本任务统一重命名。
  - 显示值、链接和产品元数据不在本任务修改范围内。
- Dependency direction:
  - locale source -> `registry.generated.json`
  - legacy HTML `data-i18next` -> locale key
  - 文档说明 -> 当前仓库实际 key 名

## API / Contract
- Signatures / Endpoints:
  - N/A
- Request/Response schema (typed):
  - `preferences.about.clarkeY = "Clarke Young"`
- Error model (codes, retryability):
  - 若 locale source 与 registry 不一致，`sync_desktop_locales.py` 后的 grep 审计会直接暴露残留旧 key。

## Data Model / Storage
- 仅重命名已有字段：`janH -> clarkeY`。
- 不新增新字段，也不新增兼容 alias。

## Invariants
- desktop locale、archived locale 与 generated registry 不再出现 `janH`。
- `app/preferences.html` 的 `data-i18next` 与 locale key 保持一致。
- 产品显示值继续是 `Clarke Young`。

## Concurrency / Lifecycle / Memory Model
- N/A。本任务不涉及并发、生命周期或内存模型。

## Observability Plan (Debug-Driven)
- Logs:
  - `rg -n "janH"` 审计残留旧 key
  - `sync_desktop_locales.py` 验证 desktop registry 已同步
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - N/A

## Security & Privacy Considerations
- 无新增权限、密钥、网络调用或敏感数据处理。

## Risks & Rollback
- Failure modes:
  - locale source 已重命名但 registry 未重生成，导致前后端看到的 key 不一致。
  - archived `app/preferences.html` 没有同步，仓库仍残留旧引用。
  - 同日旧 spec 仍描述旧 key，后续维护者会误判当前真相。
- Rollback steps:
  - 回退 locale JSON、`app/preferences.html`、registry 与相关 docs，然后重新跑 sync / validator。

## Acceptance Criteria (System)
- AC1: `apps/desktop/src/locales/messages/*.json`、`app/locales/*.json` 与 `apps/desktop/src/locales/registry.generated.json` 中不再出现 `janH`。
- AC2: `app/preferences.html` 改为使用 `preferences.about.clarkeY`。
- AC3: 同日相关 docs 不再把当前 key 写成 `janH`。
- AC4: `python3 scripts/sync_desktop_locales.py`、`npm test`、`python3 scripts/validate_workflow_docs.py --mode manual` 通过，且 `rg -n "janH"` 返回 0。

## Open Questions / Decision Requests
- 无
