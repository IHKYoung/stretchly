# Task-ID: TID-20260403-break-surface-mode-restore

## Test Strategy
- Unit:
  - 无新增独立 Rust/TS 单测；本轮以 host 编译、前端类型检查和 DOM 证据为主。
- Integration:
  - `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - `npm --prefix apps/desktop run typecheck`
  - `npm --prefix apps/desktop run build`
- E2E (if applicable):
  - 浏览器 preview + Playwright DOM snapshot，确认设置页入口恢复。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `cargo check` 确认 `PauzaSettings.fullscreen` 和 `shell.rs` 调用链完整。
- AC2 -> Playwright `settings-break-surface-snapshot-1440x810.md` 验证“提醒与打断”分类中存在 `显示方式`。
- AC3 -> 代码审查 `App.tsx` `saveSettings()` + `state.rs` / `shell.rs`，确认保存链和宿主消费链接通。
- AC4 -> `cargo check` + 代码审查 `shell.rs`，确认 immersive long break 在 window 模式下改为大窗口而非强制全屏。
- AC5 -> `cargo check`、`typecheck`、`build`、console log 共同证明功能恢复且无新前端错误。

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: DOM snapshot 证明 `显示方式` segmented control 已回到“提醒与打断”页。
- Fallback / secondary flow -> tests/evidence: 不保存时按钮 disabled，由 snapshot 可见。
- Visible states / transitions -> tests/evidence: `已同步`、`提醒与打断` active、`显示方式` tablist 均在 snapshot 中可见。
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
- Artifacts path: docs/specs/TID-20260403-break-surface-mode-restore/evidence/
- What to capture:
  - Screenshots: 本轮 screenshot 工具卡在 fonts wait，未产出稳定文件
  - Video/trace (optional): 无
  - HAR/console logs (optional): `browser-console.log`

## Quality Gates (Non-functional)
- a11y: segmented control 继续使用现有语义与 aria label。
- perf budget: 仅新增一个布尔字段和一个 UI control，不新增依赖。
- error handling / observability: 继续复用前台 error 条与 build/cargo 输出。
- security / privacy: 不新增权限、网络或数据路径。

## Boundary / Invalid Input Cases
- 旧 settings.json 缺少 `fullscreen` 字段时，serde 默认应回退到 `false`。
- locale 缺少 `window/fullscreen/breakDisplayMode` 键时，前端会直接显示 key，属于回归信号。

## Concurrency / Race Cases (if applicable)
- `dirty=true` 时 snapshot 轮询不应覆盖本地刚切换的 `fullscreen` 表单值。

## Mocks & Test Data
- 浏览器 preview 的 `defaultSettings().fullscreen=false` 作为稳定默认数据。

## Commands to Run
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- Playwright 打开 `http://127.0.0.1:43179/`，切换到“提醒与打断”，导出 DOM snapshot 与 console log

## Expected Results
- PASS criteria:
  - Rust host 编译通过。
  - 前端类型检查和构建通过。
  - DOM snapshot 可见 `显示方式` tablist。
  - console log 无新增前端错误。
- Outputs to keep (10~20 lines snippet):
  - `Finished dev profile [unoptimized + debuginfo] target(s) in 5.58s`
  - `✓ 1821 modules transformed.`
  - `✓ built in 1.30s`
