# Task-ID: TID-20260803-site-download-015

## Goals
- 保持 GitHub latest release 为下载事实源，并将 fixed fallback 同步到 `v0.1.5`。

## Non-Goals
- 不更改站点结构、视觉或发布架构。

## Constraints & Assumptions
- `v0.1.5` Release 必须先存在且通过远端 hash 回验；站点是无构建步骤的静态站点。

## System Boundaries
- Modules: `index.html`、`download/targets.js`、`script.js`、GitHub Release API/asset、Vercel。
- Ownership: GitHub 持有 release/asset 事实；站点持有跳转与回退配置；Vercel呈现已提交静态文件。
- Dependency direction: GitHub Release -> site resolver/fallback -> browser download。

## API / Contract
- Endpoints: GitHub latest releases API、latest download URL、tag-pinned asset URL。
- Schema: latest `assets[]` 选择 `_aarch64.dmg`；fixed URL 必须为 `v0.1.5/Pauza_0.1.5_aarch64.dmg`。
- Error model: latest 失败可回退；fixed 失败显式报错，不下载旧版本。

## Data Model / Storage
- 静态配置只保存 API/base/fallback/suffix，不保存凭据或动态状态。

## Invariants
- `index.html` href 与 targets fallback 相同；primary/fallback 最终版本一致；无 v0.1.4 残留。

## Concurrency / Lifecycle / Memory Model
- Release 验证后再 push site；push 前确认远端可快进；latest 缓存通过 direct/fixed/production 多点检查。

## Observability Plan (Debug-Driven)
- Logs: git/validator/curl/Vercel 输出。
- Metrics: HTTP status、release tag、asset name/size。
- Traces: site commit -> deployment -> GitHub asset URL。
- Debug flags: 不需要。

## Security & Privacy Considerations
- 无新凭据或第三方服务；不把 GitHub token 写入文件或输出。

## Risks & Rollback
- Failure modes: Release 缺失、URL 漂移、部署失败、缓存或无关 dirty 文件混入。
- Rollback steps: 停止 push，或通过普通 revert 恢复 v0.1.4 fixed URL。

## Acceptance Criteria (System)
- 两条下载路径均到达已验证 v0.1.5 DMG，且发布提交边界干净。

## Open Questions / Decision Requests
- 无。
