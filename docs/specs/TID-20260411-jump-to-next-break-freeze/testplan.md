# Task-ID: TID-20260411-jump-to-next-break-freeze

## Test Strategy
- Unit: 不新增纯逻辑单测；tray 菜单生命周期问题主要在宿主刷新时机，优先通过 Rust 构建与源码审查验证
- Integration: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- E2E (if applicable): `npm --prefix apps/desktop run build` 确认前端/宿主未被回归；用户可在本机执行 tray 手工复测

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 代码审查 `shell.rs`：tray `on_menu_event` 不再直接调用同步 `refresh_tray()`，而是延后并走 `refresh_tray_if_needed()`
- AC2 -> 代码审查 `handle_tray_action`、`state.rs` 的 `skip_to_*` 与 shortcut/command 路径，确认 action id 与调度语义未改
- AC3 -> `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` PASS；`npm --prefix apps/desktop run build` PASS；`python3 scripts/validate_workflow_docs.py --mode manual` PASS

## Interaction Contract Coverage
- Interaction impact: none  <!-- none | indirect | direct -->
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
- `orchestrator` 未显式使用 `sandbox_mode = "danger-full-access"` 与 `approval_policy = "never"`，却仍宣称当前仓库运行在 aggressive 基线。
- 代码变更前未记录 `Source Basis`，导致实现依据不可追溯。
- 子 agent 未显式 `approval_policy = "never"`。
- `moderate/complex` 任务通过缩小 `Required Roles` 伪装为 trivial fallback。
- 已使用 `single-agent-fallback`，但 logs/plans 没有单独记录 `Execution Mode` / `Fallback Scope` / `Fallback Reason Code`。
- 需要受角色边界约束的文件系统写命令没有经过 `run_role_guard.py`，只在结案时补跑范围校验。
- `git add -- <explicit paths...>` 仍被包进 wrapper / helper script，导致运行时看不到裸命令前缀。
- `interaction_impact != none`，但 plan/testplan/ui spec 没有定义 primary flow / fallback flow / visible states / evidence coverage。

## Evidence Capture (UI / E2E)
- Required: no   <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260411-jump-to-next-break-freeze/evidence/
- What to capture:
  - Screenshots: N/A
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y: 不改变现有 tray 文案和操作入口
- perf budget: 不新增后台轮询，只把 tray action 后的刷新从立即执行改为延后按需执行
- error handling / observability: 保持 `Result<(), String>` 与现有 `last_action` 观测口径
- security / privacy: 不新增权限、不引入网络或外部副作用

## Boundary / Invalid Input Cases
- `quit` 动作不应被额外刷新逻辑阻塞
- shortcut / settings 页面触发的 tray 刷新不应被 tray 菜单专属 workaround 影响

## Concurrency / Race Cases (if applicable)
- tray 菜单点击后的主线程刷新需与 native menu 关闭生命周期解耦
- engine 的 1s tick 仍走 `refresh_tray_if_needed()`，不应退回到每次强制重建

## Mocks & Test Data
- 复用现有 `PauzaState` 与 tray 宿主链路；不引入 mocks

## Commands to Run
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
- tray action 后的宿主刷新时机已改为延后按需执行
- Rust tests / 前端构建 / docs validator 通过
- Outputs to keep (10~20 lines snippet):
- `test result: ok`
- `✓ built in`
- `[OK] Workflow docs validation passed`
