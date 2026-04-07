# Task-ID: TID-20260404-compact-settings-ui

## Test Strategy
- Unit: 依赖 TypeScript 类型检查覆盖组件接口和页面结构变更。
- Integration: 通过 `vite build` 验证 `App.tsx`、基础 UI 组件与 `tauri.conf.json` 协同收敛后仍可构建。
- E2E (if applicable): 浏览器预览快照验证主设置页结构与主要可见状态。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 浏览器快照 `settings-compact-1040x585-pass4.md`，确认窄侧栏 + 单主面板成立且无第三栏/大卡片壳。
- AC2 -> 代码审阅 `button/select/segmented-control/switch/textarea/NumberControl` 样式改动 + `npm --prefix apps/desktop run build`。
- AC3 -> `apps/desktop/src-tauri/tauri.conf.json` 尺寸改动 + 浏览器 `1040x585` 快照 + typecheck/build。
- AC4 -> 自动保存逻辑代码审阅 + `npm --prefix apps/desktop run typecheck`
- AC5 -> `docs/specs/TID-20260404-compact-settings-ui/evidence/README.md` 与控制台日志。

## Interaction Contract Coverage
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: `settings-compact-1040x585-pass4.md` 展示左侧分类导航、自动保存后的轻量顶部状态与当前分类内容。
- Fallback / secondary flow -> tests/evidence: `browser_resize(1040x585)` 和更小主窗口配置 `960x540`；本轮没有单独录制 `960x540` 快照，但布局规则已在实现与 UI spec 中固定。
- Visible states / transitions -> tests/evidence: `saving` / `pendingChanges` 顶部状态由代码路径覆盖；错误态保留在代码路径内，本轮未主动制造。
- Validator expectation: 当 `interaction_impact != none` 时，本节三项与 Evidence Capture 的 `Required` 不得继续保留 `N/A/no/TBD`

### Primary flow -> tests/evidence
- `docs/specs/TID-20260404-compact-settings-ui/evidence/settings-compact-1040x585-pass4.md`

### Fallback / secondary flow -> tests/evidence
- `docs/specs/TID-20260404-compact-settings-ui/evidence/settings-compact-1040x585-pass4.md`
- `docs/specs/TID-20260404-compact-settings-ui/ui.md` 的 Responsive Rules

### Visible states / transitions -> tests/evidence
- `docs/specs/TID-20260404-compact-settings-ui/evidence/settings-compact-1040x585-pass4.md`
- 自动保存后的轻量顶部状态

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
- Required: yes   <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260404-compact-settings-ui/evidence/
- What to capture:
- Screenshots: 浏览器结构快照 `settings-compact-1200x675.md`、`settings-compact-1120x630.md`、`settings-compact-1040x585.md`、`settings-compact-1040x585-pass3.md`、`settings-compact-1040x585-pass4.md`
  - Video/trace (optional): N/A
  - HAR/console logs (optional): `browser-console.log`、`browser-console-compact-pass2.log`、`browser-console-compact-pass3.log`、`browser-console-compact-pass4.log`

## Quality Gates (Non-functional)
- a11y: 保留控件 aria label 与 focus ring。
- perf budget: 不新增运行时依赖。
- error handling / observability: 保留顶部错误条；控制台日志无错误。
- security / privacy: 不新增权限或网络请求。

## Boundary / Invalid Input Cases
- 窗口收窄后设置行允许换行，避免控件溢出。
- 自动保存启用后，无脏数据时顶部不应再出现显式保存按钮。

## Concurrency / Race Cases (if applicable)
- N/A：本轮只改静态布局与窗口配置。

## Mocks & Test Data
- 浏览器预览使用 `previewSnapshot()` 的默认中文设置集。

## Commands to Run
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - TypeScript 检查通过。
  - Vite 构建通过。
  - Workflow docs 校验通过。
  - 浏览器快照能看到窄侧栏、单主面板、轻量同步状态与当前分类结构。
- Outputs to keep (10~20 lines snippet):
  - `vite v7.3.1 building client environment for production...`
  - `✓ built in ...`
  - `[OK] Workflow docs validation passed for 2026-04-04 ✅`
