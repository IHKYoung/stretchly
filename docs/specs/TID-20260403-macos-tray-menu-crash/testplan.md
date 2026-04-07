# Task-ID: TID-20260403-macos-tray-menu-crash

## Test Strategy
- Unit: 本轮不新增单元测试；tray root menu 依赖 Tauri/muda 原生对象，优先做编译 + 真机 runtime 验证
- Integration: `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- E2E (if applicable): `npm --prefix apps/desktop run tauri dev` 启动真实 tray runtime 后，由用户手工右键验证；当前终端因缺少 assistive access 无法替代点击

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `tauri dev` 启动真实 runtime + 用户手工右键 tray；证据记录于 `evidence/README.md`
- AC2 -> 代码审查 `handle_tray_action` / tray click handler 未改 action ids 或左键 reveal 逻辑；`cargo check` 确认平台分支编译闭环
- AC3 -> 代码审查 `engine.rs` 已改为 `refresh_tray_if_needed()`，不再每秒强制 `refresh_tray()`
- AC4 -> `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` PASS；`npm --prefix apps/desktop run tauri dev` 运行中且 `pgrep` 可见 `target/debug/pauza-desktop`

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->
### Primary flow -> tests/evidence
- 通过 `npm --prefix apps/desktop run tauri dev` 启动真实 runtime，并由用户在当前运行中的 app 上执行 macOS tray 右键
- `evidence/README.md` 已记录进程运行态与 UI automation 权限阻塞，作为本轮现实证据

### Fallback / secondary flow -> tests/evidence
- 代码审查确认左键 tray 仍走 `reveal_main_window`
- 用户在手工右键复核时可顺带确认左键仍只显示主窗口

### Visible states / transitions -> tests/evidence
- `status` / `status-detail` 禁用项、strict mode 锁定分支和 `Quit` 保底项均在 `populate_tray_menu` 保留
- 后台 tick 已从秒级无条件 `refresh_tray()` 改为 `refresh_tray_if_needed()`，避免菜单展开时被整棵替换
- 运行时证据见 `evidence/README.md`

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
- Artifacts path: docs/specs/TID-20260403-macos-tray-menu-crash/evidence/
- What to capture:
  - Screenshots: 不适用；当前终端缺少菜单栏 UI automation 权限
  - Video/trace (optional): 不适用
  - HAR/console logs (optional): `pgrep` 运行态、`osascript` 权限阻塞、`cargo check` / `tauri dev` 结果摘要

## Quality Gates (Non-functional)
- a11y: 保持 macOS 原生菜单语义，不引入自绘或键盘导航回退
- perf budget: 不新增后台轮询或重型对象，仅更换 macOS 根菜单类型
- error handling / observability: 编译通过、runtime 能启动，并在 docs 中记录 UI automation 阻塞
- security / privacy: 不新增权限、不读取新数据、不引入网络请求

## Boundary / Invalid Input Cases
- strict mode 锁定时仍应只保留允许展示的状态与退出项
- 自动启动开关文案仍跟随当前 enabled 状态切换
- 即使没有 microbreak/long break，对应 submenu 仍按既有条件显示/隐藏

## Concurrency / Race Cases (if applicable)
- tray action 在线程中执行后会触发 `refresh_tray`；本轮未改并发模型，只验证菜单对象刷新仍可编译并运行

## Mocks & Test Data
- 无；复用运行中 `PauzaState` 与真实 tray runtime

## Commands to Run
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm --prefix apps/desktop run tauri dev`
- `pgrep -fl 'target/debug/pauza-desktop|node .*tauri dev|vite --host 127.0.0.1 --port 43179'`
- `rg -n \"refresh_tray_if_needed|refresh_tray\\(\" apps/desktop/src-tauri/src/{engine,shell}.rs`
- `osascript -e 'tell application "System Events" to tell process "pauza-desktop" to get count of menu bars'`（当前预期因 assistive access 缺失失败）

## Expected Results
- PASS criteria:
  - Rust host 编译通过
  - `tauri dev` 能拉起 `target/debug/pauza-desktop`
  - 后台 tick 不再每秒重建 tray 菜单
  - 用户右键 tray icon 后不再闪退
- Outputs to keep (10~20 lines snippet):
  - `Finished 'dev' profile ...`
  - `Running 'target/debug/pauza-desktop'`
  - `5230 node ... tauri dev`
  - `5523 node ... vite --host 127.0.0.1 --port 43179`
  - `5608 target/debug/pauza-desktop`
