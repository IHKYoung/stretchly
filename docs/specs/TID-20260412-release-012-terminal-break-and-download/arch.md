# Task-ID: TID-20260412-release-012-terminal-break-and-download

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 在不引入新依赖和新路由壳的前提下，复用现有桌面端 break 状态机实现终端式 typewriter 输出。
- 让官网的下载入口回到首页直链，同时尽量自动跟随 GitHub 最新 release 资产。

## Non-Goals
- 不新增后端服务、缓存层或自建下载代理。
- 不改 break 调度器、倒计时状态机和 Tauri 宿主窗口生命周期。

## Constraints & Assumptions
- 当前 session 只能单 agent 执行；不使用多 agent warmup。
- GitHub `latest release` API 可能受匿名限流影响，所以官网必须保留固定稳定链接回退。
- 截至 `2026-04-12` GitHub 最新已发布 release 仍是 `v0.1.1`，因此 pinned fallback 继续指向已存在的 `aarch64.dmg`。

## System Boundaries
- Modules:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/lib/break-ideas.ts`
  - `apps/desktop/src/styles.css`
  - `apps/site/{index.html,script.js,download/targets.js,README.md}`
  - 版本元数据：`package.json`、`apps/desktop/package.json`、`apps/desktop/package-lock.json`、`apps/desktop/src-tauri/{Cargo.toml,Cargo.lock,tauri.conf.json}`
- Ownership:
  - break prompt 的渲染与轮播逻辑由前端 React 负责。
  - 官网下载目标选择由静态前端脚本负责。
- Dependency direction:
  - desktop prompt 继续依赖已有 break ideas 真源，不新增第二套文案源。
  - site 首页依赖 `targets.js` 中的稳定目标配置，不再依赖 `/download/*` 中转页。

## API / Contract
- Signatures / Endpoints:
  - `rotateBreakPromptEntries(entries, breakType, startedAtMs)`：为当前 break 生成稳定 prompt 顺序。
  - `GET https://api.github.com/repos/IHKYoung/Pauza/releases/latest`：官网点击下载时读取最新 release 资产。
- Request/Response schema (typed):
  - 只读取 release `assets[].name` 与 `assets[].browser_download_url`，匹配 `_aarch64.dmg` 结尾资产。
- Error model (codes, retryability):
  - GitHub API 失败、超时或未匹配到资产时，不报页面错误，直接回退到 pinned `fallbackUrl`。

## Data Model / Storage
- 不新增持久化结构。
- 版本号真源统一更新到 `0.1.2`，但官网 pinned 下载链接保持已发布资产版本。

## Invariants
- 微休息每次 break 只展示一条稳定 prompt。
- 长休息必须在整句打完后至少停留 `30s` 才允许切换下一条。
- 官网下载路径对用户只能表现为“点击按钮即下载/跳转”，不能重新引入目录页或中转页。

## Concurrency / Lifecycle / Memory Model
- typewriter 通过 `startedAtMs` 派生稳定顺序，避免每轮 snapshot/poll 都重新抽签。
- `prefers-reduced-motion` 和 `manualAwaiting` 直接绕过动画，避免不必要的定时器轮转。
- 官网下载解析只在按钮点击时发起一次请求，失败即回退，不长期持有状态。

## Observability Plan (Debug-Driven)
- Logs: 本轮只保留 docs/logs/plans 与手工验证记录，不新增运行时结构化日志。
- Metrics: N/A
- Traces: N/A
- Debug flags: break preview 仍可通过 `?window=break` 人工验证 prompt 展示。

## Security & Privacy Considerations
- 不新增权限、密钥或第三方服务。
- 官网仅请求 GitHub 官方 release API，不上传用户数据。

## Risks & Rollback
- Failure modes:
  - GitHub API 限流或资产命名变化会让官网落回 pinned `v0.1.1` 链接。
  - typewriter 节奏若后续被认为仍偏快，可继续微调字符间隔与 hold duration。
- Rollback steps:
  - 回退 `apps/desktop` 的 prompt 逻辑和样式即可恢复静态 break 文案。
  - 回退 `apps/site` 首页脚本和 `targets.js` 即可恢复旧下载方式。
  - 若版本 bump 需要撤回，以本次 commit 为边界执行后续 revert。

## Acceptance Criteria (System)
- break prompt 逻辑对微休息/长休息有稳定且可预测的不同轮播策略。
- 官网下载入口只依赖首页与 `targets.js`，点击时能够优先使用 GitHub 最新 release 资产并保留固定回退。
- 根、desktop、Tauri 与 Cargo manifest 的版本号统一到 `0.1.2`。

## Open Questions / Decision Requests
- 无；回退链接是否改到 `0.1.2` 取决于 GitHub release 资产何时真正发布，不在本次本地 commit 内强行猜测。
