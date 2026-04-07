# Task-ID: TID-20260403-tauri-usable-core

## Test Strategy
- Unit: `cargo check` 覆盖 Rust host 模块接口连通性；`npm --prefix apps/desktop run typecheck` 覆盖前端类型与命令契约。
- Integration: `npm --prefix apps/desktop run build` 覆盖 React + Vite + Tauri command shape 的整体前端集成；`npm run desktop:dev` 覆盖实际宿主启动链路。
- E2E (if applicable): 使用 Playwright 对浏览器 preview 采集主设置页与 break prompt 的结构快照，作为本轮轻量交互证据。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`；源码审阅 `state.rs`、`engine.rs`、`platform.rs`
- AC2 -> `npm run desktop:dev` 启动到 `target/debug/pauza-desktop`；tray / shortcut / break window 由 `shell.rs` / `commands.rs` 对接
- AC3 -> `npm --prefix apps/desktop run typecheck`；`npm --prefix apps/desktop run build`；Playwright `settings-snapshot.md` 与 `break-snapshot.md`
- AC4 -> `python3 scripts/validate_workflow_docs.py --mode manual`；docs/specs/logs/plans/changelog 巡检

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: `settings-snapshot.md` 覆盖设置页结构；`npm run desktop:dev` 覆盖原生窗口可启动
- Fallback / secondary flow -> tests/evidence: `break-snapshot.md` 覆盖 prompt 结构；`browser-console.log` 覆盖 preview 无运行时崩溃
- Visible states / transitions -> tests/evidence: Save/Revert disabled 态、Preview 状态、break prompt 动作按钮由 Playwright snapshot 采样

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
- Artifacts path: docs/specs/TID-20260403-tauri-usable-core/evidence/
- What to capture:
  - Screenshots: 本轮改用结构快照 `settings-snapshot.md`、`break-snapshot.md`
  - Video/trace (optional): N/A
  - HAR/console logs (optional): `browser-console.log`

## Quality Gates (Non-functional)
- a11y: 表单控件均为原生输入；break prompt 保持少量高对比动作按钮
- perf budget: 主设置页单页 bundle 构建后约 `206 KB` JS / `4.7 KB` CSS，未引入额外重型前端框架
- error handling / observability: 所有 Tauri commands 返回 `Result<_, String>`；前台保留 error banner；运行时状态在 UI 中可见
- security / privacy: 仅使用本地设置文件、系统命令和进程列表；未引入外部服务或网络同步

## Boundary / Invalid Input Cases
- 关闭所有 break 类型后，状态应回到 idle 而不是继续调度。
- app exclusion 命令列表为空时，不应错误地进入 exclusion 阻塞态。
- preview 无 Tauri runtime 时，页面不能直接因 `invoke` 缺失崩溃。

## Concurrency / Race Cases (if applicable)
- 后台 engine tick 与前台 settings command 共享 `PauzaState`，依赖 `Arc<Mutex<RuntimeState>>` 串行化访问。
- break prompt 关闭请求会同步调用 `skip_current_break` 并隐藏窗口，避免同一时刻出现悬空 break 状态。

## Mocks & Test Data
- 浏览器 preview 使用本地 `previewSnapshot()` 作为静态状态源。
- app exclusion 示例输入：`zoom.us`、`meet.google.com`、`obs`。

## Commands to Run
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `npm run desktop:dev`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - Rust host 编译通过
  - 前端类型检查与构建通过
  - Tauri dev 实际运行到 `target/debug/pauza-desktop`
  - workflow docs validator 通过
  - 证据目录包含主设置页与 break prompt 快照
- Outputs to keep (10~20 lines snippet):
  - `Finished dev profile ... Running target/debug/pauza-desktop`
  - `vite ... ✓ built in ...`
