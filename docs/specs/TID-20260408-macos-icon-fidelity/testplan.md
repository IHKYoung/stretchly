# Task-ID: TID-20260408-macos-icon-fidelity

## Test Strategy
- Unit: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- Integration: `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`、`npm --prefix apps/desktop run build`
- E2E (if applicable): `python3 graphics/generate_icon_assets.py` 验证资源链能跑通；系统级截图因权限不足未执行

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 阅读 `shell.rs` 实现 + `cargo check`，确认 tray patch 通过 `with_inner_tray_icon()` 只操作 Pauza 自己的 `NSStatusItem`
- AC2 -> 阅读 `shell.rs` 实现 + `cargo check`，确认 Dock 使用 `icon.icns`
- AC3 -> `python3 graphics/generate_icon_assets.py`
- AC4 -> `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`npm --prefix apps/desktop run build`

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
- Artifacts path: docs/specs/TID-20260408-macos-icon-fidelity/evidence/
- What to capture:
  - Screenshots: 本轮未采集；Screen Recording 未授权
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y: tray icon 继续走 macOS template image 着色
- perf budget: 仅图标资源与原生 patch，无额外性能预算风险
- error handling / observability: 图标脚本需完整跑通；Rust / 前端构建不报错
- security / privacy: 不新增权限；仅视觉截图验证受 Screen Recording 权限限制

## Boundary / Invalid Input Cases
- 图标生成脚本在目标文件与输出文件同路径时不应再因 ffmpeg 原地覆盖失败
- `ensure_srgb()` 必须同时兼容单参数原地修正和带输出路径的复制修正

## Concurrency / Race Cases (if applicable)
- 通过 `with_inner_tray_icon()` 在主线程直接拿原生 tray handle，避免旧实现里异步延迟后枚举 status items 的竞态

## Mocks & Test Data
- 使用仓库内现有 `graphics/app-icon.svg`、`graphics/tray-icon.svg` 作为源数据

## Commands to Run
- `python3 graphics/generate_icon_assets.py`
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - 图标生成脚本输出 `generated icon assets`
  - Rust check/test 通过
  - 前端生产构建通过
  - workflow docs gate 通过
- Outputs to keep (10~20 lines snippet):
  - `Finished 'dev' profile [unoptimized + debuginfo]`
  - `3 passed; 0 failed`
  - `✓ 1820 modules transformed.`
  - `generated icon assets`
