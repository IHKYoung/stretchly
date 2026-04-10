# Task-ID: TID-20260409-smart-reminder-ui-simplification

## Summary
- Title: 收回智能提醒阈值设置项并保留内部策略
- Date: 2026-04-09
- Level: trivial
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 减少 smart reminder 的前台参数心智，把阈值设置从 UI 收回，但不改 host 内部默认策略。
- In-scope:
  - 撤回设置页中的 `等待空档` 控件
  - 删除对应 locale key
  - 同步测试与文档口径
- Out-of-scope:
  - 修改 `state.rs` 内部策略
  - 新增其他提醒设置
- Assumptions:
  - 当前更重要的是减少可见参数，而不是继续细调默认值
- Risks:
  - 文档或 registry 残留旧 key
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 让用户只需要理解“智能提醒 / 强制提醒”，不需要再面对额外阈值设置。

## Scope
- In-scope:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/locales/messages/{en,zh-CN}.json`
  - `apps/desktop/src/locales/registry.generated.json`
  - `test/desktopSettingsControls.js`
  - `docs/{UI,Architecture,SettingsInventory,CHANGELOG}.md` 与本任务 specs/logs/plans
- Out-of-scope:
  - `apps/desktop/src-tauri/src/state.rs`
  - reminder mode / schema 设计调整

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/locales/messages/{en,zh-CN}.json`
  - `apps/desktop/src/i18n.ts`
  - `test/desktopSettingsControls.js`
- Related docs/specs/logs reviewed:
  - `docs/{UI,Architecture,SettingsInventory,CHANGELOG}.md`
  - `docs/logs/2026-04-09.md`
  - `docs/specs/TID-20260409-smart-reminder-wait-guard/*`
- Why these are sufficient:
  - 已覆盖当前 UI 暴露点、locale 真源、测试入口与同日刚落地的 smart reminder 产品口径。

## Acceptance Criteria (AC)
- AC1: 设置页中不再出现 smart reminder 的阈值控件。
- AC2: locale 真源与 registry 不再保留 `ui.waitForPause*`。
- AC3: smart reminder 的内部 `6s + bounded wait cap` 策略保持不变。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: N/A
- Primary flow: N/A
- Fallback / secondary flow: N/A
- Interaction authority / ownership boundary: N/A
- Visible entrypoints / handoff cues: N/A
- In-scope interactions: N/A
- Out-of-scope interactions: N/A
- Interaction acceptance criteria: N/A
- Validator expectation: 当 `interaction_impact != none` 时，本节与 Requirement Brief 中的交互字段不得继续保留 `N/A/TBD`

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限设置页 smart reminder 控件、locale 与相关文档
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 locale sync、npm test、typecheck、build 与 docs validator
- rollback_plan: 回退 `App.tsx`、locale、测试与 docs
- destructive_operations: none
- operator_approval_required: no
- rationale: 不改 host 状态机，不涉及线上服务、权限或数据迁移

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 撤回前台 smart reminder 阈值控件
  - DoD: `App.tsx` 中不再渲染 `waitForPause` 行
- [x] Task-2: 清理 locale / 测试 / 文档口径
  - DoD: registry 重建，测试通过，文档恢复为 hidden strategy 口径

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260409-smart-reminder-ui-simplification/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: N/A
  - disabled: N/A
  - success: N/A

## Observability / Debug Plan
- Logs: 无新增
- Error codes: 无
- Trace/metrics (optional): 无
- Debug flags (optional): 无

## Risks & Rollback
- Risks:
  - 文档与当前 UI 真相漂移
  - locale registry 残留删除前 key
- Rollback plan:
  - 回退 `App.tsx`、locale、测试与 docs
  - 重跑 locale sync 和前端验证

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 删除设置页中的 smart reminder 阈值控件。
  2. 删除对应 locale key 并重建 registry。
  3. 更新测试和文档口径。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: no  <!-- yes | no -->
- Approved: N/A（trivial 默认直行；如需审批请手动填写）
