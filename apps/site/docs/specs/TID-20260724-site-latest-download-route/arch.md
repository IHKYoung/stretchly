# Task-ID: TID-20260724-site-latest-download-route

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。

## Goals
- 将 GitHub latest release 恢复为官网最终下载地址的事实源。
- 由 latest API 返回的资产名构造 `releases/latest/download/<asset-name>`，并保留显性的固定版本回退。

## Non-Goals
- 不调整 Release/tag/DMG 命名规范，不修改桌面端源码，不改变官网视觉。
- 不把父仓库完整源码推送到官网 `baseline`。

## Constraints & Assumptions
- Release 继续包含且仅需选择一个名称以 `_aarch64.dmg` 结尾的 macOS 资产。
- GitHub latest API 可能因网络、限流或响应内容异常而失败；失败时必须保留可点击的 pinned URL。
- 资产名包含版本号，因此不能只靠一个永久静态文件名脱离 API 获取未来版本。

## System Boundaries
- Modules: `download/targets.js` 提供 API、latest download base 与 fallback；`script.js` 解析并更新 `<a>`；`index.html` 提供首屏 fallback。
- Ownership: GitHub Release API 拥有最新 release/资产事实；浏览器是观察者和路径构造执行者；下载按钮是展示者。
- Dependency direction: `index.html -> targets.js -> script.js -> GitHub API -> GitHub latest redirect`。

## API / Contract
- Signatures / Endpoints: `GET https://api.github.com/repos/IHKYoung/Pauza/releases/latest`；下载入口 `https://github.com/IHKYoung/Pauza/releases/latest/download/<encoded asset name>`。
- Request/Response schema (typed): `{ assets: Array<{ name?: string; browser_download_url?: string }> }`；只消费 `name`，并筛选 `_aarch64.dmg`。
- Error model (codes, retryability): 非 2xx、fetch reject、非法 JSON 或无匹配资产均在本次页面生命周期内回退 pinned；不伪造 Release 失败，不自动重试。

## Data Model / Storage
- 无持久化数据；单页生命周期内仅缓存一个 `latestDownloadUrlPromise`。

## Invariants
- API 返回匹配资产时，最终 href 必须是 `releases/latest/download/<asset-name>`。
- API/内容失败时，最终 href 必须保持有效的 `v0.1.4` pinned URL。
- 单次请求失败不改变 GitHub Release 状态，也不显示内部队列或运行态信息。

## Concurrency / Lifecycle / Memory Model
- 页面初始化与首次点击共享同一个 promise，避免并发请求和结果互相覆盖；页面刷新后重新解析 latest。

## Observability Plan (Debug-Driven)
- Logs: GitHub API 解析失败继续使用现有 `[Pauza site] Failed to resolve...` console warning。
- Metrics: 无新增指标。
- Traces: 通过 curl/DOM smoke 与 GitHub/Vercel deployment 记录留证。
- Debug flags: 无。

## Security & Privacy Considerations
- 仅请求公开 GitHub API，不使用 token、不收集用户数据；资产名经 `encodeURIComponent` 后拼入路径。

## Risks & Rollback
- Failure modes: GitHub API 限流时回退 pinned；未来资产命名不再满足 suffix 时回退 pinned；错误部署可能令线上仍保持旧 href。
- Rollback steps: revert 本任务代码提交并推送 `baseline`，或将 Vercel Production 回滚到 `bad94ee` 对应部署。

## Acceptance Criteria (System)
- 模拟 newer latest 时最终 href 使用 latest download 路径；API 失败/无资产时使用 pinned；线上部署记录指向新提交且最终 DOM 命中 0.1.4 latest 路径。

## Open Questions / Decision Requests
- 无；用户已明确 latest 路径契约和生产更新授权。
