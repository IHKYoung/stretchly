# Task-ID: TID-20260424-break-ideas-asset-migration

## Test Strategy
- Unit:
  - `test/desktopBreakIdeas.js`
  - `test/desktopBreakCopySource.js`
  - `test/translations.js`
- Integration:
  - `python3 scripts/sync_desktop_break_ideas.py`
  - `python3 scripts/sync_desktop_locales.py`
  - `npm run typecheck`
  - `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- E2E (if applicable):
  - N/A

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `test/desktopBreakCopySource.js` + `python3 scripts/sync_desktop_locales.py`
- AC2 -> `test/desktopBreakIdeas.js` + `npm run typecheck`
- AC3 -> `test/desktopBreakCopySource.js` + `python3 scripts/sync_desktop_break_ideas.py`
- AC4 -> `python3 scripts/sync_desktop_break_ideas.py` + `python3 scripts/sync_desktop_locales.py` + `npm run typecheck` + `npm test` + `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` + `python3 scripts/validate_workflow_docs.py --mode manual`

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
- Artifacts path: docs/specs/TID-20260424-break-ideas-asset-migration/evidence/
- What to capture:
  - Screenshots:
  - Video/trace (optional):
  - HAR/console logs (optional):

## Quality Gates (Non-functional)
- a11y: N/A（无 UI 交互变更）
- perf budget: break prompt 继续走本地 JSON 内存查找，不引入运行时网络或异步 IO
- error handling / observability: sync scripts 必须在 source boundary 被破坏时直接失败
- security / privacy: 全部为本地静态文案资产，无新增隐私与外部权限面

## Boundary / Invalid Input Cases
- `messages/*.json` 残留 `miniBreakIdeas / longBreakIdeas` 时，`sync_desktop_locales.py` 必须失败
- `break-ideas/messages/*.json` 出现未知顶层 key 时，`sync_desktop_break_ideas.py` 必须失败
- `break-ideas/messages/*.json` 缺少 `miniBreakIdeas` 或 `longBreakIdeas` 时，`sync_desktop_break_ideas.py` 必须失败

## Concurrency / Race Cases (if applicable)
- N/A（静态资产迁移，无并发写入路径）

## Mocks & Test Data
- 真实的 50 语言 locale / break-idea JSON 资产
- `break-ideas/registry.json` 中的 `officialLanguages` 元数据

## Commands to Run
- `python3 scripts/sync_desktop_break_ideas.py`
- `python3 scripts/sync_desktop_locales.py`
- `npm run typecheck`
- `npm test`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
- Outputs to keep (10~20 lines snippet):
  - break ideas registry 生成成功，显示 50 languages / 50 bundles
  - locale registry 生成成功，且 message bundle 不再包含 ideas key
  - Vitest 全绿
  - Rust `cargo test` 继续保持 27 passed
