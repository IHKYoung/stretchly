# Task-ID: TID-20260403-tauri-i18n-resource-layer

## Goals
- 为 Tauri 前后端建立共享的 locale 资源层，避免在 React 与 Rust 文件中分散维护文本。
- 让 Rust host 可以直接消费 `apps/desktop/src/locales/*.json`，保证 tray、break prompt 与运行时状态文案和前端一致。

## Non-Goals
- 不引入新的第三方 i18n 依赖。
- 不扩展到 `zh-CN / en` 之外的语言范围。
- 不调整调度逻辑、窗口策略或 UI 结构。

## Constraints & Assumptions
- 必须兼容当前 `PauzaSettings.language` 持久化。
- 旧 Electron 的 i18next 方案继续保留在 `app/**`，本轮只处理 Tauri 端。

## System Boundaries
- Modules:
  - `apps/desktop/src/locales/*.json`
  - `apps/desktop/src/i18n.ts`
  - `apps/desktop/src-tauri/src/i18n.rs`
  - `apps/desktop/src-tauri/src/{state,shell,commands}.rs`
- Ownership: 前端通过 key lookup 使用 locale；宿主层通过 Rust lookup 使用同源 locale。
- Dependency direction: UI / Rust host -> i18n layer -> locale JSON。

## API / Contract
- Signatures / Endpoints:
  - 前端 `t(language, key, vars)` / `formatDuration(ms, language)`
  - Rust `i18n::text*` / `i18n::duration`
- Request/Response schema (typed): locale key 为点路径字符串，插值变量统一通过命名占位符 `{{name}}`。
- Error model (codes, retryability): key 缺失时回退到 key 本身，避免运行时崩溃。

## Data Model / Storage
- `PauzaSettings.language` 持久化为字符串，当前允许 `zh-CN` 或 `en`，并在 sanitize 阶段归一化。

## Invariants
- 默认语言必须是中文。
- 前后端不可各自维护重复的文案源。
- 设置页、tray、break prompt 与状态文案必须共享同一批 key。

## Concurrency / Lifecycle / Memory Model
- Rust locale bundle 通过 `OnceLock` 惰性初始化，避免重复解析 JSON。
- locale 重构不改变 engine/tick 线程模型。

## Observability Plan (Debug-Driven)
- Logs: 延用现有运行时状态文案；key 缺失直接回落到 key，便于定位。
- Metrics: 无新增。
- Traces: 无。
- Debug flags: 无。

## Security & Privacy Considerations
- locale 仅包含静态文本与占位符，不涉及用户敏感数据落盘。

## Risks & Rollback
- Failure modes:
  - key 漏配导致界面显示 key 本身。
  - 前后端变量名不一致导致插值残缺。
- Rollback steps:
  - 回退 `src/locales/*.json`、`src/i18n.ts`、`src-tauri/src/i18n.rs` 及调用点。

## Acceptance Criteria (System)
- 前后端存在共享 locale 资源，不再在调用点内维护大段翻译文本。
- 默认语言为 `zh-CN`。
- Rust host 的 tray / break / status 文案全部通过 i18n 层输出。

## Open Questions / Decision Requests
- 暂无；未来若扩展更多语言，可再决定是否引入正式 i18n 依赖。
