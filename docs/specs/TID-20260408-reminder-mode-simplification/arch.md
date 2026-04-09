# Task-ID: TID-20260408-reminder-mode-simplification

## Goals
- 把提醒投递策略收敛到一个统一的 `reminder_mode` 真源。
- 让 `forced` 直接代表严格 break，而不是和独立 strict setting 并列。
- 去掉 soft nudge，把状态机回到最小必要复杂度。

## Non-Goals
- 不回填 Electron legacy 的同等模型。
- 不引入新的 OS 级输入监听能力。
- 不恢复 per-break-kind 的独立 reminder mode。

## Constraints & Assumptions
- 继续只使用系统 idle time 作为“是否仍在操作电脑”的代理。
- `natural_breaks` 只做重置规则，不参与提醒方式选择。
- break window 只保留 `窗口 / 全屏` 一根设置轴；原 `break_prompt_style` 从主实现与设置 schema 中移除。

## System Boundaries
- Modules:
  - `state.rs`：settings 真源、tick 状态机、break 生命周期与迁移逻辑
  - `platform.rs`：idle / DND / app exclusion 探测
  - `shell.rs`：严格 break 的 tray / close 行为
- Ownership:
  - 调度与可绕过性归 `state.rs`
  - 窗口呈现归 `shell.rs`
- Dependency direction:
  - `platform -> state -> shell`

## API / Contract
- Request/Response schema (typed):
  - `PauzaSettings` 新增 `reminderMode`
  - `DesktopSnapshot.settings.reminderMode` 作为前台唯一 reminder mode 真源
  - `CurrentBreakSnapshot.strictMode` 现在由 `reminderMode == forced` 推导

## Data Model / Storage
- 新增持久化字段：
  - `reminderMode: smart | forced`
  - `idleOpportunitySeconds: number`（hidden）
- 删除持久化真源：
  - `microbreakStrictMode`
  - `longBreakStrictMode`
  - `adaptiveBreaksEnabled`
  - `microbreakIdleOpportunitySeconds`
  - `longBreakIdleOpportunitySeconds`
  - `microbreakSoftNudgeSeconds`
  - `longBreakSoftNudgeSeconds`
- 加载旧 `settings.json` 时，若发现 legacy strict 字段，则迁移到 `reminderMode=forced`。

## Invariants
- `forced` break 一旦开始，不能 skip、postpone、window close 绕过。
- `smart` break 到点后若 `idle_ms` 未达到机会阈值，则只等待，不再 soft nudge。
- `idle_ms` 必须始终可用，不得再依赖 `natural_breaks` 开关。

## Concurrency / Lifecycle / Memory Model
- `tick()` 先判 blocker / natural break，再判 due，再决定是否等待空档或开始 break。
- `next_break_wait_started_ms` 是唯一“等待空档中”运行时标记。

## Observability Plan (Debug-Driven)
- Logs:
  - 继续复用 `last_action`
- Debug flags:
  - 仅保留 hidden `idleOpportunitySeconds`

## Security & Privacy Considerations
- 不读取输入内容，只读取系统级 idle time。

## Risks & Rollback
- Failure modes:
  - 旧 strict 配置迁移失败
  - 强制提醒仍有绕过路径
- Rollback steps:
  - 回退 `state.rs`、`platform.rs` 与 `shell.rs` 相关改动

## Acceptance Criteria (System)
- `state.rs` 只有 smart / forced 两种提醒方式。
- `forced` break 不可 skip / postpone / close。
- `platform.rs` 在 `natural_breaks=false` 时仍提供 idle signal。

## Open Questions / Decision Requests
- N/A
