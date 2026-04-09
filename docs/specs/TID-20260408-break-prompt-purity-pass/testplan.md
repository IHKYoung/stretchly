# Task-ID: TID-20260408-break-prompt-purity-pass

## Test Strategy
- Unit: 不单独新增单元测试；通过 TypeScript 编译保证 `tList()` 与 break prompt 相关类型契约成立
- Integration: 运行 `npm --prefix apps/desktop run typecheck` 与 `npm --prefix apps/desktop run build`
- E2E (if applicable): 以浏览器/Tauri preview 截图验证 break prompt 单列布局，manual-awaiting 与 prompt fallback 由代码路径审查补充

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 审查 `App.tsx` 的 `BreakWindow`，确认不再存在双栏、圆环与 cue card 结构；截图验证主界面为单列
- AC2 -> 审查 `App.tsx`，确认主界面只保留数字倒计时与条形进度
- AC3 -> 审查 `break-prompt.ts` + locale JSON，确认 prompt 轮播与默认 prompt 都从 locale 取值
- AC4 -> 审查 `break-prompt.ts`，确认不再保存 prompt copy 常量；locale 中存在 prompt 数组
- AC5 -> 运行 typecheck/build 并人工检查设置页 sound label 不再显示原始字段名
- AC6 -> 运行 `typecheck`、`build`、`validate_workflow_docs.py --mode manual`

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->

### Primary flow -> tests/evidence
preview 截图 + `BreakWindow` 代码审查，证明 break 主界面为单列 prompt + countdown。

### Fallback / secondary flow -> tests/evidence
审查 `pickBreakPrompt() || defaultPrompt` 路径与 manual-awaiting 分支，证明 prompt 轮播关闭或 locale 缺失时可回退。

### Visible states / transitions -> tests/evidence
运行中倒计时、manual-awaiting 归零提示与 CTA disabled 状态均在 `App.tsx` 中可追溯；如可行，补一张 preview 截图作为佐证。

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
- Artifacts path: docs/specs/TID-20260408-break-prompt-purity-pass/evidence/
- What to capture:
  - Screenshots: 单列 break prompt 预览截图；如可行，再补设置页 sound labels 正常显示截图
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y: CTA 与设置控件保持键盘可达；主文案不依赖颜色传达状态
- perf budget: 不新增依赖，不引入重计算；prompt 选择仍为 O(1)
- error handling / observability: locale 缺失时必须回退默认 prompt，不露 key
- security / privacy: 不新增网络、权限或数据写入

## Boundary / Invalid Input Cases
- locale prompt 数组为空或缺失
- `breakIdeasEnabled = false`
- `manualAwaiting = true`
- `currentBreak = null`

## Concurrency / Race Cases (if applicable)
- 1s snapshot polling 更新时，数字倒计时与条形进度必须使用同一个 `remaining` 结果，避免不同步

## Mocks & Test Data
- 使用浏览器 preview 的 `previewSnapshot()` 作为 UI 预览数据
- 使用 locale JSON 中的中英文 prompt 列表作为文案数据源

## Commands to Run
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - TypeScript 和 Vite 构建通过
  - workflow docs 校验通过
  - break prompt 主视觉与 specs 中的单列方案一致
- Outputs to keep (10~20 lines snippet):
  - `npm --prefix apps/desktop run build`
  - `python3 scripts/validate_workflow_docs.py --mode manual`
