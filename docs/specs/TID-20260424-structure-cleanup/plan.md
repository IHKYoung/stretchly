# Task-ID: TID-20260424-structure-cleanup

## Summary
- Title: 整理 reminder 与 break ideas 的结构残留
- Date: 2026-04-24
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 收掉当前代码里“已经不是现行策略、却还继续挂在主模型或主流程里”的 reminder / break ideas 结构残留，让前台模型、host 序列化边界和调度主流程重新对齐。
- In-scope:
  - `apps/desktop/src/App.tsx` 的设置模型整理
  - `apps/desktop/src-tauri/src/state.rs` 的兼容字段与 due helper 收口
  - `docs/SettingsInventory.md`、`docs/Architecture.md`、`docs/CHANGELOG.md`
- Out-of-scope:
  - 大规模拆分 `state.rs`
  - TS/Rust i18n 逻辑去重
  - 调整 smart / forced 的运行时行为和阈值
- Assumptions:
  - 本轮目标是结构整理，不是行为变更
  - 旧配置里的 `idleOpportunitySeconds` 仍可能存在，载入兼容不能断
- Risks:
  - 若兼容字段处理不当，可能影响旧配置反序列化
  - 若 due helper 抽取有误，可能导致 heads-up / wait / break start 路径漂移
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 让现行 reminder 结构的代码表达与文档表达重新一致：前台不再继续携带废弃兼容字段，host 兼容字段只保留在载入层，主调度路径只保留当前仍生效的分支。

## Scope
- In-scope:
  - 前台 `PauzaSettings` 类型与默认值
  - `PauzaSettings` 的序列化/反序列化边界
  - `RuntimeState` 的到点通知 / 到点开休息 helper
  - 结构相关文档
- Out-of-scope:
  - 新增设置项
  - 更改现有 locale 资产结构
  - 新一轮状态机设计

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src/i18n.ts`
  - `apps/desktop/src/lib/break-ideas.ts`
- Related docs/specs/logs reviewed:
  - `docs/SettingsInventory.md`
  - `docs/Architecture.md`
  - `docs/ReminderScheduling.md`
  - `docs/specs/TID-20260424-smart-reminder-minimal-wait-model/arch.md`
  - `docs/specs/TID-20260424-break-ideas-asset-migration/README.md`
- Why these are sufficient:
  - 已覆盖当前 reminder 主逻辑、break ideas 消费入口、前台设置模型和现行文档口径，足以完成这次结构性收口而不引入新设计。

## Acceptance Criteria (AC)
- AC1: 前台设置模型和新的 settings/snapshot 序列化结果不再继续暴露 `idleOpportunitySeconds`，但 host 仍可兼容读取旧配置。
- AC2: `state.rs::tick()` 的“到点通知 / 到点开休息”主路径被 helper 收口，行为不变且 Rust/Vitest/typecheck 全部通过。
- AC3: 结构类文档不再把 `idle_opportunity_seconds` 写成当前可用设置，也不再描述旧的 staged threshold / recovery hold 模型。

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
- Required Roles: architect,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限本地 desktop 前台设置模型、Rust host 调度 helper、文档与测试
- touches_running_service: no
- backup_required: no
- backup_plan: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`npm run typecheck`、`npm test`、`python3 scripts/validate_workflow_docs.py --mode manual`
- rollback_plan: 回退 `App.tsx`、`state.rs` 与本任务 docs，恢复兼容字段的旧序列化行为和 `tick()` 内联分支
- destructive_operations: none
- operator_approval_required: no
- rationale: 不涉及线上服务、外部副作用、权限提升或破坏性数据操作

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 清理前台/host 的 `idleOpportunitySeconds` 残留
  - DoD: 前台类型移除该字段；host 仅保留反序列化兼容；新增测试证明不再写回序列化结果
- [x] Task-2: 收口 `tick()` 的 due 分支与结构文档
  - DoD: due notification / due break helper 落地；文档与 changelog 更新；验证通过

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260424-structure-cleanup/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading:
  - empty:
  - error:
  - disabled:
  - success:

## Observability / Debug Plan
- Logs: 继续依赖现有 `runtime.actions.notificationSent`、`waitingForOpportunity`、`breakStarted` 等状态文案，不新增结构化日志
- Error codes: N/A
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 旧配置载入后若立即保存，兼容字段会被自然清理出 settings 文件
  - helper 抽取若判定边界不一致，可能影响 due 时序
- Rollback plan:
  - 回退 `App.tsx` 与 `state.rs` 的本轮改动
  - 重新执行 Rust/Vitest/typecheck 验证

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 从前台设置模型移除 `idleOpportunitySeconds`
  2. 将 host 兼容字段改为“只读旧配置，不再序列化”
  3. 抽取 due notification / due break helper
  4. 更新文档并执行验证

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: 用户在当前对话中以“需要整理一下”批准继续按最小结构整理推进
