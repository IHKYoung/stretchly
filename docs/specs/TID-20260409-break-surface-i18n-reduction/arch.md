# Task-ID: TID-20260409-break-surface-i18n-reduction

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 为 break/settings 当前需求补齐最少且稳定的 schema 和宿主边界。
- 把 i18n 从双 bundle 硬编码改成可扩展的注册机制，前后端使用同一语言目录约束。
- 明确 legacy `app/` 对当前仓库仍承担的职责，避免误删。

## Non-Goals
- 不把整个 Electron legacy 运行时立即迁移到 Tauri。
- 不引入新的第三方 i18n 依赖或构建时服务。

## Constraints & Assumptions
- 不能新增依赖。
- 前端与 Rust host 都需要消费同一批 locale 资源或同源注册结果。
- 结束音效优先复用现有前端音频播放链路，除非验证证明不可靠。

## System Boundaries
- Modules:
-  `apps/desktop/src/App.tsx`: break/settings 前台与浏览器 preview。
-  `apps/desktop/src/i18n.ts`: 前端 locale lookup、bundle registry 与 fallback。
-  `apps/desktop/src-tauri/src/i18n.rs`: Rust host locale lookup 与 fallback。
-  `apps/desktop/src-tauri/src/state.rs`: settings 真源、snapshot、开始/结束音配置持久化。
-  `apps/desktop/src-tauri/src/shell.rs`: break window profile 与 window-mode 几何。
-  `app/locales/*.json` / `app/preferences.html`: legacy 语言清单与历史翻译来源审查输入。
- Ownership:
  - Tauri 前台 owns 桌面端设置页与 break 页面文案显示。
  - Rust host owns settings 存储、snapshot 结构和 break window geometry。
  - legacy `app/` 当前仅作为审查输入和兼容链路候选，不应新增桌面端能力。
- Dependency direction:
  - locale registry -> 前端 lookup / Rust lookup
  - settings schema -> App.tsx form / snapshot
  - shell geometry -> break window 展示

## API / Contract
- Signatures / Endpoints:
  - `bootstrap`, `get_snapshot`, `update_settings` 保持不变。
- Request/Response schema (typed):
  - `PauzaSettings` 新增 `microbreak_end_sound` 与 `long_break_end_sound`。
  - 语言注册输出需要提供：`code`、`label`、`nativeLabel`、`desktopReady`、`fallback`。
- Error model (codes, retryability):
  - locale 缺失时回退到默认语言并记录清晰错误文本；不新增复杂 code。

## Data Model / Storage
- `settings.json` 将持久化四个 break 音效字段：微休息开始/结束、休息开始/结束。
- locale 结构调整为：
  - 消息文件：每语言一份
  - 配置文件：每语言一份
  - 注册层：自动汇总可用语言与 fallback 信息

## Invariants
- break prompt 仍是纯净单列结构。
- `normalize_language` 不能再把支持语言集合写死成 `zh-CN/en`。
- 未完整覆盖的语言不应直接导致设置页出现 key、占位符或多语言混杂。

## Concurrency / Lifecycle / Memory Model
- break 开始音与结束音都必须按 break 实例键值只播放一次，避免 React 重渲染重复触发。
- break 结束音的判定需与 `startedAtMs` / `manualAwaiting` 或结束时刻绑定，避免跨 break 串音。

## Observability Plan (Debug-Driven)
- Logs:
  - locale 注册缺失或读取失败时输出明确 fallback 信息。
  - 不新增 runtime metrics。
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:

## Security & Privacy Considerations
- 自定义壁纸继续以压缩 data URL 存在本地设置，不引入外部上传。
- 语言配置和消息文件为静态本地资源，不含外部拉取。

## Risks & Rollback
- Failure modes:
  - locale registry 前后端不同步，导致 host/前台语言名或 fallback 不一致。
  - 结束音播放时序与 break 关闭逻辑冲突。
  - legacy 目录审查结论误判，导致删掉仍被根级脚本引用的资源。
- Rollback steps:
  - 回退 i18n registry、新增 settings 字段、break 结束音逻辑和 shell ratio 改动。
  - 删除动作本轮只做审查，不先执行。

## Acceptance Criteria (System)
- `PauzaSettings` schema、前端 form 与音频播放逻辑对齐。
- 前后端 i18n 都通过同一注册结果解析语言，而非手写双语言分支。
- legacy `app/` 的删除边界有可追溯结论。

## Open Questions / Decision Requests
- 若后续要真正开放所有 legacy 语言到新桌面端，需要先补齐桌面端新增文案翻译；本轮不强行用英文 fallback 伪装“已支持”。
