# Task-ID: TID-20260403-tauri-core-parity

## Goals
- 让 Tauri host 覆盖旧 Electron 当前最关键的一组产品能力，并成为根仓库默认入口。
- 保留 Electron 作为对照与回退边界，而不是继续让它承担默认开发/构建链路。
- 让设置、tray、shortcut、break prompt 全部依附于同一份 `PauzaState` 真源。

## Non-Goals
- 彻底删除 Electron 代码与构建配置。
- 完整重建分发、CI、商店发布链路。
- 引入云同步、健康生态或 Apple-only 能力。

## Constraints & Assumptions
- 仓库仍是双壳并存期，文档必须明确“默认入口已切换”与“Electron 仍保留”两个事实。
- `platform.rs` 只提供轻量跨平台信号探测，不代表最终最强实现。
- 保持前台简洁比继续堆功能展示更重要，高级设置可折叠但不能消失。

## System Boundaries
- Modules:
  - `state.rs`: 设置、调度、snapshot、break state 与 action 真源
  - `engine.rs`: 后台 tick 循环与通知分发
  - `platform.rs`: idle / DND / app exclusion 信号探测
  - `shell.rs`: tray、shortcut、主窗口与 break window 生命周期
  - `commands.rs`: 前台命令面与 snapshot 返回
  - `App.tsx`: 设置页 / break prompt 前台壳
- Ownership:
  - Rust host 决定运行时行为与系统交互
  - React 前台只负责展示 snapshot 和提交命令
- Dependency direction:
  - `App.tsx` -> `commands.rs` -> `state.rs`
  - `engine.rs` -> `platform.rs` + `state.rs` -> `shell.rs`
  - 文案层前后端都依赖 i18n lookup，而不是互相硬编码字符串

## API / Contract
- Signatures / Endpoints:
  - `get_snapshot`
  - `update_settings`
  - `pause_breaks`
  - `resume_breaks`
  - `start_focus_session`
  - `clear_focus_session`
  - `finish_current_break`
  - `skip_current_break`
  - `postpone_current_break`
  - `skip_to_next_scheduled_break`
  - `skip_to_next_microbreak`
  - `skip_to_next_long_break`
  - `reset_breaks`
  - `toggle_autostart`
- Request/Response schema (typed):
  - 前台提交 `PauzaSettings`
  - 宿主统一返回 `DesktopSnapshot`
  - break prompt 消费 `currentBreak` 子结构中的 `manualAwaiting/canPostpone/canSkip/showClock`
- Error model (codes, retryability):
  - 当前为字符串错误模型，无稳定 error code
  - 命令失败直接回传前台展示，通常可重试

## Data Model / Storage
- `PauzaSettings` 已扩展至 parity 所需配置：
  - per-kind notification
  - per-kind postpone + limit
  - per-kind strict mode
  - per-kind manual finish
  - break prompt style
  - all screens / target screen / current time
  - tray strict visibility
  - reveal/focus/pause/skip/reset shortcuts
- `settings.json` 是持久化真源，保存在 app config 目录。
- `DesktopSnapshot` 是前台唯一读取模型，禁止从多个命令拼接前台状态。

## Invariants
- 所有前台显示的运行时状态必须可由单次 `snapshot()` 推导得到。
- `update_settings()` 必须经过 `sanitized()` 再写盘。
- root 默认脚本必须优先指向 Tauri；Electron 只能作为 legacy 入口保留。
- break window 是否可关闭，由当前 break strict/manual 状态与设置共同决定。

## Concurrency / Lifecycle / Memory Model
- `PauzaState` 使用 `Arc<Mutex<RuntimeState>>` 作为进程内共享状态容器。
- `engine.rs` 每秒 tick 一次，读取平台信号并驱动 `state.tick()`。
- tray action、shortcut action、window close action 和前台命令都收敛到 `PauzaState` 方法，避免多处各自维护调度状态。
- break windows 按 monitor 生成，label 以 `break-*` 区分。

## Observability Plan (Debug-Driven)
- Logs:
  - `last_action` 作为最低限度行为日志。
  - `status/statusDetail` 作为用户可见调度状态。
- Metrics:
  - 当前未独立实现。
- Traces:
  - 当前未独立实现。
- Debug flags:
  - 浏览器 preview 作为无 Tauri runtime 的前台调试模式。

## Security & Privacy Considerations
- 所有配置保存在本地 `settings.json`。
- app exclusion 只存命令片段，不采集远端遥测。
- autostart / tray / shortcut 只使用本地宿主能力。

## Risks & Rollback
- Failure modes:
  - 根默认脚本切换后，用户仍误跑 legacy Electron 或旧构建脚本。
  - tray/shortcut/break window 在特定平台行为与 Electron 不完全一致。
  - `platform.rs` 探测精度不足导致 DND 或应用排除判断偏差。
- Rollback steps:
  - 恢复根 `package.json` 默认脚本到 Electron。
  - 回退 `apps/desktop/**` parity 变更。
  - 继续使用 `app/**` 作为运行主入口。

## Acceptance Criteria (System)
- Tauri host 已成为默认入口并能独立完成核心调度闭环。
- React 前台不再依赖展示页结构。
- 所有运行时操作都能通过 `DesktopSnapshot` 与命令层闭合。
- 文档与验证记录足以支撑后续删除/裁剪 Electron 外围资产。

## Open Questions / Decision Requests
- 是否在下一轮删除根 `package.json` 中剩余的 Electron builder 配置，还是保留到 Tauri 分发链完全稳定后再动。
- 是否需要为 `platform.rs` 的 DND / app exclusion 补原生插件级实现。
