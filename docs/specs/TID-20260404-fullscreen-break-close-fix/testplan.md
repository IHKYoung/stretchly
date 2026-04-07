# Task-ID: TID-20260404-fullscreen-break-close-fix

## Test Strategy
- Unit:
  - 无新增单测；本轮为宿主窗口生命周期修复。
- Integration:
  - `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - `npm --prefix apps/desktop run typecheck`
  - `npm --prefix apps/desktop run build`
- E2E (if applicable):
  - 需用户本机手动验证 fullscreen break 点击 `跳过` 后不再黑屏。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 代码审查 `shell.rs`，确认不再只是 `hide()`
- AC2 -> 代码审查 `shell.rs`，确认 fullscreen break 先 `set_fullscreen(false)`
- AC3 -> 代码审查 `shell.rs`，确认 break windows 被 `destroy()`
- AC4 -> `cargo check` + `typecheck` + `build`
- AC5 -> docs 中显式记录真实 fullscreen 验证缺口

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: 代码路径已保证 fullscreen break 关闭前先退出 fullscreen，再销毁窗口；最终 UI 现实确认待用户本机点击验证。
- Fallback / secondary flow -> tests/evidence: `完成` / `稍后` 共用同一关闭链路，因此受同一代码修复覆盖。
- Visible states / transitions -> tests/evidence: 从 fullscreen visible 到退出 fullscreen 再销毁 break window 的状态转换已在 `shell.rs` 中显式实现。
- Validator expectation: 证据字段已经补齐，不保留占位内容。

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
- Artifacts path: docs/specs/TID-20260404-fullscreen-break-close-fix/evidence/
- What to capture:
  - Screenshots: 本轮无新鲜 fullscreen 截图
  - Video/trace (optional): 无
  - HAR/console logs (optional): 无

## Quality Gates (Non-functional)
- a11y: N/A，本轮不改前端结构
- perf budget: 无新增依赖或重资源
- error handling / observability: 编译链通过
- security / privacy: 不新增权限、网络或数据路径

## Boundary / Invalid Input Cases
- 无 active break 时，`close_break_window()` 仍应安全处理空窗口列表。
- 非 fullscreen break 也应沿用同一 destroy 路径，不引入回归。

## Concurrency / Race Cases (if applicable)
- tray action、break CTA 和 window close 都可能并发触发同一 close path；本轮修复不应依赖前端顺序。

## Mocks & Test Data
- 不适用；本轮以真实宿主代码路径为主。

## Commands to Run
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`

## Expected Results
- PASS criteria:
  - Rust host 与前端构建通过
  - `shell.rs` 显式先退出 fullscreen 再 destroy
  - docs 诚实记录仍待用户现实确认
- Outputs to keep (10~20 lines snippet):
  - `Finished dev profile [unoptimized + debuginfo] target(s) in 1.95s`
  - `✓ 1821 modules transformed.`
  - `✓ built in 1.37s`
