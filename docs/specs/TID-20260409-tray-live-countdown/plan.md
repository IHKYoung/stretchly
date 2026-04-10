# Task-ID: TID-20260409-tray-live-countdown

## Summary
- Title: 修复托盘实时倒计时
- Date: 2026-04-09
- Level: moderate
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 修复顶部 tray 倒计时看起来“卡住”的问题，让剩余时间按秒真实更新。
- In-scope:
  - tray 标题实时倒计时
  - tray tooltip 状态同步
  - Rust 纯函数测试
- Out-of-scope:
  - tray 菜单结构和动作
  - 调度状态机语义
  - 新增设置项或视觉重设计
- Assumptions:
  - 用户反馈的“顶部 tray”指 macOS menubar 上的托盘标题区域
  - 菜单内容不需要每秒重建，只需要顶部时间实时变化
- Risks:
  - Windows/Linux 的 tray title/tooltip 平台支持差异
  - 若把菜单也改成每秒刷新，会引入原生菜单闪烁回归
- Interaction impact: none
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 让 tray 顶部时间展示变成秒级真实倒计时，同时保留现有菜单刷新去抖策略。

## Scope
- In-scope:
  - `apps/desktop/src-tauri/src/shell.rs`
  - `docs/specs/TID-20260409-tray-live-countdown/*`
  - `docs/{plans,logs,CHANGELOG}/2026-04-09`
- Out-of-scope:
  - `apps/desktop/src-tauri/src/state.rs`
  - 前端设置页
  - tray icon 资源

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src-tauri/src/engine.rs`
  - `apps/desktop/src-tauri/src/state.rs`
  - `/Users/changkunyang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tauri-2.10.3/src/tray/mod.rs`
- Related docs/specs/logs reviewed:
  - `docs/Architecture.md`
  - 当日 `docs/plans/2026-04-09.md` / `docs/logs/2026-04-09.md`
- Why these are sufficient:
  - 已覆盖 tray 刷新入口、后台 1s tick、snapshot 真源以及 Tauri tray title/tooltip API 能力。

## Acceptance Criteria (AC)
- AC1: 当前 break、focus、定时 pause、下一次 scheduled break 的 tray 标题显示秒级倒计时。
- AC2: app exclusion / DND / natural break 等阻塞态不显示误导性的 scheduled countdown。
- AC3: tray 菜单仍按原来的状态/分钟桶变化刷新，不会为了倒计时每秒重建。
- AC4: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`npm --prefix apps/desktop run build` 通过。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: N/A
- Primary flow: N/A
- Fallback / secondary flow: N/A
- Interaction authority / ownership boundary: N/A
- Visible entrypoints / handoff cues: N/A
- In-scope interactions: N/A
- Out-of-scope interactions: N/A
- Interaction acceptance criteria: N/A
- Validator expectation: 当 `interaction_impact != none` 时，本节与 Requirement Brief 中的交互字段不得继续保留 `N/A/TBD`

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,architect,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: tray 标题/tooltip 展示层
- touches_running_service: no
- backup_required: no
- backup_plan: 以 Rust tests、desktop build 和 docs validator 为边界
- rollback_plan: 回退 `shell.rs` 与本任务 docs
- destructive_operations: none
- operator_approval_required: no
- rationale: 宿主展示层修复，不涉及外部副作用或持久化数据

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 将 tray 实时倒计时从菜单刷新解耦
  - DoD: 顶部倒计时通过独立 title 同步，每秒更新但不每秒重建菜单。
- [x] Task-2: 补充 tooltip 与回归测试
  - DoD: tooltip 能反映状态语义，纯函数测试覆盖关键倒计时分支。

## Evidence Plan (UI / E2E)
- Evidence required: no
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260409-tray-live-countdown/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: N/A
  - disabled: N/A
  - success: N/A

## Observability / Debug Plan
- Logs: 沿用现有 tray menu/status 刷新链路，不新增日志。
- Error codes: N/A
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 平台不支持的 title/tooltip API 若未做条件分支，可能引发宿主错误
  - 若标题和菜单一起每秒刷新，会复发 tray 菜单闪烁
- Rollback plan:
  - 回退 `shell.rs` 中的 live title/tooltip 同步逻辑和新增测试

## Sequential Phases
- phase_execution: N/A
- phase_confirmation_policy: N/A
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 审查 tray 当前刷新链路和 Tauri tray title API。
  2. 为现有 tray 增加每秒 title/tooltip 同步，不改变菜单刷新节奏。
  3. 增加 Rust 纯函数测试并执行验证。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes
- Approved: yes（orchestrator 已按当前 session 授权路由批准执行）
