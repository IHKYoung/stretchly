# Task-ID: TID-20260411-pauza-six-step-product-plan

## Summary
- Title: 整理 Pauza 六步产品演进文档
- Date: 2026-04-11
- Level: trivial
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 把此前围绕“Pauza 应从 Stretchly fork 演进成什么”的讨论，整理成一份正式的六步产品文档，明确每一步的目标、具体修改内容、涉及模块与完成标志。
- In-scope:
  - 产出一份稳定的产品规划文档
  - 将六个步骤从抽象建议扩展为可执行的修改清单
  - 同步本任务 specs、当日 plans/logs 与 CodeMap
- Out-of-scope:
  - 直接改动桌面端或官网代码
  - 立即实现 onboarding、dashboard、低打扰能力或付费边界
  - 新增外部依赖或商业化接入
- Assumptions:
  - 用户当前需要的是后续 1~N 个任务的执行蓝图，而不是继续在对话中反复重述方向
  - 六步计划应强调“连续过关步骤”，而不是按周数预估
  - 文档应同时覆盖定位、产品结构、运行时能力和付费边界
- Risks:
  - 若文档只停留在口号层，会继续无法指导具体实现
  - 若步骤过散，会让后续任务又重新滑回“补功能而非做产品”
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 新增一份 `docs/PauzaV1SixStepPlan.md`，把 Pauza 从“另一个 Stretchly”演进为“为 Mac 设计的、低打扰、高质感的恢复节奏工具”的路径写清楚。

## Scope
- In-scope:
  - `docs/PauzaV1SixStepPlan.md`
  - `docs/specs/TID-20260411-pauza-six-step-product-plan/*`
  - `docs/CodeMap.md`
  - `docs/{plans,logs}/2026-04-11.md`
- Out-of-scope:
  - `apps/desktop/**`
  - `apps/site/**`
  - `docs/UI.md`、`docs/Architecture.md` 的产品行为更新

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/site/index.html`
  - `apps/site/copy.js`
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src-tauri/src/platform.rs`
  - `apps/desktop/src/locales/messages/zh-CN.json`
- Related docs/specs/logs reviewed:
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `docs/UI.md`
  - `docs/Architecture.md`
  - 当日 `docs/plans/2026-04-11.md` 与 `docs/logs/2026-04-11.md`
- Why these are sufficient:
  - 已覆盖官网当前定位、桌面端主入口结构、运行时能力边界、核心文案真源与仓库文档约束，足以把“下一步该怎么改”从讨论整理成面向执行的文档。

## Acceptance Criteria (AC)
- AC1: 新文档明确写出 6 个连续步骤，并为每一步补齐目标、为什么先做、具体修改内容、涉及模块和完成标志。
- AC2: 文档内容能够直接映射到当前仓库模块，不停留在抽象口号。
- AC3: 本任务 specs、daily plans/logs 与 CodeMap 完整同步，workflow docs validator 通过。

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
- Required Roles: scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅新增产品规划文档并同步 docs 索引/任务包；不改桌面端、官网或构建产物
- touches_running_service: no
- backup_required: no
- backup_plan: 通过 `git diff` 与 workflow docs validator 验证；若文档方向不满意，可整体回退本任务 docs
- rollback_plan: 回退 `docs/PauzaV1SixStepPlan.md`、`docs/CodeMap.md` 与本任务 specs / daily docs
- destructive_operations: none
- operator_approval_required: no
- rationale: 纯 docs 任务，不涉及代码执行路径、数据、权限、依赖或外部副作用

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 整理六步产品方案主文档
  - DoD: 新文档清楚覆盖定位、主窗口、低打扰能力、反馈闭环和付费边界
- [x] Task-2: 同步任务包与仓库索引
  - DoD: 本任务 spec、daily plans/logs 与 CodeMap 全部更新完成并通过 validator

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260411-pauza-six-step-product-plan/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: N/A
  - disabled: N/A
  - success: `docs/PauzaV1SixStepPlan.md` 成功落盘并可被索引

## Observability / Debug Plan
- Logs: 通过当日 `docs/plans` / `docs/logs` 记录本任务的 Source Basis、Change Summary 与 Verification
- Error codes: N/A
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 文档若没有绑定当前真实模块，会在执行时再次失真
  - 若六步之间缺少依赖说明，后续任务容易越级实现
- Rollback plan:
  - 删除新文档并回退本任务 docs 变更，恢复到无稳定产品方案文档的状态

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 汇总当前产品定位、主入口、运行时能力和对外文案的真实状态。
  2. 将此前口头讨论整理为六个连续过关步骤，并映射到仓库模块。
  3. 把文档落到 `docs/`，再同步本任务 specs、daily docs 与 CodeMap。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: no  <!-- yes | no -->
- Approved: N/A（trivial 默认直行；如需审批请手动填写）
