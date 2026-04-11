# Task-ID: TID-20260411-break-activation-settings-input-fix

## Test Strategy
- Unit:
  - 如有必要，补 `test/desktopSettingsControls.js` 的 helper 级回归
- Integration:
  - `npm test`
  - `npm --prefix apps/desktop run typecheck`
  - `npm --prefix apps/desktop run build`
  - `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - `python3 scripts/validate_workflow_docs.py --mode manual`
- E2E (if applicable):
  - 本轮不新增完整 native E2E；macOS fullscreen Space 以代码路径审查 + 本机现实 spot-check 缺口说明补足

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 审查 `apps/desktop/src-tauri/src/shell.rs` 的 show/present/activate 路径；`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` 与 `npm --prefix apps/desktop run build`
- AC2 -> 审查 `apps/desktop/src/App.tsx` 的 `CompactNumber` 草稿输入逻辑；`npm test`、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`
- AC3 -> 代码审查 `shell.rs` 现有 tray refresh workaround 与 break close/fullscreen helper 未被回退；Rust/前端构建通过
- AC4 -> `npm test`、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`、`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`python3 scripts/validate_workflow_docs.py --mode manual`

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->

### Primary flow -> tests/evidence
`shell.rs` 的 macOS break show/present/activate 路径代码审查、`cargo test` 与前端 build 通过；evidence 中显式记录仍需本机全屏 Space spot-check

### Fallback / secondary flow -> tests/evidence
`CompactNumber` 的草稿输入/blur/Enter/step button 代码路径审查，配合 `npm test`、`typecheck`、`build`

### Visible states / transitions -> tests/evidence
数字输入 `empty/partial draft -> blur|Enter commit -> Escape restore`，以及 `due -> break window visible on current Space` 的代码路径与验证输出

### Validator Expectation
当 `interaction_impact != none` 时，本节三项与 Evidence Capture 的 `Required` 不得继续保留 `N/A/no/TBD`

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
- Artifacts path: docs/specs/TID-20260411-break-activation-settings-input-fix/evidence/
- What to capture:
  - Screenshots: 如可行，设置页数字输入交互截图；否则以 build/test 输出与代码路径说明替代
  - Video/trace (optional): N/A
  - HAR/console logs (optional): build/test/validator 关键输出

## Quality Gates (Non-functional)
- a11y:
  - `CompactNumber` 继续支持键盘 Enter / Escape
- perf budget:
  - 数字输入不再每个击键都触发 autosave 链路
- error handling / observability:
  - 继续沿用现有 save/load error、`last_action` 与 tray status
- security / privacy:
  - 无新增权限、网络请求或外部数据流

## Boundary / Invalid Input Cases
- 数字输入可暂时为空字符串
- 仅允许数字字符草稿
- blur / Enter 时仍需按 `min/max` clamp
- `Escape` 恢复当前持久化值

## Concurrency / Race Cases (if applicable)
- 输入框 blur 与加减按钮点击不应互相覆盖
- 现有 `shell.rs` tray 刷新 workaround 不应因 break 激活补丁回退

## Mocks & Test Data
- 继续复用仓库现有 Vitest/Rust 测试基座
- 不新增 host mock 或 fixture

## Commands to Run
- `npm test`
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - 现有前端与 Rust 测试全部通过
  - `typecheck` 无 TS 错误
  - `build` 成功，允许保留既有 chunk size warning
  - docs validator 通过
- Outputs to keep (10~20 lines snippet):
  - `Test Files ... passed`
  - `cargo test ... ok`
  - `✓ built in ...`
  - `[OK] Workflow docs validation passed ...`
