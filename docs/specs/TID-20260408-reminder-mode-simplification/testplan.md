# Task-ID: TID-20260408-reminder-mode-simplification

## Test Strategy
- Unit: Rust 单测覆盖智能提醒等待空档、强制提醒立即严格开始、自然休息重置、旧 strict 设置迁移。
- Integration: `cargo check`、`npm --prefix apps/desktop run build`、workflow docs validator。
- E2E (if applicable): N/A（本轮以 host 状态机和设置页构建验证为主）

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 人工检查 `App.tsx` 和 locale：设置页只剩 reminder mode 与 natural breaks，不再暴露 strict mode / break style 控件。
- AC2 -> Rust 单测：智能提醒到点后等待空档；soft nudge 不再存在。
- AC3 -> Rust 单测：`PlatformMonitor::probe()` 在 `natural_breaks=false` 时仍返回 idle signal。
- AC4 -> Rust 单测：强制提醒开始后的 `can_skip` / `can_postpone` / window-close 路径受 strict 限制。
- AC5 -> `cargo test` / `cargo check` / `npm --prefix apps/desktop run build` / `python3 scripts/validate_workflow_docs.py --mode manual`

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: `forced_mode_starts_break_immediately`、`due_break_waits_for_idle_opportunity`、evidence/README.md
- Fallback / secondary flow -> tests/evidence: `legacy_strict_settings_migrate_to_forced_mode`、自然休息与 blocker 仍由既有状态机覆盖
- Visible states / transitions -> tests/evidence: `snapshot_status_reflects_waiting_for_opportunity`、evidence/README.md

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
- Owner: orchestrator（single-agent-fallback）
- Artifacts path: docs/specs/TID-20260408-reminder-mode-simplification/evidence/
- What to capture:
  - Screenshots: N/A
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y: 设置页新增 segmented control 文案需清晰可读。
- perf budget: 不应引入比当前更重的后台探测或频繁 tray 刷新。
- error handling / observability: settings 迁移失败不得破坏默认启动；`last_action` / status 文案应能解释当前处于等待空档还是自然休息。
- security / privacy: 继续只使用系统 idle time，不引入内容级输入监听。

## Boundary / Invalid Input Cases
- 旧 settings.json 中存在 `microbreakStrictMode=true` 或 `longBreakStrictMode=true` 时，应迁移为 `reminderMode=forced`。
- `natural_breaks=false` 时也必须继续采集 `idle_ms`，否则智能提醒无法工作。
- 强制提醒下即便 `allow_postpone=true`，运行时也不能允许 postpone。

## Concurrency / Race Cases (if applicable)
- due、blocker、natural break 同时竞争时，blocker / natural break 仍优先于开始 break。
- 等待空档中的 due break 在用户刚空下来时只能开始一次，不得重复开窗。

## Mocks & Test Data
- 使用内存中的 `RuntimeState` 和手工构造的 settings / now / idle_ms。

## Commands to Run
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - Rust 单测全部通过，且覆盖新 reminder mode 主路径。
  - 前端构建通过。
  - workflow docs validator 通过。
- Outputs to keep (10~20 lines snippet):
  - `cargo test` 汇总
  - `cargo check` 完成行
  - `npm --prefix apps/desktop run build` 完成行
  - docs validator 的 `[OK]` 输出
