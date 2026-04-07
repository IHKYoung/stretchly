# Task-ID: TID-20260403-tauri-migration-foundation

## Goals
- 以 Tauri 2 取代 Electron 作为未来跨平台桌面壳的主方向。
- 在 Rust host 中先建立可演进的宿主层边界，而不是继续把所有桌面能力塞进单个 JS 主进程。

## Non-Goals
- 本轮不迁移完整久坐调度引擎。
- 本轮不定义正式持久化模型和跨设备同步。
- 本轮不移除旧 Electron 主体。

## Constraints & Assumptions
- 必须保留旧 `app/**` 作为迁移阶段的 source basis。
- 必须兼顾跨平台，而不是走只适配 macOS 的原生路线。
- Tauri 官方插件覆盖 tray/shortcut/notification/autostart，但 idle / DND / app exclusion 仍需后续自研宿主服务。

## System Boundaries
- Modules:
  - `src-tauri/src/lib.rs`：组装应用状态、插件和命令。
  - `src-tauri/src/shell.rs`：tray 菜单、托盘点击和全局快捷键。
  - `src-tauri/src/commands.rs`：前端 invoke 可调用的命令边界。
  - `src-tauri/src/state.rs`：运行时状态快照与迁移队列元数据。
  - `src/App.tsx` + `src/styles.css`：新的桌面 dashboard。
- Ownership:
  - Rust host 拥有系统入口、宿主能力与状态真源。
  - React dashboard 只拥有展示与轻交互，不直接控制系统能力实现。
- Dependency direction:
  - UI -> invoke commands -> Rust host -> runtime state
  - 旧 `app/**` 只作为迁移参考，不被新壳直接 import。

## API / Contract
- Signatures / Endpoints:
  - `bootstrap() -> DesktopSnapshot`
  - `show_main_window() -> Result<(), String>`
  - `hide_main_window() -> Result<(), String>`
  - `start_focus_session(minutes: u64) -> Result<DesktopSnapshot, String>`
  - `clear_focus_session() -> Result<DesktopSnapshot, String>`
  - `toggle_autostart() -> Result<DesktopSnapshot, String>`
- Request/Response schema (typed):
  - `DesktopSnapshot` 返回 product/runtime/platform/appVersion/autostartEnabled/focusUntilMs/focusSource/lastAction，以及 `shellCapabilities`、`migrationTracks`、`legacySources` 三组序列化数据。
- Error model (codes, retryability):
  - 当前使用字符串错误直接上传到前端；大多为可重试的宿主动作失败，如窗口不存在或 autostart 操作失败。

## Data Model / Storage
- 当前仅有内存态 `PauzaState`，通过 `Mutex<RuntimeState>` 保存 focus session 和最近一次动作。
- 本轮不引入磁盘持久化；设置与历史记录仍保留在旧 Electron 端，等待下一阶段重建。

## Invariants
- `main` 窗口 label 固定存在于 Tauri 配置中。
- `DesktopSnapshot` 必须始终能被前端渲染，即使在 preview fallback 下也要有可展示的默认值。
- tray、shortcut 与 dashboard 三个入口对 focus session 的行为语义必须一致。

## Concurrency / Lifecycle / Memory Model
- `PauzaState` 用 `Mutex` 保护运行时状态，当前仅处理轻量命令竞争。
- focus session 到期时由前端刷新快照时清理过期状态；后续迁移 scheduler 时会下沉到 Rust 定时服务。
- tray、shortcut 和前端 invoke 都共享同一 `PauzaState`。

## Observability Plan (Debug-Driven)
- Logs:
  - `last_action` 作为最小可视状态日志，帮助快速判断最近发生了什么。
  - Tauri dev / cargo stdout 用于当前阶段的宿主层诊断。
- Metrics: 当前无正式 metrics；后续迁移 planner 时引入 break completion / postpone / streak 等指标。
- Traces: N/A
- Debug flags: N/A

## Security & Privacy Considerations
- 本轮未接入外部服务、账户或同步。
- 全局快捷键与 autostart 属于宿主权限能力，但未新增用户数据采集。
- 浏览器 preview 不暴露原生宿主能力，只做 UI 兜底。

## Risks & Rollback
- Failure modes:
  - Tauri runtime 初始化失败
  - tray 或 shortcut 注册失败
  - autostart 插件在不同平台上的行为差异
- Rollback steps:
  - 删除 `apps/desktop/**`
  - 回退根命令入口与相关文档
  - 保持 Electron 为唯一运行时

## Acceptance Criteria (System)
- `cargo check` 通过，证明 Rust host 与插件装配有效。
- `npm run typecheck`、`npm run build` 通过，证明 dashboard 构建有效。
- `npm run tauri:dev` 能启动到 `target/debug/pauza-desktop`。

## Open Questions / Decision Requests
- 下一阶段是否优先迁移 planner，还是先做 settings / today dashboard。
- idle / DND / app exclusions 的宿主信号层是集中在单个 Rust service，还是按能力拆多个 adapter。
