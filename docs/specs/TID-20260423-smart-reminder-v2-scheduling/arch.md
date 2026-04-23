# Task-ID: TID-20260423-smart-reminder-v2-scheduling

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 将 smart reminder 从“单一 idle gap 触发 break”改成“运行中 break 生命周期优先、blocker 冻结投递、恢复 credit 结算”的 host 状态机。
- 修正 passive blocker（pause/focus/DND/app exclusion/natural break）会重置或吞掉当前提醒节奏的问题。
- 在不新增依赖、不改前台设置结构的前提下，让等待空档和恢复结算具备更稳定、可解释的语义。

## Non-Goals
- 不引入新的系统权限、全局输入监听或外部服务。
- 不新增新的 reminder mode，也不把内部阈值重新暴露到设置页。
- 不重做 `App.tsx` 布局、tray 菜单结构或 break window 视觉，只更新 host 提供的状态文案与触发时机。

## Constraints & Assumptions
- 当前唯一稳定输入信号仍是 `idle_ms`，无法精确判断“人在不在桌前”或更高阶专注语义。
- 现有 `smart / forced / naturalBreaks` 用户概念保持不变；本轮只收敛其 host 侧执行语义。
- 现有调度主链仍收口在 `apps/desktop/src-tauri/src/state.rs`，`engine.rs` 继续以 1s tick 驱动。

## System Boundaries
- Modules:
  - `apps/desktop/src-tauri/src/state.rs`：提醒状态机、blocker freeze、恢复 credit、状态文案真源。
  - `apps/desktop/src-tauri/src/engine.rs`：后台 tick 与通知/开窗执行。
  - `apps/desktop/src-tauri/src/commands.rs`：pause/focus/reset/skip 命令边界，需与新 freeze 语义对齐。
  - `apps/desktop/src-tauri/src/platform.rs`：idle / DND / app exclusion 轻量探测，不扩充信号种类。
  - `apps/desktop/src/locales/messages/{zh-CN,en}.json`：等待空档、恢复结算与状态细节文案。
- Ownership:
  - host 状态机与 snapshot/status 契约由 `state.rs` 负责。
  - `commands.rs` 只负责运行时动作入口，不自行重写 reminder 语义。
  - 前台与 tray 继续只消费 snapshot 文本，不引入第二套派生逻辑。
- Dependency direction:
  - `platform -> engine -> state`
  - `commands -> state`
  - `state -> i18n`
  - 前台 / tray -> `DesktopSnapshot`

## API / Contract
- Signatures / Endpoints:
  - `PauzaState::tick(now, idle_ms, dnd_active, app_exclusion_match) -> EngineActions`
  - `PauzaState::{pause_for_minutes,resume,start_focus_session,clear_focus_session}`
  - `RuntimeState::status(now) -> (String, String)`
- Request/Response schema (typed):
  - 不新增对外 command 或 settings schema；新语义收口在 `RuntimeState` 内部字段与 `DesktopSnapshot.status/status_detail/last_action`。
  - `idle_opportunity_seconds` 继续只保留为兼容字段，不重新变成当前策略的主阈值。
- Error model (codes, retryability):
  - 无新增错误码。
  - 被 forced break 锁住的现有命令失败路径保持不变。

## Data Model / Storage
- 持久化：不新增 settings 字段。
- 运行时：
  - 保留 `next_break_due_ms` / `next_break_kind` / `next_break_wait_started_ms`。
  - 新增 blocker freeze 起点，用于把 pause/focus/DND/app exclusion 的阻塞时间平移到 due / notification / waiting timer。
  - 基于最近一次长 idle gap 计算恢复 credit，不单独持久化。

## Invariants
- passive blocker 不得关闭已经开始的 break；active break 生命周期优先于 blocker 投递判定。
- pause/focus/DND/app exclusion 只冻结投递，不重置节奏。
- smart 模式下，若 idle gap 已进入恢复 credit 区间，不应在用户离开时直接弹出 break。
- full reset 只由足够长的自然离开触发；短于 full reset 的离开只能消费 microbreak 或顺延 long break。

## Concurrency / Lifecycle / Memory Model
- `RuntimeState` 继续由单一 `Mutex` 串行更新；freeze / release / recovery credit 都必须在同一状态真源内完成。
- `tick()`、pause/focus 命令与 snapshot 读取都共享同一份 pending schedule，避免 blocker 与恢复 credit 产生双真源。
- 进入 blocker 与解除 blocker 时，due / notification / waiting timer 的平移必须幂等，避免重复累计。

## Observability Plan (Debug-Driven)
- Logs:
  - 复用 `last_action` 输出 blocker freeze、等待空档、恢复 credit、full reset 等关键状态切换。
  - `status_detail` 补充“最多还会再等多久”或“已按离开时长顺延/抵扣”的人类可读解释。
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - 无新增 debug flag；通过单测和 snapshot 文案自证。

## Security & Privacy Considerations
- 继续只消费本地 idle / DND / app exclusion 信号，不新增键盘监听、埋点或外发。
- 不修改权限面，不引入网络请求或外部成本。

## Risks & Rollback
- Failure modes:
  - blocker 与恢复 credit 重叠时，due 平移可能出现重复累加或未累加。
  - microbreak credit 可能错误推进 `cycle_index`，导致 long break 节奏漂移。
  - 等待空档 / 恢复 credit 文案若与真实状态不一致，会让用户觉得随机。
- Rollback steps:
  - 回退 `state.rs`、`commands.rs`、locale 与相关文档。
  - 重新运行 `cargo test`、`npm test`、`npm run typecheck`、`npm --prefix apps/desktop run build` 与 workflow validator。

## Acceptance Criteria (System)
- passive blocker 不会中断已经开始的 break；只有用户主动 pause/focus/skip/reset 才会关闭当前 break。
- pause/focus/DND/app exclusion 开始后会冻结 pending schedule；解除后不会整轮 reset，而是按阻塞时长平移 due / waiting timer。
- smart 模式下，用户长时间离开时不会直接在离开过程中启动 break，而是在返回时按 idle gap 执行 microbreak 抵扣、long break 顺延或 full reset。
- snapshot/status 能稳定区分 `heads-up`、`waiting for opportunity`、`recovery hold` 与 `natural break reset`。

## Open Questions / Decision Requests
- 本轮先用本地规则化 credit（`45s` 起记 credit，`5min` full reset）实现 v2，不继续扩展更复杂的 activity score；若后续仍觉得不够聪明，再单开 v3 任务引入更细的活动强度模型。
