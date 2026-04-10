# Task-ID: TID-20260409-break-window-native-exception-guard

## Test Strategy
- Unit:
  - `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- Integration:
  - `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - `npm --prefix apps/desktop run build`
  - `python3 scripts/validate_workflow_docs.py --mode manual`
  - `python3 scripts/validate_agent_configs.py`
- E2E (if applicable):
  - 当前缺少可直接强制进入 break 的自动化入口；最终现实确认需用户在 macOS 本机手动触发一次 break 进入流程。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 代码审查 `shell.rs` 中 `run_macos_native_break_window_patch()`；crash report 栈证明原问题确实落在 ObjC message send 路径
- AC2 -> 代码审查 `show_break_window()`，确认 `configure_break_window_native_behavior()` 与 `present_break_window()` 已移到 `window.show()` 之后
- AC3 -> `shell.rs` 中 exception 分支输出 `eprintln!` 并返回 `Ok(())`；`cargo check` / `cargo test` 保证接线无编译回归
- AC4 -> `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`；`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`；`npm --prefix apps/desktop run build`；`python3 scripts/validate_workflow_docs.py --mode manual`；`python3 scripts/validate_agent_configs.py`

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: `show_break_window()` 现在先 `window.show()` 再做 native patch；自动化通过代码路径与构建链证明接线正确。
- Fallback / secondary flow -> tests/evidence: ObjC exception 或空句柄路径经 `run_macos_native_break_window_patch()` 降级为 no-op，不再 abort；证据见 crash report 分析与 `evidence/README.md`。
- Visible states / transitions -> tests/evidence: `break due -> window.show() -> native patch attempt -> success or skip-on-exception` 的宿主状态转换已在 `shell.rs` 中显式实现，并在 docs 中记录。
- Validator expectation: 交互字段和 Evidence Capture 已补齐。

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
- Artifacts path: docs/specs/TID-20260409-break-window-native-exception-guard/evidence/
- What to capture:
  - Screenshots: 无新鲜原生截图；本轮主证据来自 crash report 与代码路径
  - Video/trace (optional): 无
  - HAR/console logs (optional): 使用 crash report 栈与构建命令输出替代

## Quality Gates (Non-functional)
- a11y: 不改前端结构和键盘路径，维持现状
- perf budget: 仅新增一个 best-effort exception guard，不引入新依赖和额外后台任务
- error handling / observability: ObjC exception 由直接 abort 改为可追踪的 `eprintln!` + graceful no-op
- security / privacy: 不新增权限、网络、日志上报或数据采集

## Boundary / Invalid Input Cases
- `window.ns_window()` 返回空句柄时 helper 应直接 no-op，不阻断 break 宿主后续逻辑。
- ObjC exception 发生在 `collectionBehavior`、`setLevel` 或 `orderFrontRegardless` 任一步时，都不应再终止进程。

## Concurrency / Race Cases (if applicable)
- 所有 native patch 继续发生在 `show_break_window()` 的主线程窗口显示时序中，不新增后台线程或共享状态。
- 将 patch 延后到 `window.show()` 之后，目的是降低窗口尚未 ready 时访问原生句柄的生命周期竞态。

## Mocks & Test Data
- 使用本机 `pauza-desktop-2026-04-09-224258.ips` 与 `pauza-desktop-2026-04-09-232038.ips` 作为事故样本。
- Rust 侧使用现有 `shell::tests::macos_break_window_native_overlay_policy_uses_expected_levels` 与构建链回归，不新增 mock。

## Commands to Run
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`
- `python3 scripts/validate_agent_configs.py`

## Expected Results
- PASS criteria:
  - `shell.rs` 的 macOS break native patch 具备 exception guard，且时机已移到 `window.show()` 之后
  - Rust host 与 desktop 前端构建链通过
  - workflow docs 与 agent config validator 通过
  - Evidence 明确记录 crash report 依据与“仍需用户本机手动进入一次 break”的现实缺口
- Outputs to keep (10~20 lines snippet):
  - `test shell::tests::macos_break_window_native_overlay_policy_uses_expected_levels ... ok`
  - `test result: ok. 22 passed; 0 failed`
  - `✓ built in 13.96s`
