# Task-ID: TID-20260411-site-landing-page

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 为 Pauza 增加一个独立的静态官网应用目录，不干扰现有桌面端实现和构建链。
- 将下载按钮与真实文件地址之间增加稳定跳转层，避免未来切换下载源时影响公开路由。

## Non-Goals
- 不接入 SSR、CMS、博客系统或额外前端框架。
- 不在本轮接入真实 GitHub Releases API、自动版本探测或 Vercel 部署脚本。

## Constraints & Assumptions
- 不新增 npm 依赖；官网应基于原生 HTML/CSS/JS 落地。
- 网站目录需与 `apps/desktop` 明确隔离，避免影响桌面端默认运行/构建链。
- 下载目标地址当前允许先通过单一配置文件维护；若未配置真实 URL，跳转页应给出明确 fallback 提示。

## System Boundaries
- Modules:
  - `apps/site/index.html`
  - `apps/site/styles.css`
  - `apps/site/script.js`
  - `apps/site/download/**`
- Ownership:
  - 官网首页负责品牌表达、能力概览与下载入口
  - 下载子目录负责稳定路由与真实下载地址解耦
- Dependency direction:
  - landing page -> local static assets / script
  - download pages -> shared targets config
  - root scripts / docs 只提供开发入口和索引说明

## API / Contract
- Signatures / Endpoints:
  - `/`
  - `/download/macos-apple-silicon/`
  - `/download/macos-intel/`
  - `/download/releases/`
- Request/Response schema (typed): N/A（纯静态页面）
- Error model (codes, retryability): 当目标 URL 未配置时，下载跳转页必须保持可见 fallback 状态，而不是空白页或死链

## Data Model / Storage
- `apps/site/download/targets.js` 作为站点侧单一下载地址真源，不涉及持久化和运行时写入。

## Invariants
- landing page 始终只有一个主页面，不演化为多级滚动信息架构。
- 下载按钮永远指向站内稳定路由，而不是在首页直接散落第三方资产地址。
- 官网 copy 必须优先表达“久坐提醒 / 更安静的 break companion / 智能等待空档”。

## Concurrency / Lifecycle / Memory Model
- 无后台状态和共享并发模型。
- 页面生命周期只包含首屏入场动画、轻量 hover / pointer 交互和下载跳转定时器。

## Observability Plan (Debug-Driven)
- Logs: 下载跳转页在未配置 URL 时打印 `console.warn`
- Metrics: N/A
- Traces: 通过 Playwright 本地现实检查与截图取证
- Debug flags: `targets.js` 中通过是否留空 URL 控制 redirect / fallback

## Security & Privacy Considerations
- 无登录、表单、追踪脚本或用户数据采集。
- 下载跳转页只做前端跳转，不写入本地存储，不携带敏感参数。

## Risks & Rollback
- Failure modes:
  - 页面过度追求视觉效果而牺牲信息密度
  - 下载路由配置缺失导致按钮落到无目标状态
  - 新增站点目录后 docs / scripts 没同步，后续维护者找不到入口
- Rollback steps:
  - 删除 `apps/site` 与根脚本接线
  - 回退 `README.md`、`docs/RepositoryGuidelines.md`、`docs/CodeMap.md`、`docs/CHANGELOG.md`
  - 重新跑 workflow docs validator

## Acceptance Criteria (System)
- 仓库新增一个独立的 `apps/site` 静态官网目录，可本地直接预览。
- 首页能在单页内承载产品定位、能力摘要与下载按钮。
- 下载按钮统一走 `apps/site/download/**` 路由，并由独立配置文件管理真实目标地址。
- docs 与本地验证链能追溯网站入口、下载配置和现实检查证据。

## Open Questions / Decision Requests
- 真实下载源最终落 GitHub Releases 还是对象存储，本轮先不锁死到代码之外的部署决策。
