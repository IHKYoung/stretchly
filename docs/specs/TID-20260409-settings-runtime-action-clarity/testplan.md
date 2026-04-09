# Task-ID: TID-20260409-settings-runtime-action-clarity

## Test Strategy
- Unit: N/A（本轮不新增独立纯函数测试）
- Integration: 代码审查 `App.tsx` 的暂停态 / focus 态 / 默认态显隐分支，以及 preview fallback 的 paused/focus 模拟路径（如实现）
- E2E (if applicable): 浏览器 preview + Playwright 截图 / snapshot 验证

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 代码审查 `App.tsx` + paused preview screenshot，确认顶部只在 pause active 时显示 `恢复提醒`
- AC2 -> default schedule screenshot + 代码审查，确认 `重置节奏` 位于独立节奏控制区，且说明文案明确
- AC3 -> focus preview screenshot 或代码审查，确认 focus 态不再复用 `恢复提醒`
- AC4 -> Evidence README + 截图 / snapshot 对照 primary / fallback flow
- AC5 -> `python3 scripts/sync_desktop_locales.py`、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`、`python3 scripts/validate_workflow_docs.py --mode manual`

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->

### Primary flow -> tests/evidence
- paused preview 看到顶部 `恢复提醒`；点击后回到普通预览态（如实现 preview mock），或以代码审查确认显隐逻辑。

### Fallback / secondary flow -> tests/evidence
- focus preview 看到 `结束专注`；默认态不出现 `恢复提醒`。

### Visible states / transitions -> tests/evidence
- schedule page 中 `重置节奏` 独立成组，并带“不自动解除暂停”的文案。

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
- Owner: orchestrator（single-agent-fallback）
- Artifacts path: docs/specs/TID-20260409-settings-runtime-action-clarity/evidence/
- What to capture:
  - Screenshots: default schedule state、paused state（若可 mock）
  - Video/trace (optional): N/A
  - HAR/console logs (optional): 浏览器 console log（若使用 Playwright）

## Quality Gates (Non-functional)
- a11y: 新按钮可键盘聚焦，沿用 `Button` 组件默认 focus ring
- perf budget: 不新增重量级依赖或额外轮询
- error handling / observability: 保持 `busyAction` / `error` 现有行为不退化
- security / privacy: preview mock 仅使用 URL 查询参数，不读取外部资源或本地敏感数据

## Boundary / Invalid Input Cases
- 默认运行态不应显示 `恢复提醒`
- pause 与 focus 同时存在时，应优先显示 `恢复提醒`，避免双义入口
- `重置节奏` 在强制休息锁定期间不应制造假成功反馈

## Concurrency / Race Cases (if applicable)
- 运行时动作与自动保存共享 `busyAction`，需要证明按钮在保存中不可重复触发

## Mocks & Test Data
- 如实现 preview mock：`/?preview=paused`、`/?preview=focus`

## Commands to Run
- `python3 scripts/sync_desktop_locales.py`
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - 设置页动作语义与用户心智对齐，且构建/文档校验通过
- Outputs to keep (10~20 lines snippet):
  - typecheck/build/validator 的 PASS 摘要
