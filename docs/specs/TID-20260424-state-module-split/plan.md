# Task-ID: TID-20260424-state-module-split

## Summary
- Title: 拆分 state.rs 结构边界
- Date: 2026-04-24
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 在不改变 smart / forced 提醒行为和外部 API 的前提下，把 `state.rs` 中已经能清晰归类的“设置 schema / 规范化 / 持久化迁移 / 单测”拆成子模块，让 runtime 调度核心和配置边界更清楚。
- In-scope:
  - 新增 `apps/desktop/src-tauri/src/state/settings.rs`
  - 新增 `apps/desktop/src-tauri/src/state/persistence.rs`
  - 新增 `apps/desktop/src-tauri/src/state/tests.rs`
  - 调整 `apps/desktop/src-tauri/src/state.rs` 的 import / re-export / 边界
  - 更新 `docs/CodeMap.md`、`docs/Architecture.md`、`docs/CHANGELOG.md`
  - 补齐本任务 specs / logs / plans
- Out-of-scope:
  - 拆分 `RuntimeState` / `PauzaState` 调度主逻辑
  - 调整 reminder 策略、阈值或设置项
  - 改动前端设置模型
- Assumptions:
  - `crate::state` 对外暴露的 `PauzaSettings`、`BreakKind`、`ShortcutAction` 等入口需要保持兼容
  - 当前收益最大的结构整理是先拆“settings + persistence + tests”，而不是继续切 runtime 主状态机
- Risks:
  - 子模块可见性或 re-export 处理不当，会导致 `commands.rs` / `platform.rs` / `shell.rs` 编译失败
  - 设置迁移逻辑若在拆分时漂移，可能影响旧配置载入
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 将 `state.rs` 从“设置 schema + 迁移持久化 + runtime 调度 + inline tests”混堆，收口为“runtime 主文件 + settings 子模块 + persistence 子模块 + tests 子模块”的清晰结构。

## Scope
- In-scope:
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src-tauri/src/state/settings.rs`
  - `apps/desktop/src-tauri/src/state/persistence.rs`
  - `apps/desktop/src-tauri/src/state/tests.rs`
  - `docs/CodeMap.md`
  - `docs/Architecture.md`
  - `docs/CHANGELOG.md`
  - 本任务 specs / logs / plans
- Out-of-scope:
  - `engine.rs` / `shell.rs` / `commands.rs` 的行为改动
  - 测试逻辑重写
  - `state.rs` runtime 状态机进一步文件化

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src-tauri/src/{commands.rs,engine.rs,platform.rs,shell.rs}`
  - `apps/desktop/src/App.tsx`
- Related docs/specs/logs reviewed:
  - `docs/CodeMap.md`
  - `docs/Architecture.md`
  - `docs/ReminderScheduling.md`
  - `docs/specs/TID-20260424-structure-cleanup/plan.md`
  - `docs/logs/2026-04-24.md`
- Why these are sufficient:
  - 已覆盖 `crate::state` 的内部实现、外部消费者、现行调度设计和前一轮结构整理边界，足以完成本轮低风险模块拆分。

## Acceptance Criteria (AC)
- AC1: `PauzaSettings`、相关 enum/shortcut 类型的定义、默认值、规范化逻辑全部迁移到 `state/settings.rs`，`crate::state` 对外导出保持不变。
- AC2: `settings.json` 的 load / save / legacy migration 逻辑迁移到 `state/persistence.rs`，旧配置兼容与序列化行为不变。
- AC3: `state.rs` 保留 runtime 调度与 snapshot 逻辑，不再混放 settings schema / 持久化实现 / inline tests；Rust tests、typecheck、Vitest 与 workflow docs validator 全部通过。

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
- service_impact: 仅限本地 desktop Rust host 的模块边界、文档与测试
- touches_running_service: no
- backup_required: no
- backup_plan: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`npm run typecheck`、`npm test`、`python3 scripts/validate_workflow_docs.py --mode manual`
- rollback_plan: 回退 `state.rs`、删除 `state/{settings,persistence}.rs` 并恢复文档口径
- destructive_operations: none
- operator_approval_required: no
- rationale: 本轮是纯本地结构整理，不涉及线上服务、权限提升或外部副作用

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 拆出 settings 子模块
  - DoD: `PauzaSettings`、相关 enum、shortcut 和规范化 helper 从 `state.rs` 迁出，外部 API 不变
- [x] Task-2: 拆出 persistence / tests 子模块并收文档
  - DoD: `load/save/migrate` 与 Rust tests 迁出完成，CodeMap/Architecture/CHANGELOG 与 task docs 同步，验证通过

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260424-state-module-split/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading:
  - empty:
  - error:
  - disabled:
  - success:

## Observability / Debug Plan
- Logs: 不新增日志；保留现有 runtime 状态文案与测试覆盖，确保结构整理不改变行为
- Error codes: N/A
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - re-export 漏项会造成外部消费者编译失败
  - 持久化逻辑迁移若丢掉 legacy path，会影响旧配置兼容
- Rollback plan:
  - 回退 `state.rs` 与新增子模块
  - 重新执行 Rust / TS / Vitest 校验确认恢复

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 将 settings schema / sanitization 从 `state.rs` 迁到 `state/settings.rs`
  2. 将 load / save / migrate 从 `state.rs` 迁到 `state/persistence.rs`
  3. 将 inline tests 从 `state.rs` 迁到 `state/tests.rs`
  4. 用 re-export 和子模块声明保持 `crate::state` 外部 API 不变
  5. 更新结构文档并执行验证

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: 用户在当前对话中以“继续整理一下，把代码结构整理清楚”批准继续做结构收口
