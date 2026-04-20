# Task-ID: TID-20260412-site-release-0-1-2

## Goals
- 让官网下载按钮稳定指向 `Pauza_0.1.2_aarch64.dmg`。
- 避免 GitHub latest release API 仍返回旧资产时把页面按钮回退到过期版本。

## Non-Goals
- 不负责创建 GitHub `v0.1.2` release、上传资产或调整 release 命名规范。
- 不改站点视觉设计、文案、字体、布局和其它交互逻辑。

## Constraints & Assumptions
- 站点是纯静态 HTML/CSS/JS，无构建步骤、无服务端存储、无运行态后端。
- 当前 release 资产命名延续 `Pauza_<version>_aarch64.dmg`。
- `download/targets.js` 中的 pinned URL 代表当前应发布的目标版本。

## System Boundaries
- Modules: `index.html` 固定 href、`download/targets.js` 下载配置、`script.js` 运行时解析逻辑。
- Ownership: 下载目标由站点仓库维护；真正的 release 资产托管于 GitHub Releases。
- Dependency direction: 页面先读取本地 pinned 配置，再尝试请求 GitHub latest API；API 仅在返回同名目标资产时接管。

## API / Contract
- Signatures / Endpoints: `window.PAUZA_DOWNLOAD_TARGETS.macosAppleSilicon` 包含 `fallbackUrl`、`latestReleaseApi`、`assetNameSuffix`；`resolveLatestDownloadUrl()` 返回最终下载 URL。
- Request/Response schema (typed): `latestReleaseApi` 请求 GitHub release JSON，读取 `assets[].name` 与 `assets[].browser_download_url`。
- Error model (codes, retryability): 网络失败或 API 返回非 2xx 时沿用 `console.warn` 并回退到 pinned URL；无页面级报错 UI。

## Data Model / Storage
- 无持久化数据；下载目标仅由静态配置与运行时 fetch 结果组成。

## Invariants
- 页面初始化后，下载按钮必须至少可用且指向 pinned URL。
- 若 API 解析出的资产名与 pinned 资产名不一致，不允许自动回退到旧 release。
- 若 API 不可用，按钮继续使用 pinned URL。

## Concurrency / Lifecycle / Memory Model
- 通过单例 `latestDownloadUrlPromise` 复用一次请求结果，避免重复点击时并发发起多次同源 fetch。
- 新增 basename 比对逻辑不改变现有 promise 生命周期，仅影响返回 URL 的决策。

## Observability Plan (Debug-Driven)
- Logs: 保留现有 `console.warn('[Pauza site] Failed to resolve latest GitHub release asset.', error)`。
- Metrics: 不新增。
- Traces: 不新增。
- Debug flags: 不新增。

## Security & Privacy Considerations
- 仅请求公开 GitHub Releases API，不引入新权限或用户数据采集。
- 下载按钮继续指向 GitHub release 资产，不写入本地敏感信息。

## Risks & Rollback
- Failure modes: pinned URL 写错导致 404；GitHub latest API 返回结构变化；版本名比较逻辑误判。
- Rollback steps: 回退 `index.html`、`download/targets.js`、`script.js` 到前一个 commit，恢复 `v0.1.1` 行为。

## Acceptance Criteria (System)
- 首页下载按钮的固定链接与 pinned fallback 都更新到 `Pauza_0.1.2_aarch64.dmg`。
- 当 GitHub latest API 仍返回 `Pauza_0.1.1_aarch64.dmg` 时，页面最终仍保留 `0.1.2` 的下载链接。
- 手动点击下载按钮时，跳转目标与页面最终 href 一致。

## Open Questions / Decision Requests
- 无；本次按现有 GitHub Releases 路径与命名规则执行。
