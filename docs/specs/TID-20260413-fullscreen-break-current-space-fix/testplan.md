# Task-ID: TID-20260413-fullscreen-break-current-space-fix

## Test Strategy
- Unit:
  - 更新 `shell.rs` 的 macOS overlay policy 单测，确保 collection behavior 断言与当前修复一致
- Integration:
  - `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - `npm --prefix apps/desktop run build`
- E2E (if applicable):
  - 本轮不做自动化 fullscreen Space E2E；通过 evidence README 保留本机手动 spot-check 缺口

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> Rust 单测断言 macOS break window overlay policy 包含 `CanJoinAllSpaces | MoveToActiveSpace | FullScreenAuxiliary`
- AC2 -> 代码审查 `show_break_window()` / `configure_break_window_native_behavior()` / `activate_break_application()`，并通过 `cargo test` + desktop build 防回归
- AC3 -> `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`npm --prefix apps/desktop run build`、`python3 scripts/validate_workflow_docs.py --mode manual`
- AC4 -> `docs/specs/TID-20260413-fullscreen-break-current-space-fix/evidence/README.md`

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: Rust 代码路径校验 `shell.rs` native overlay policy + evidence README 中的 fullscreen current-Space 预期与手动 spot-check 缺口说明
- Fallback / secondary flow -> tests/evidence: `show_break_window()` 现有 `present + focus/activate` 链路保持不变，通过 `cargo test` / build 证明无结构性回退
- Visible states / transitions -> tests/evidence: due -> native patch -> current Space visible overlay -> close/finish 路径，分别由代码审查、单测和 evidence README 对照说明

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
- Artifacts path: docs/specs/TID-20260413-fullscreen-break-current-space-fix/evidence/
- What to capture:
  - Screenshots: N/A（当前终端环境无法自动进入系统 fullscreen Space）
  - Video/trace (optional): N/A
  - HAR/console logs (optional): 保留验证命令结果摘要与必要的关键输出片段

## Quality Gates (Non-functional)
- a11y:
  - 不改前端交互控件与可访问性语义
- perf budget:
  - 不新增后台轮询、线程或大对象分配
- error handling / observability:
  - 保持现有 `Result<(), String>` 错误边界，不引入静默失败分支
- security / privacy:
  - 不新增权限、外部服务或敏感数据流

## Boundary / Invalid Input Cases
- `show_breaks_on_all_screens = true` 与 `target_screen = primary/cursor` 的 monitor 选择逻辑不应被本轮修复改坏
- fullscreen / windowed break 共用同一 native behavior patch，不应只修 fullscreen 而回退 windowed

## Concurrency / Race Cases (if applicable)
- 继续保留“先 native patch 再 `show()`”的顺序，避免 tao 异步层级设置把 level 冲掉
- 不改 `run_on_main_thread()` 与 break close deferred teardown 的线程模型

## Mocks & Test Data
- 使用 `shell.rs` 现有 `BreakWindowProfile` 常量断言；无需外部 mock 数据

## Commands to Run
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - Rust 测试全部通过，含 macOS overlay policy 断言
  - desktop build 通过
  - workflow docs validator 通过
- Outputs to keep (10~20 lines snippet):
  - `cargo test` 末尾 PASS 统计
  - desktop build 成功摘要
  - docs validator 成功摘要
