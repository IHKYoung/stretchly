# Task-ID: TID-20260727-settings-simplification-015

## Test Strategy
- Unit: profile 匹配/patch/小时周期、导航源码契约、提示语 hold 常量与执行顺序、locale 术语、Rust 默认调度。
- Integration: locale/break-ideas registry 生成、TypeScript typecheck、Vite production build、Rust 全量单测、版本一致性与 workflow validators。
- E2E (if applicable): 本地 Dev URL HTTP 200；浏览器后端不可用，截图/点击与真实 60 秒等待未自动执行。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `desktopSettingsNavigation.js` + TypeScript build。
- AC2 -> `desktopSettingsControls.js` + Rust default/schedule tests。
- AC3 -> `desktopBreakIdeas.js`、`desktopBreakCopySource.js`、`translations.js`。
- AC4 -> Node JSON version check、Cargo `rg`、README/CHANGELOG review。
- AC5 -> generators、`npm test`、typecheck、Cargo tests、frontend build、workflow validators、`git diff --check`。

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: overview 包含五个直接 route，back 恒为 overview 的源码回归测试。
- Fallback / secondary flow -> tests/evidence: custom profile、disabled/nonmatching profile、prompt fallback 与 reduced-motion 代码路径 review。
- Visible states / transitions -> tests/evidence: typing body -> 60s hold -> 520ms gap 顺序测试；截图因 browser backend unavailable 缺失。

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
- 仅改 profile 常量而未同步 Rust/preview 默认值，导致新用户显示 custom。
- 60 秒从 break 开始而不是从完整文案呈现后开始。
- staging 混入 `apps/site/**`、`.gitignore` 或 `scratch/**`。

## Evidence Capture (UI / E2E)
- Required: partial   <!-- yes | no | partial -->
- Owner: orchestrator
- Artifacts path: docs/specs/TID-20260727-settings-simplification-015/evidence/
- What to capture:
  - Screenshots: 未采集；browser discovery 返回空后端列表。
  - Video/trace (optional): 未采集；60 秒时序由常量与源码顺序测试覆盖。
  - HAR/console logs (optional): 本地 URL `HTTP 200`；无远端请求。

## Quality Gates (Non-functional)
- a11y: radiogroup label、原生 button、focus ring 与返回 aria-label 保持。
- perf budget: 无新依赖/资源；Vite 仅保留既有 large-chunk warning。
- error handling / observability: load/autosave error 显式；timer cleanup 在 effect teardown 中执行。
- security / privacy: 无新网络、权限、凭据或数据采集。

## Boundary / Invalid Input Cases
- profile 值不匹配、微休息/完整休息关闭、break ideas 为空、reduced motion、manual awaiting、locale fallback。

## Concurrency / Race Cases (if applicable)
- route/typewriter effect 重建时清理 timeout；autosave 保持既有串行/合并行为，未引入并发保存。

## Mocks & Test Data
- 使用 preview snapshot、固定 break start timestamp、真实 locale registry 与 Rust `PauzaSettings::default()`。

## Commands to Run
- `python3 scripts/sync_desktop_locales.py`
- `python3 scripts/sync_desktop_break_ideas.py`
- `npm test`
- `npm run typecheck`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_agent_configs.py`
- `python3 scripts/validate_workflow_kit_sync.py --repo-root .`
- `python3 scripts/validate_workflow_docs.py --mode manual`
- `git diff --check` / `git diff --cached --check`

## Expected Results
- PASS criteria: 版本统一 0.1.5；122 frontend / 29 Rust tests PASS；typecheck/build/generators/workflow PASS；staged paths 仅包含任务范围。
- Outputs to keep (10~20 lines snippet): 测试计数、build 完成、generator 50 languages、workflow validator 与版本一致性输出。
