# Task-ID: TID-20260411-site-typewriter-redesign

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 将 `apps/site` 首页重构为纯白纸面背景上的极简打字机单页，同时保留现有下载稳定路由结构。
- 让官网展示文案直接复用 App 内嵌提醒文案的气质，而不是再造一套脱离产品的 marketing copy。

## Non-Goals
- 不改 `apps/site/download/**` 的目录结构与跳转协议。
- 不接入真实 GitHub Releases 地址、API 拉取或部署脚本。

## Constraints & Assumptions
- 不新增 npm 依赖或外部字体 CDN。
- 首页仍应保持纯静态实现，能直接由 `python3 -m http.server` 预览。
- 动态打字机展示的文案应来自现有 App 真源，而不是站外来源。

## System Boundaries
- Modules:
  - `apps/site/index.html`
  - `apps/site/styles.css`
  - `apps/site/copy.js`
  - `apps/site/script.js`
  - `apps/site/fonts/LXGWWenKaiScreen.ttf`
  - `apps/site/download/**`（只读复用）
- Ownership:
  - 首页负责品牌气质表达与下载入口
  - `copy.js` 负责站点循环文案
  - 下载子目录继续负责稳定路由
- Dependency direction:
  - `index.html` -> `styles.css` / `copy.js` / `script.js`
  - `script.js` -> `window.PAUZA_SITE_LINES`
  - 首页下载按钮 -> `/download/releases/`

## API / Contract
- Signatures / Endpoints:
  - `/`
  - `/download/releases/`
- Request/Response schema (typed): N/A（纯静态页面）
- Error model (codes, retryability): 首页不依赖远端数据；下载地址未配置时仍由既有下载页 fallback 逻辑兜底

## Data Model / Storage
- `window.PAUZA_SITE_LINES` 作为官网首页循环展示文案的本地真源；不涉及持久化或用户状态写入。

## Invariants
- 首页只保留一个中央展示舞台和一个下载按钮，不回退到多卡片 marketing layout。
- 首页文案优先复用 App 内嵌提醒文案，不应漂移成与产品脱节的营销口号。
- 下载按钮仍然先进入站内稳定路由，而不是直接写死外部下载文件 URL。

## Concurrency / Lifecycle / Memory Model
- 页面生命周期只包含前台打字机循环；无共享并发状态。
- 单个循环严格按“输入 -> 停留约 5 秒 -> 退格 -> 下一条”推进。

## Observability Plan (Debug-Driven)
- Logs: 无下载目标时继续沿用下载页的 `console.warn`
- Metrics: N/A
- Traces: 通过本地浏览器 snapshot / screenshot 做现实检查
- Debug flags: `copy.js` 中的数组内容和 `targets.js` 中的 URL 留空与否即为主要调试入口

## Security & Privacy Considerations
- 无登录、表单、埋点、追踪脚本或本地存储写入。

## Risks & Rollback
- Failure modes:
- 首页过于极简，导致信息过少或字体加载失败
- 打字机动画节奏过快/过慢，影响阅读感受
- 复制字体资源后若路径错误，会导致官网字体回退
- Rollback steps:
  - 回退 `apps/site/index.html`、`styles.css`、`script.js`、`copy.js`、`fonts/`
  - 保留既有 `apps/site/download/**`
  - 重新执行 workflow docs validator

## Acceptance Criteria (System)
- 首页变成纯白纸面背景 + 单一打字机舞台 + 单下载按钮的极简结构。
- 打字机展示的句子来自 App 内嵌提醒文案，按“输入 / 停留 / 退格 / 切换”循环。
- 网站字体优先使用本地 `LXGW WenKai Screen` 资源，不依赖外部字体服务。
- 下载按钮继续走站内稳定路由，docs 与本地验证链可追溯。

## Open Questions / Decision Requests
- 上线前只需补 `targets.js` 的真实 GitHub 地址；本轮先不改变公开路由和下载页结构。
