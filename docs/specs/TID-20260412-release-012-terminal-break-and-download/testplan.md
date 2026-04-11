# Task-ID: TID-20260412-release-012-terminal-break-and-download

## Test Strategy
- Unit: 复用 `test/desktopBreakIdeas.js`，验证 microbreak 单条、long break 轮播与稳定排序逻辑。
- Integration: 通过 `npm --prefix apps/desktop run typecheck` 验证 break 页组件与样式引用没有类型回归；通过 `npm test -- test/desktopBreakCopyLayout.js test/desktopBreakIdeas.js` 验证 break 文案布局/轮播逻辑。
- E2E (if applicable): 通过浏览器 preview `?window=break` 做人工现实检查，确认 `Pauza>` prompt、逐字打字和居中布局可见；官网下载入口以源码检查和 GitHub latest release 状态核对为主，不额外跑浏览器自动化。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `npm test -- test/desktopBreakCopyLayout.js test/desktopBreakIdeas.js` + `?window=break` 人工预览
- AC2 -> `git diff --cached --name-only | rg '^apps/site/'` + 检查 `apps/site/index.html`、`apps/site/script.js`、`apps/site/download/targets.js`
- AC3 -> `node -p "require('./package.json').version"`、`node -p "require('./apps/desktop/package.json').version"`、`rg -n '^version = \"0\\.1\\.2\"$' apps/desktop/src-tauri/Cargo.toml`、`rg -n '\"version\"\\s*:\\s*\"0\\.1\\.2\"' apps/desktop/src-tauri/tauri.conf.json`
- AC4 -> `python3 scripts/validate_workflow_docs.py --mode manual` + `git commit` + `git log -1 --stat`

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
- Artifacts path: docs/specs/TID-20260412-release-012-terminal-break-and-download/evidence/
- What to capture:
  - Screenshots: N/A（本任务沿用人工预览结论，不单独新增 evidence 资产）
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y: `prefers-reduced-motion` 下 prompt 必须直接完整显示，不强行动画
- perf budget: 不新增依赖；官网下载解析只在按钮点击时发生
- error handling / observability: GitHub API 失败必须无噪声回退到 pinned 下载链接；workflow docs validator 必须通过
- security / privacy: 不新增外部服务、密钥和用户数据上报

## Boundary / Invalid Input Cases
- 若 GitHub `latest release` 响应里不存在 `_aarch64.dmg` 资产，官网必须使用 `fallbackUrl`
- 若 prompt 池为空，break 页必须保留兜底文案而不是空白
- 若 staged files 混入 `src-tauri/src/{commands,state}.rs` 等无关路径，视为提交边界失败

## Concurrency / Race Cases (if applicable)
- break preview 的 `startedAtMs` 必须稳定，避免 snapshot 轮询导致每次重新换 prompt
- 长休息轮播计时不能在打字尚未完成时提前切句

## Mocks & Test Data
- 使用现有 `miniBreakIdeas` / `longBreakIdeas` 文案真源与浏览器 preview snapshot
- 使用当前 GitHub release 实际发布状态作为官网回退策略判断依据

## Commands to Run
- `git diff --check`
- `npm --prefix apps/desktop run typecheck`
- `npm test -- test/desktopBreakCopyLayout.js test/desktopBreakIdeas.js`
- `node -p "require('./package.json').version"`
- `node -p "require('./apps/desktop/package.json').version"`
- `rg -n '^version = \"0\\.1\\.2\"$' apps/desktop/src-tauri/Cargo.toml`
- `rg -n '\"version\"\\s*:\\s*\"0\\.1\\.2\"' apps/desktop/src-tauri/tauri.conf.json`
- `python3 scripts/validate_workflow_docs.py --mode manual`
- `git log -1 --stat`

## Expected Results
- PASS criteria:
-  `npm --prefix apps/desktop run typecheck` 通过
-  `npm test -- test/desktopBreakCopyLayout.js test/desktopBreakIdeas.js` 通过
-  根、desktop、Tauri/Cargo 版本号都输出 `0.1.2`
-  workflow docs validator 通过，最终形成一条详细中文本地 commit
- Outputs to keep (10~20 lines snippet):
-  `vitest` 的通过统计
-  `validate_workflow_docs.py` 的 `[OK]`
