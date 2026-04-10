# Task-ID: TID-20260409-break-window-layout-and-controls-refresh

## Summary
- Title: 重做 break 界面布局并移除跳过按钮
- Date: 2026-04-09
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 将 break 页面收口为更安静的单列休息界面，并移除主界面的 `Skip`，只保留与当前 break 状态一致的动作。
- In-scope:
  - `apps/desktop/src/App.tsx` 的 break 页面布局与 CTA 渲染
  - `apps/desktop/src/lib/break-prompt.ts` 的 break scene / palette 接线
  - 本任务 specs、daily logs/plans 与 `docs/{UI,CHANGELOG}.md`
- Out-of-scope:
  - `state.rs` 中的 break 调度与 CTA 时机判定
  - 设置页结构、多语言真源与宿主窗口行为
- Assumptions:
  - 当前用户希望 break 页面更克制，而不是保留展示型双栏结构
  - `Skip` 不应继续作为主界面常驻动作；必要时可在后续任务评估更隐蔽的次级入口
- Risks:
  - CTA 做减法过度会让退出路径过窄
  - 布局调整若破坏壁纸/主题层次，break 页面可读性会下降
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: break 到点后，页面以单列方式显示 eyebrow、提示语、倒计时和细进度条，只暴露当前状态允许的动作。
- Fallback / secondary flow: `manualAwaiting` 时显示 `Resume work`；`canPostpone` 为真时显示 `Later`。
- User-visible boundary: 仅影响 `?window=break` 的前台 break 页面。
- Key visible states / transitions:
  - break start
  - active countdown
  - canPostpone window
  - manualAwaiting
  - exit via allowed CTA

## Goal
- 将 break 页面从“展示型双栏 + 多余控制”收口为真正面向休息的单列界面，并把 CTA 边界压回运行时允许的动作集合。

## Scope
- In-scope:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/lib/break-prompt.ts`
  - `docs/specs/TID-20260409-break-window-layout-and-controls-refresh/*`
  - `docs/{UI,CHANGELOG}.md`
  - `docs/{plans,logs}/2026-04-09.md`
- Out-of-scope:
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src/locales/messages/*.json`
  - `app/**`

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/lib/break-prompt.ts`
- Related docs/specs/logs reviewed:
  - `docs/UI.md`
  - `docs/CHANGELOG.md`
  - 同日 break prompt / CTA 相邻任务记录
- Why these are sufficient:
  - break 页的视觉层级和 CTA 出口集中在 `App.tsx` 与 `break-prompt.ts`，结合当日相邻任务即可确定本轮只做前台收敛而不改 host 语义。

## Acceptance Criteria (AC)
- AC1: break 页面采用居中的单列主结构，不再保留旧的展示型双栏布局。
- AC2: 主视觉改为数字倒计时与细条形进度，保留单条提示语和更克制的文本层次。
- AC3: break 页面主界面不再渲染 `Skip`，只按当前运行态渲染 `Resume work` / `Later` 等允许动作。
- AC4: `npm test`、`npm --prefix apps/desktop run build` 与 workflow docs validator 通过。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen
- Primary flow: break 到点后以单列内容区呈现提示语、倒计时和进度，视觉焦点只围绕“休息正在发生”。
- Fallback / secondary flow: CTA 只按 `manualAwaiting` / `canPostpone` 暴露；无额外 `Skip` 常驻入口。
- Interaction authority / ownership boundary: 只改前台 break 页的布局和 CTA 渲染，不改 host 判定与命令面。
- Visible entrypoints / handoff cues: `?window=break`
- In-scope interactions:
  - countdown + meter 主视觉
  - 单列文本布局
  - CTA 显示边界
- Out-of-scope interactions:
  - settings page
  - reminder scheduling
  - host-level CTA availability rules
- Interaction acceptance criteria:
  - break 页面不再表现为展示型双栏结构。
  - 当前 break 状态不允许的动作不会在主界面出现。
- Validator expectation: direct interaction 字段已补齐。

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,architect,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限 break 页面布局、CTA 出口、相关文档与验证
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 `npm test`、desktop build 与 workflow docs validator
- rollback_plan: 回退 `App.tsx`、`break-prompt.ts` 与相关 docs
- destructive_operations: none
- operator_approval_required: no
- rationale: 仅前台界面收敛，不涉及运行中服务、数据迁移、权限或外部副作用

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 收口 break 页面主结构
  - DoD: 页面改为单列内容区，主视觉为提示语、数字倒计时与细进度条
- [x] Task-2: 清理主界面 CTA 并补文档/验证
  - DoD: `Skip` 不再出现在主界面，构建链与 docs gate 通过

## Evidence Plan (UI / E2E)
- Evidence required: partial  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260409-break-window-layout-and-controls-refresh/evidence/
- Interaction validation note: 当前以代码路径、build/test 与 evidence README 作为主要证明；未保留额外录屏资产。
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: 不允许的 CTA 不应继续被渲染
  - disabled: busy action 时沿用现有按钮 disabled
  - success: break 页面呈现单列结构，CTA 与 `manualAwaiting` / `canPostpone` 一致

## Observability / Debug Plan
- Logs: 沿用现有 `lastAction` 与 command error 提示
- Error codes: 沿用现有前台命令错误模型
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 去掉 `Skip` 后若没有保留正确的替代动作，可能让页面出口过窄
  - 布局收敛若破坏壁纸/主题层次，break 页面可读性会下降
- Rollback plan:
  - 回退 `App.tsx` / `break-prompt.ts` / docs
  - 重新执行 `npm test`、desktop build 与 docs validator

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 收口 break 页面文本、倒计时和进度主结构
  2. 清理主界面 CTA，只保留与当前状态一致的动作
  3. 更新 UI/CHANGELOG 与 task docs，并跑前台验证

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: yes（用户于 2026-04-09 明确要求重做 break 界面布局并移除跳过按钮）
