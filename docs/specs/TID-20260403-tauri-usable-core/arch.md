# Task-ID: TID-20260403-tauri-usable-core

## Goals
- 在 Tauri host 中形成一个可运行的最小久坐干预闭环，替代“只有壳”的迁移阶段。
- 保持 Electron 旧实现作为行为参考，但不再让其决定新产品壳的交互形态。

## Non-Goals
- 不追求与 Electron 全量 feature parity。
- 不在本轮引入云同步、统计面板、外部生态集成或签名分发策略。

## Constraints & Assumptions
- 保持跨平台路线，避免重新回到 Electron。
- 暂不新增更多宿主依赖，优先用 `sysinfo` 和系统命令完成平台信号探测。
- 前端只保留一个极简设置工作台和一个 break prompt 入口。

## System Boundaries
- Modules:
  - `state.rs`：设置、当前休息、下次休息、pause/focus、idle、DND、app exclusion 的运行时真源
  - `platform.rs`：idle / DND / app exclusion 探测
  - `engine.rs`：后台 1s tick，驱动状态机和 break window / notification 动作
  - `shell.rs`：tray、shortcut、主窗口/休息窗口生命周期
  - `commands.rs`：前台 invoke 命令边界
- Ownership:
  - Rust host 拥有真实运行状态
  - React 前台只负责读取 snapshot 与提交用户动作
- Dependency direction:
  - frontend -> commands -> state
  - engine -> platform + state + shell
  - shell 不持有业务规则，只调用 state 和窗口 API

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
  - `toggle_autostart`
  - `show_main_window` / `hide_main_window`
- Request/Response schema (typed):
  - 所有可写命令返回 `DesktopSnapshot`
  - 前端设置提交使用 `PauzaSettings`
- Error model (codes, retryability):
  - 当前统一为 `String` 错误
  - 多数错误为不可重试的配置/宿主错误，由前台 banner 呈现

## Data Model / Storage
- `settings.json` 位于 Tauri app config 目录
- 运行时核心结构：
  - `PauzaSettings`
  - `RuntimeState`
  - `CurrentBreak`
  - `DesktopSnapshot`

## Invariants
- 任一时刻最多只有一个 `current_break`
- 阻塞态（pause/focus/DND/app exclusion/natural break）期间不应继续触发新休息
- 所有前台展示都来自同一个 `DesktopSnapshot`

## Concurrency / Lifecycle / Memory Model
- `PauzaState` 使用 `Arc<Mutex<RuntimeState>>`
- 后台 engine thread 每秒读取平台信号并调用 `tick`
- 前台 commands 与后台 tick 共用同一状态锁，避免并发漂移
- break window 由 `shell.rs` 在主线程创建，已有窗口则复用并显示

## Observability Plan (Debug-Driven)
- Logs:
  - `last_action`
  - `status`
  - `status_detail`
- Metrics:
  - 当前无独立 metrics 系统
- Traces:
  - dev runtime stdout
  - browser preview console log
- Debug flags:
  - 无额外开关；preview 即轻量调试入口

## Security & Privacy Considerations
- 只读本地设置文件与系统进程列表
- 未引入外部 API、账号体系或云端同步
- DND / idle 仅使用本地命令与系统接口

## Risks & Rollback
- Failure modes:
  - 平台命令不可用时，DND/idle 退化为 false/0
  - 窗口生命周期配置不当时可能导致 break prompt 悬空
- Rollback steps:
  - 回退 `apps/desktop/src-tauri/src/*.rs`
  - 恢复前一阶段 dashboard 版本
  - 同步回退 docs 与 changelog

## Acceptance Criteria (System)
- `cargo check` 通过
- `tauri:dev` 可启动
- state / engine / platform / shell / commands 的职责边界清晰

## Open Questions / Decision Requests
- 后续是否将 DND 与 idle 改为更强的原生插件层实现
- 下一阶段是否优先做 today 统计还是更智能的 not now / skip reason 闭环
