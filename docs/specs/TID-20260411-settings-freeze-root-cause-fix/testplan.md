# Task-ID: TID-20260411-settings-freeze-root-cause-fix

## Test Strategy
- Unit:
  - Rust `commands.rs` 新增纯函数测试，验证 settings 变化只触发必要的 host refresh
- Integration:
  - `npm test`
  - `npm run typecheck`
  - `npm --prefix apps/desktop run build`
  - `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - `python3 scripts/validate_workflow_docs.py --mode manual`
- E2E (if applicable):
  - 本轮不新增完整桌面端 E2E；交互契约主要由源码链路分析 + 自动化验证覆盖

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `App.tsx` 串行 autosave 代码路径审查 + `npm run typecheck` + `npm --prefix apps/desktop run build`
- AC2 -> `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` 中新增的 `commands::tests::*`
- AC3 -> `state.update_settings()` no-op 早退代码路径审查 + `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- AC4 -> `npm test`、`npm run typecheck`、`npm --prefix apps/desktop run build`、`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`python3 scripts/validate_workflow_docs.py --mode manual`

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: `App.tsx` autosave 串行化实现 + 本任务 `evidence/README.md`
- Fallback / secondary flow -> tests/evidence: 保存进行中继续修改时的“下一轮重试”逻辑代码路径 + 本任务 `evidence/README.md`
- Visible states / transitions -> tests/evidence: `busyAction='save settings'`、`dirty`、`formRevision`、Rust host refresh diff path 代码审查 + 构建/测试结果

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
- Artifacts path: docs/specs/TID-20260411-settings-freeze-root-cause-fix/evidence/
- What to capture:
  - Screenshots: N/A（本轮以源码路径 + 自动化验证为主）
  - Video/trace (optional): N/A
  - HAR/console logs (optional): 记录测试/构建与 validator 输出摘要

## Quality Gates (Non-functional)
- a11y: 不应影响现有设置页键盘输入与 blur / Enter / Escape 行为
- perf budget: 设置保存不再无条件触发快捷键重绑 + tray 整棵 rebuild
- error handling / observability: 失败时仍通过既有 UI error surfaced；daily logs 可追溯根因与验证
- security / privacy: 不新增依赖、不改外部权限或网络访问

## Boundary / Invalid Input Cases
- 连续点击多个设置项时，不应并发发起多个 `update_settings`
- no-op 设置保存不应重复触发 host refresh
- 语言切换仍应触发 tray menu rebuild，避免菜单文案滞后

## Concurrency / Race Cases (if applicable)
- autosave in-flight 时继续修改 `form`
- 一轮保存回包时已有更新的 `formRevision`

## Mocks & Test Data
- `PauzaSettings::default()` 及其少量字段差异体，用于 Rust host refresh 纯函数测试

## Commands to Run
- `npm test`
- `npm run typecheck`
- `npm --prefix apps/desktop run build`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - JS tests 全部通过
  - Rust tests 增加到 `26 passed`
  - docs validator 通过
- Outputs to keep (10~20 lines snippet):
  - `cargo test` 中 `commands::tests::*` 的通过输出
  - `npm test` 的通过统计
  - workflow docs validator 的 `[OK]`
