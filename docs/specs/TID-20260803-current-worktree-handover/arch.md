# Task-ID: TID-20260803-current-worktree-handover

## Goals
- 固化智能提醒“已到点、等空档”阶段的单一事实源与 tray 展示契约。
- 在不扩展设置 schema 的前提下收口节奏输入、三语言和视觉层级。
- 将已完成的官网 latest 下载能力作为父仓库快照纳入可审计提交。

## Non-Goals
- 不改变 smart/forced 调度阈值、break 生命周期、autosave 或下载发布流程。
- 不新增提示语、依赖、权限、网络服务或运行时持久化字段。
- 不提交本地翻译生成中间物，也不收口嵌套 site 仓库的独立残留历史。

## Constraints & Assumptions
- `RuntimeState` 是 break due/wait/blocking 的事实源；前端和 tray 不得从本地时间猜测等待进度。
- `DesktopSnapshot` 使用 serde camelCase 暴露给前台；新增 `Option<u64>` 在非等待态为 `null`，兼容既有消费者。
- 官网 pinned URL 必须始终可点击；latest API 只负责提供资产名，不能让失败路径变成空链接。
- 父仓库会同时 staged 根 task package 与 `apps/site/docs/specs/TID-20260724-site-latest-download-route/**`，commit 必须按多 Task-ID 处理。

## System Boundaries
- Modules:
  - Runtime/state: `apps/desktop/src-tauri/src/state.rs`、`state/tests.rs`。
  - Tray observer/presenter: `apps/desktop/src-tauri/src/shell.rs`。
  - Settings presenter/editor: `apps/desktop/src/App.tsx`、`styles.css`。
  - Locale source/generated data: `apps/desktop/src/locales/messages/{en,zh-CN,zh-TW}.json`、`registry.generated.json`。
  - Site resolver: `apps/site/download/targets.js`、`apps/site/script.js`。
  - Governance: root/site docs、tests、Git hooks 与 audit scripts。
- Ownership:
  - 调度事实、等待起点、blocker、最大等待：Rust `RuntimeState`。
  - 派生剩余等待时间：`PauzaState::snapshot()`。
  - tray title/刷新 bucket：Tauri shell。
  - 设置草稿与展示：React；持久化事实仍为 Rust settings。
  - latest release/asset：GitHub Release API；浏览器只解析并呈现 href。
- Dependency direction:
  - `RuntimeState -> DesktopSnapshot -> shell/App`。
  - `messages/*.json -> sync_desktop_locales.py -> registry.generated.json -> App`。
  - `index.html pinned -> targets.js -> script.js -> GitHub API/latest redirect`。

## API / Contract
- Signatures / Endpoints:
  - `DesktopSnapshot.next_break_wait_remaining_ms: Option<u64>` / `nextBreakWaitRemainingMs: number | null`。
  - `RuntimeState::waiting_for_opportunity_kind(now)` 与 `current_smart_wait_remaining_ms(kind, now)` 提供 backend wait 事实。
  - Site `GET https://api.github.com/repos/IHKYoung/Pauza/releases/latest`，下载路径 `releases/latest/download/<encoded name>`。
- Request/Response schema (typed):
  - Desktop snapshot 字段只在 no blocker + waiting kind + valid wait budget 时存在。
  - Site 只读取 `{ assets: Array<{ name?: string }> }` 并匹配 `_aarch64.dmg`。
- Error model (codes, retryability):
  - Desktop blocker/非等待态返回 `None`，不合成失败状态；单次 snapshot 只反映当前 runtime。
  - Site 非 2xx、reject、非法/空 assets 均在当前页面生命周期回退 pinned，不重试、不伪造 release 失败。

## Data Model / Storage
- 新字段仅属于瞬时 snapshot，不写入 `settings.json`，不需要迁移。
- 设置字段和值域不变，只调整输入展示、单位语义与 locale key。
- Site 下载解析不持久化；共享单个页面内 promise。

## Invariants
- 前端不得把 `next_break_in_ms` 当作 due 后 smart wait 进度。
- blocker 存在时不暴露 smart wait countdown；forced mode 不进入 waiting。
- waiting tray title 优先于 `show_time_to_break_in_tray=false`，因为它是临时操作反馈而非常规计划显示。
- preset 值命中时 custom 输入显示入口而不重复数字；空 draft/escape 不改当前值。
- 三个主语言 key 与 generated registry 同步；generated 文件不得手改漂移。
- 官网任何解析失败都保留 pinned href。

## Concurrency / Lifecycle / Memory Model
- `PauzaState::snapshot()` 在 runtime mutex 内读取 waiting/blocker 并派生剩余预算，避免观察跨 tick 的拼接状态。
- tray detail mode 使用分钟 bucket 控制结构刷新，title 可按 tick 更新秒级文本；不新增 timer owner。
- React custom draft 仍由组件本地 state 持有，blur/Enter 提交、Escape/空值恢复；autosave owner 不变。
- Site 初始化和点击共享 `latestDownloadUrlPromise`，避免并发 fetch 竞争。

## Observability Plan (Debug-Driven)
- Logs: Rust/Vitest assertions、site console warning、generator/build/workflow/hook 输出。
- Metrics: 无运行时指标；验证记录测试数量、bundle 数与 staged Task-ID。
- Traces: source contract tests + build；既有 site 任务保留 production deployment/302 evidence。
- Debug flags: 无新增。

## Security & Privacy Considerations
- Desktop 新字段只包含毫秒预算，不包含用户内容或行为明细。
- Site 只请求公开 GitHub API，不使用 token、不新增数据采集；资产名经过 URL 编码。

## Risks & Rollback
- Failure modes:
  - snapshot 在 blocker/非等待态泄漏旧 wait 值。
  - tray 尊重常规倒计时开关而隐藏必要 waiting feedback，或反过来污染普通状态。
  - locale 真源和 registry 不一致，导致运行时缺 key。
  - custom placeholder 交互导致 preset 值被意外提交为空。
  - 多 Task-ID message 与 staged spec 集不一致，被 hook 拒绝。
- Rollback steps:
  1. 普通 revert 父仓库主提交；如需同时移除审计记录，再 revert audit-only follow-up。
  2. 重跑 locale generator 和相关测试确认回到上一状态。
  3. 不触碰嵌套 site repo 的独立历史或远端部署。

## Acceptance Criteria (System)
- Rust snapshot/tray tests证明 backend-owned wait countdown、blocker/普通 toggle 边界成立。
- 前端/locale tests证明 custom affordance、提醒周期语义、非“立即开始”文案和字体层级成立。
- Site source/docs证明 latest success + pinned fallback 契约保持。
- 生成器、tests、typecheck/build、workflow hooks 与 staged multi-task identity 全部通过或缺口显性记录。

## Open Questions / Decision Requests
- 无；父仓库本地 commit 已获用户明确授权，push/release 和嵌套 repo 残留另行处理。
