# Task-ID: TID-20260403-tauri-i18n-resource-layer

## Test Strategy
- Unit:
  - 前端 `tsc --noEmit` 检查 locale lookup 与 `App.tsx` 类型边界
- Integration:
  - `npm --prefix apps/desktop run build`
  - `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- E2E (if applicable):
  - 不新增；沿用正在运行的 `desktop:dev` 做人工热更新观察

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`
- AC2 -> 检查 `defaultSettings.language = zh-CN` 与前端 locale 取词
- AC3 -> `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`，确保 Rust host 文案调用点全部编译通过

## Interaction Contract Coverage
- Interaction impact: none
- Primary flow -> tests/evidence: N/A
- Fallback / secondary flow -> tests/evidence: N/A
- Visible states / transitions -> tests/evidence: N/A

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
- Required: no
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260403-tauri-i18n-resource-layer/evidence/
- What to capture:
  - Screenshots: 本轮不新增
  - Video/trace (optional): 无
  - HAR/console logs (optional): 无

## Quality Gates (Non-functional)
- a11y: 继续使用原生 select / input，保持键盘可达
- perf budget: locale lookup 为本地 JSON，不能影响启动体验
- error handling / observability: key 缺失时回退到 key 本身
- security / privacy: locale 仅包含静态文本

## Boundary / Invalid Input Cases
- `language` 字段缺失或为未知值时，sanitize 必须回退到 `zh-CN`
- locale key 缺失时，lookup 不能导致前端或 Rust 崩溃

## Concurrency / Race Cases (if applicable)
- Rust `OnceLock` 初始化 bundle 时不应影响 engine 线程稳定性

## Mocks & Test Data
- 使用 `src/locales/{zh-CN,en}.json` 作为唯一测试数据源

## Commands to Run
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - 前端 typecheck/build 通过
  - Rust host 编译通过
  - workflow docs 校验通过
- Outputs to keep (10~20 lines snippet):
  - `Finished dev profile` from `cargo check`
  - `✓ built in ...` from Vite build
