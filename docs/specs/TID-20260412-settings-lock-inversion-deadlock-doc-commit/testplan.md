# Task-ID: TID-20260412-settings-lock-inversion-deadlock-doc-commit

## Test Strategy
- Unit:
  - `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- Integration:
  - `python3 scripts/validate_workflow_docs.py --mode manual`
  - `git diff --check`
- E2E (if applicable):
  - N/A。本任务不新增 UI 流程，只验证宿主层并发修正和文档门禁。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 代码审查 `apps/desktop/src-tauri/src/shell.rs`：`sync_tray_menu_text()` 锁内只 clone updater，锁外再执行 `set_text()`；`register_tray_menu_text_updater()` 保存的是 `Arc` 闭包而非持锁同步对象。
- AC2 -> 文档审查 `docs/specs/TID-20260412-settings-lock-inversion-deadlock-doc-commit/*`、`docs/Architecture.md`、`docs/CHANGELOG.md`、`docs/logs/2026-04-12.md`、`docs/plans/2026-04-12.md`。
- AC3 -> `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` + `python3 scripts/validate_workflow_docs.py --mode manual` + `git diff --check` + 本地 commit 完成。

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
- Artifacts path: docs/specs/TID-20260412-settings-lock-inversion-deadlock-doc-commit/evidence/
- What to capture:
  - Screenshots: N/A
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y: N/A
- perf budget: 不新增性能预算，但必须避免再次把主线程阻塞调用放回共享锁区间。
- error handling / observability: 文档必须明确说明死锁链和修复不变量。
- security / privacy: 不新增外部权限、数据或密钥。

## Boundary / Invalid Input Cases
- 旧 updater 在主线程重建期间被替换时，后台线程持有的 `Arc` 仍应保持一次调用有效，不应悬垂。
- 当 tray 尚未创建时，`sync_tray_menu_text()` 仍应安全返回，不因空 updater 触发异常。

## Concurrency / Race Cases (if applicable)
- 后台 tick 与设置保存触发的 tray rebuild 并发发生时，主线程不能再等待 `LAST_TRAY_MENU_TEXT_UPDATER` 被后台线程释放。
- 持锁区间内禁止再引入会同步等待主线程执行完成的宿主 API。

## Mocks & Test Data
- 无额外 mock 或测试数据。

## Commands to Run
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `python3 scripts/validate_workflow_docs.py --mode manual`
- `git diff --check`

## Expected Results
- PASS criteria:
- PASS criteria:
  - Rust tests 全部通过。
  - workflow docs validator 通过，且本 Task-ID 不再保留 `TBD/INIT` 占位。
  - `git diff --check` 无 whitespace / conflict 标记问题。
- Outputs to keep (10~20 lines snippet):
  - `test result: ok. ... passed; 0 failed`
  - `[OK] Workflow docs validation passed for 2026-04-12 ✅`
