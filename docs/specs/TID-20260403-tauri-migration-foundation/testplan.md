# Task-ID: TID-20260403-tauri-migration-foundation

## Test Strategy
- Unit: 当前主要依赖 TypeScript typecheck 与 Rust `cargo check` 作为契约校验。
- Integration: 通过 `npm run build` 验证 React dashboard 与 Tauri bridge 的前端集成；通过 `npm run tauri:dev` 验证宿主层能实际启动。
- E2E (if applicable): 浏览器 preview 使用 Playwright 访问 `http://127.0.0.1:1420` 检查可视结构与无 runtime 崩溃 fallback。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `npm --prefix apps/desktop run typecheck` + `npm --prefix apps/desktop run build` + `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- AC2 -> `npm --prefix apps/desktop run tauri:dev` 启动到 `Running target/debug/pauza-desktop`，并由 `shell.rs` / `commands.rs` 代码审查覆盖 tray/shortcut/autostart 命令边界
- AC3 -> Playwright 访问 preview 页面，确认不再出现 `Cannot read properties of undefined (reading 'invoke')`
- AC4 -> `python3 scripts/validate_workflow_docs.py --mode manual`

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: `npm run tauri:dev` 启动主窗口；`docs/specs/TID-20260403-tauri-migration-foundation/evidence/README.md` 记录 dashboard 结构与原生命令入口。
- Fallback / secondary flow -> tests/evidence: Playwright 访问浏览器 preview，确认 CTA 和可视结构可用且无 runtime 崩溃。
- Visible states / transitions -> tests/evidence: 默认 empty state、preview CTA 反馈、focus session 倒计时与 footer 最近动作文案。

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
- Artifacts path: docs/specs/TID-20260403-tauri-migration-foundation/evidence/
- What to capture:
  - Screenshots: 浏览器 preview 的 dashboard 全页截图
  - Video/trace (optional): N/A
  - HAR/console logs (optional): `npm run tauri:dev` 启动日志关键片段

## Quality Gates (Non-functional)
- a11y: 键盘可操作 CTA，文字对比满足深色正文 / 暖白背景的基本可读性。
- perf budget: 不做正式预算，但要求首屏布局简单、无额外重量级 UI 框架。
- error handling / observability: 缺失 Tauri runtime 时自动 preview fallback；原生命令失败保留 error banner。
- security / privacy: 不新增外部服务，不引入额外数据采集。

## Boundary / Invalid Input Cases
- 在纯浏览器环境下调用页面，不得因为 `invoke` 缺失而崩溃。
- `start_focus_session` 的分钟数仅由前端固定按钮触发，避免无边界输入。
- 主窗口不存在时命令需要返回错误字符串而不是 panic。

## Concurrency / Race Cases (if applicable)
- tray、快捷键与 dashboard 都可能同时写入 `PauzaState`；本轮通过 `Mutex` 保证最小运行时安全。
- focus session 到期与手动清除同时发生时，以最新 snapshot 结果为准。

## Mocks & Test Data
- 浏览器 preview 的 `previewSnapshotBase` 作为本地 UI 模拟数据。
- Rust `DesktopSnapshot` 作为真实 runtime 返回结构。

## Commands to Run
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `python3 scripts/validate_workflow_docs.py --mode manual`
- `npm --prefix apps/desktop run tauri:dev`

## Expected Results
- PASS criteria:
  - TypeScript 构建通过
  - Rust host 编译检查通过
  - Tauri dev 能启动主程序
  - Workflow docs validator 通过
- Outputs to keep (10~20 lines snippet):
  - `Finished dev profile [unoptimized + debuginfo] target(s) in ...`
  - `Running target/debug/pauza-desktop`
