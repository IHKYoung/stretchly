# Task-ID: TID-20260409-tray-live-countdown

## Test Strategy
- Unit:
  - Rust 纯函数测试覆盖 countdown 格式、当前 break 优先级、阻塞态隐藏 countdown。
- Integration:
  - `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - `npm --prefix apps/desktop run build`
- E2E (if applicable):
  - N/A

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `shell::tests::format_tray_countdown_uses_live_seconds` + `shell::tests::tray_title_prefers_current_break_remaining`
- AC2 -> `shell::tests::blocked_states_hide_schedule_countdown`
- AC3 -> 代码审查 `TrayRefreshKey` 与 `refresh_tray_if_needed()` 未改为每秒菜单重建
- AC4 -> `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`npm --prefix apps/desktop run build`

## Interaction Contract Coverage
- Interaction impact: none
- Primary flow -> tests/evidence: N/A
- Fallback / secondary flow -> tests/evidence: N/A
- Visible states / transitions -> tests/evidence: N/A
- Validator expectation: 当 `interaction_impact != none` 时，本节三项与 Evidence Capture 的 `Required` 不得继续保留 `N/A/no/TBD`

## Governance Gates
- Agent Config Validation: `python3 scripts/validate_agent_configs.py`
- Workflow Docs Validation: `python3 scripts/validate_workflow_docs.py --mode manual`
- Approval Escalation Owner: orchestrator

## False-pass Cases
- `DONE` 任务对应的 spec 仍保留 `TBD/INIT` 占位。
- tray title 虽然更新了，但菜单也被改成每秒重建。
- 只改了格式函数，没有把实时 title 同步接入现有 tray。
- 阻塞态仍显示 schedule countdown。

## Evidence Capture (UI / E2E)
- Required: no
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260409-tray-live-countdown/evidence/
- What to capture:
  - Screenshots: N/A
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y: 维持系统 tray 原生可读性
- perf budget: 不允许把菜单重建频率提升到每秒
- error handling / observability: 平台不支持 title/tooltip 时走条件分支
- security / privacy: 不新增数据暴露

## Boundary / Invalid Input Cases
- 倒计时为 0 时应显示 `0:00`
- 超过 1 小时时应切换到 `h:mm:ss`
- 无有效 countdown 时应清空标题

## Concurrency / Race Cases (if applicable)
- 后台 1s tick 与主线程 tray 更新之间不能引入额外共享状态竞争

## Mocks & Test Data
- 直接构造 `DesktopSnapshot` / `CurrentBreakSnapshot`

## Commands to Run
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - Rust 测试包含新增 `shell::tests::*` 并全部通过
  - desktop build 通过
  - workflow docs validator 通过
- Outputs to keep (10~20 lines snippet):
  - `running 15 tests`
  - `test shell::tests::format_tray_countdown_uses_live_seconds ... ok`
  - `test shell::tests::tray_title_prefers_current_break_remaining ... ok`
  - `test shell::tests::blocked_states_hide_schedule_countdown ... ok`
