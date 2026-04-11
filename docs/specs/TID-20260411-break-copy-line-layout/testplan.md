# Task-ID: TID-20260411-break-copy-line-layout

## Test Strategy
- Unit: `test/desktopBreakCopyLayout.js` 覆盖中文长句分行、英文多句一行一句、英文长句按分句换行
- Integration: `npm test`
- E2E (if applicable): `npm --prefix apps/desktop run build` + browser preview 截图验证 break prompt 实际排版

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `desktopBreakCopyLayout.js` 中文长句分行用例 + `evidence/break-copy-preview.png`
- AC2 -> `desktopBreakCopyLayout.js` 英文多句 / 长句分行用例
- AC3 -> `desktopBreakCopyLayout.js`
- AC4 -> `npm test`、`npm --prefix apps/desktop run build`、`python3 scripts/validate_workflow_docs.py --mode manual`

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->

### Primary flow -> tests/evidence
- browser preview `?window=break` 截图 `evidence/break-copy-preview.png` 展示中央文案按句分行

### Fallback / secondary flow -> tests/evidence
- `desktopBreakCopyLayout.js` 覆盖英文多句一行一句与超长句按分句换行

### Visible states / transitions -> tests/evidence
- `App.tsx` 代码审查确认 `title/body` 都改为按 line list 渲染，且标题 4 行以上自动收紧字号

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
- Artifacts path: docs/specs/TID-20260411-break-copy-line-layout/evidence/
- What to capture:
  - Screenshots: `break-copy-preview.png`
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y: 不改变 focus / CTA 顺序，只改阅读节奏
- perf budget: 仅新增轻量字符串 helper，不引入新 effect 或重计算热点
- error handling / observability: helper 对空字符串安全返回空数组
- security / privacy: 无新增外部输入或权限

## Boundary / Invalid Input Cases
- 空文案时不渲染空行
- 多句英文保留一句一行
- 超长中文句允许退化为按逗号/分句换行，而不是任意字位断开

## Concurrency / Race Cases (if applicable)
- 不适用；纯前台同步 helper

## Mocks & Test Data
- 使用固定中英文 prompt 字符串，无需 mocks

## Commands to Run
- `npm test`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
- 5 个 test files / 62 个 tests 全部通过
- desktop build 成功
- workflow docs validator 通过
- Outputs to keep (10~20 lines snippet):
- `✓ test/desktopBreakCopyLayout.js (3 tests)`
- `Test Files 5 passed`
- `✓ built in`
- `[OK] Workflow docs validation passed`
