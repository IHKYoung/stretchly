# Task-ID: TID-20260410-pre-break-heads-up-restoration

## Test Strategy
- Unit:
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- Integration:
- `python3 scripts/sync_desktop_locales.py`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`
- `python3 scripts/validate_agent_configs.py`
- E2E (if applicable):
- tray 属于系统级表面，当前无可编排自动化入口；最终可见性由用户本机观察一次实际运行态作为现实确认。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `state::tests::snapshot_status_reflects_pre_break_heads_up`；代码审查 `heads_up_kind()` 与 `status()` 分支
- AC2 -> `state::tests::snapshot_status_reflects_waiting_for_opportunity`、`state::tests::due_break_waits_for_idle_opportunity`、`state::tests::smart_mode_starts_break_after_final_stage_even_without_idle_gap`
- AC3 -> 代码审查 `engine.rs` 的 `failed to show desktop notification` 日志分支
- AC4 -> locale diff + `python3 scripts/sync_desktop_locales.py`；文档对照 `docs/ReminderScheduling.md`、`docs/SettingsInventory.md`、`docs/Architecture.md`、`docs/UI.md`
- AC5 -> `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`；`npm --prefix apps/desktop run build`

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: `snapshot_status_reflects_pre_break_heads_up` 证明 due 前进入 heads-up；evidence README 记录文案和构建结果
- Fallback / secondary flow -> tests/evidence: `snapshot_status_reflects_waiting_for_opportunity` 证明 due 后 smart 模式转入等待空档；`engine.rs` 日志分支覆盖通知失败兜底
- Visible states / transitions -> tests/evidence: `scheduled -> heads-up -> due -> waiting/active` 通过状态机单测与代码路径对照验证
- Validator expectation: 交互字段与 Evidence Capture 已补齐

## Governance Gates
- Agent Config Validation: `python3 scripts/validate_agent_configs.py`
- Workflow Docs Validation: `python3 scripts/validate_workflow_docs.py --mode manual`
- Approval Escalation Owner: orchestrator

## False-pass Cases
- `DONE` 任务对应的 spec 仍保留 `TBD/INIT` 占位。
- `orchestrator` 未显式使用 `sandbox_mode = "danger-full-access"` 与 `approval_policy = "never"`，却仍宣称当前仓库运行在 aggressive 基线。
- 代码变更前未记录 `Source Basis`，导致实现依据不可追溯。
- 子 agent 未显式 `approval_policy = "never"`。
- `moderate/complex` 任务通过缩小 `Required Roles` 伪装为 trivial fallback。
- 已使用 `single-agent-fallback`，但 logs/plans 没有单独记录 `Execution Mode` / `Fallback Scope` / `Fallback Reason Code`。
- 需要受角色边界约束的文件系统写命令没有经过 `run_role_guard.py`，只在结案时补跑范围校验。
- `git add -- <explicit paths...>` 仍被包进 wrapper / helper script，导致运行时看不到裸命令前缀。
- `interaction_impact != none`，但 plan/testplan/ui spec 没有定义 primary flow / fallback flow / visible states / evidence coverage。

## Evidence Capture (UI / E2E)
- Required: partial   <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260410-pre-break-heads-up-restoration/evidence/
- What to capture:
  - Screenshots: 当前未新增独立前台控件，主要证据不是浏览器截图
  - Video/trace (optional): N/A
  - HAR/console logs (optional): 记录 Rust 测试、locale sync 与 desktop build 输出摘要

## Quality Gates (Non-functional)
- a11y: 不新增新的前台控件与交互，仅更新状态文案语义
- perf budget: `heads_up_kind()` 为常量级派生判断，不新增后台线程或轮询频率
- error handling / observability: 通知失败改为显式 `eprintln!`，不再静默吞掉
- security / privacy: 不新增权限、网络、文件上传或数据采集

## Boundary / Invalid Input Cases
- 关闭某类 break 的 heads-up 开关后，该类 break 不应进入 heads-up 状态。
- heads-up 只在 due 前有效；due 后必须转入 `等待空档` 或 `break active`。
- 若系统通知底层失败，heads-up 仍需通过 runtime 状态继续存在。

## Concurrency / Race Cases (if applicable)
- 本轮只在现有后台 1s tick 上增加派生状态判断，不新增共享状态或线程同步边界。
- heads-up 状态来自 `next_break_due_ms` 和设置 lead time 计算，因此不会因一次性通知发送后清空 `next_notification_due_ms` 而瞬间消失。

## Mocks & Test Data
- 使用 `PauzaSettings::default()` 和已有调度状态机构造测试数据。
- `snapshot_status_reflects_pre_break_heads_up` 使用相对 `now_ms()` 的 due 时间验证 heads-up 窗口。

## Commands to Run
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `python3 scripts/sync_desktop_locales.py`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`
- `python3 scripts/validate_agent_configs.py`

## Expected Results
- PASS criteria:
- `state::tests::snapshot_status_reflects_pre_break_heads_up ... ok`
- `state::tests::snapshot_status_reflects_waiting_for_opportunity ... ok`
- `test result: ok. 23 passed; 0 failed`
- `✓ 1825 modules transformed.`
- Outputs to keep (10~20 lines snippet):
- `test state::tests::snapshot_status_reflects_pre_break_heads_up ... ok`
- `test state::tests::snapshot_status_reflects_waiting_for_opportunity ... ok`
- `test result: ok. 23 passed; 0 failed`
- `[OK] Built desktop locale registry: 50 languages`
- `✓ built in 1.82s`
