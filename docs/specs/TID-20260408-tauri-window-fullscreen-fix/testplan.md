# Task-ID: TID-20260408-tauri-window-fullscreen-fix

## Test Strategy
- Unit: 无；本轮宿主窗口修复未新增独立 Rust 单测。
- Integration:
  - `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - `npm --prefix apps/desktop run build`
- E2E (if applicable):
  - macOS 本机手动触发一次 windowed break，确认窗口居中且约占工作区 `80% x 80%`
  - macOS 本机手动触发一次 fullscreen break，确认顶部无空白且关闭时正常退出

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 审查 `apps/desktop/src-tauri/tauri.conf.json`，确认默认/最小尺寸为 `960x640` / `800x600`
- AC2 -> 审查 `apps/desktop/src-tauri/src/shell.rs`，确认 windowed break profile 统一改为居中且约占工作区 `80% x 80%`
- AC3 -> 审查 `apps/desktop/src-tauri/src/shell.rs`，确认 macOS fullscreen 改走 `set_simple_fullscreen()`
- AC4 -> 审查 `close_break_window()` 与 helper，确认退出 simple/native fullscreen 后再 hide + destroy
- AC5 -> 运行 `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` 和 `npm --prefix apps/desktop run build`

## Interaction Contract Coverage
- Interaction impact: direct
- Primary flow -> tests/evidence: 通过 `shell.rs` profile 路径审查 + 用户本机 windowed break 手测，确认 break 居中且不再缩成右下角小浮窗。
- Fallback / secondary flow -> tests/evidence: 通过 fullscreen helper 路径审查 + 用户本机 fullscreen break 手测，确认顶部无空白；通过 `tauri.conf.json` 代码审查，确认主窗口 resize 受 `800x600` 下限约束。
- Visible states / transitions -> tests/evidence: 通过 close path 代码审查，确认 fullscreen visible -> exit fullscreen -> destroy 的顺序固定。

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
- Required: partial
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260408-tauri-window-fullscreen-fix/evidence/
- What to capture:
  - Screenshots: windowed break 居中且约占工作区 `80% x 80%` 的手工截图（待用户本机）
  - Screenshots: macOS fullscreen break 顶部无空白的手工截图（待用户本机）
  - Video/trace (optional): fullscreen break 进入/退出录屏（可选）
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y: 不改键盘交互与文案
- perf budget: 不引入新依赖，不新增长期后台任务
- error handling / observability: fullscreen 逻辑集中到单一 helper，便于后续继续加日志
- security / privacy: 不新增权限和外部连接

## Boundary / Invalid Input Cases
- windowed break 不应再因 `BreakKind::Microbreak` 走右下角角落卡片分支。
- 非 macOS 平台仍应继续走 `set_fullscreen()`，不能被 macOS 分支影响。
- 关闭 break window 时不能只依赖 `is_fullscreen()`，否则 simple fullscreen 会漏退。

## Concurrency / Race Cases (if applicable)
- break CTA 仍沿用现有 deferred close 路径，本轮不改变其异步 teardown 时序。

## Mocks & Test Data
- 无额外 mock；依赖现有 Tauri host 与前端构建链。

## Commands to Run
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - Rust host profile 逻辑已统一，windowed break 居中且尺寸接近工作区 `80% x 80%`
  - Rust host 能编译通过，`set_simple_fullscreen()` 调用合法
  - 前端 build 通过
  - specs / logs / plans 不保留 `TBD/INIT`
- Outputs to keep (10~20 lines snippet):
  - `Finished 'dev' profile [unoptimized + debuginfo] target(s) ...`
  - `✓ built in ...`
