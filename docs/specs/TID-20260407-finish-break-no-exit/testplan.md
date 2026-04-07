# Task-ID: TID-20260407-finish-break-no-exit

## Test Strategy
- Unit:
  - `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- Integration:
  - `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - `npm --prefix apps/desktop run typecheck`
  - `npm --prefix apps/desktop run build`
  - `python3 scripts/validate_workflow_docs.py --mode manual`
- E2E (if applicable):
  - 需用户本机真实点击一次“完成休息”验证应用未退出。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 代码审查 `commands.rs`，确认 break CTA 改为 `close_break_window_deferred(app.clone())`
- AC2 -> 代码审查 `shell.rs` 与用户手动验证，确认 break window teardown 被延后到命令回包之后
- AC3 -> 代码审查 `commands.rs`，确认 `skip_current_break` / `postpone_current_break` 也走同一 deferred close
- AC4 -> 代码审查 `shell.rs`，确认 `close_break_window()` 仍保留 fullscreen exit + destroy
- AC5 -> `cargo check` + `cargo test` + `typecheck` + `build` + docs validator

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: 代码路径保证 `finish_current_break` 先返回 snapshot，再异步 teardown break window；现实点击验证仍需用户本机完成。
- Fallback / secondary flow -> tests/evidence: `skip` / `postpone` 共享同一 deferred close 路径，由代码审查覆盖。
- Visible states / transitions -> tests/evidence: `visible break` -> `CTA click` -> `snapshot returned` -> `window teardown` 的状态转换已在 `commands.rs` / `shell.rs` 中显式实现。
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
- Artifacts path: docs/specs/TID-20260407-finish-break-no-exit/evidence/
- What to capture:
  - Screenshots: 无新鲜原生截图
  - Video/trace (optional): 无
  - HAR/console logs (optional): 无

## Quality Gates (Non-functional)
- a11y: 不改前端结构，维持现状
- perf budget: 仅新增一次 75ms deferred close，不引入新依赖
- error handling / observability: 编译链和 docs validator 通过
- security / privacy: 不新增权限、网络和数据路径

## Boundary / Invalid Input Cases
- 没有 active break 时，命令仍应安全返回 snapshot，不触发 break window teardown。
- fullscreen break 关闭时仍应先退出 fullscreen 再 destroy。

## Concurrency / Race Cases (if applicable)
- 当前修复专门覆盖“当前 break webview 正在等待 `invoke` 回包，同时宿主想同步 destroy 它”的生命周期竞态。

## Mocks & Test Data
- 不适用；本轮以真实宿主代码路径和现有单测为主。

## Commands to Run
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
- Rust host 与前端构建链通过
- `commands.rs` 先回包再 deferred close
- `shell.rs` 仍保留 fullscreen exit + destroy
- docs 明确记录最终现实确认缺口
- Outputs to keep (10~20 lines snippet):
  - `Finished 'dev' profile [unoptimized + debuginfo] target(s) in 5.12s`
  - `test result: ok. 3 passed; 0 failed`
  - `✓ built in 1.39s`
