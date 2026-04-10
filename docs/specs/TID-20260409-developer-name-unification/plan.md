# Task-ID: TID-20260409-developer-name-unification

## Summary
- Title: 统一替换开发者显示名
- Date: 2026-04-09
- Level: trivial
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 把当前产品与仓库元数据中的开发者显示名从 `Jan Hovancik` 统一替换为 `Clarke Young`。
- In-scope:
  - desktop locale source 与生成 registry
  - archived locale source
  - package / README / metainfo 等当前可见元数据
- Out-of-scope:
  - `LICENSE`
  - 新邮箱、主页或 sponsor 链接替换
  - locale key 重命名
- Assumptions:
  - 用户提供的新开发者姓名 `Clarke Young` 是唯一需要更新的信息
  - 旧 URL/邮箱并非本轮任务目标
- Risks:
  - registry 未重生成会导致桌面端仍展示旧值
  - Turkish about 文案顺序若不处理会出现错位
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 统一替换当前开发者显示名，同时保持功能、契约和法律归属声明不被误改。

## Scope
- In-scope:
  - `apps/desktop/src/locales/messages/*.json`
  - `app/locales/*.json`
  - `apps/desktop/src/locales/registry.generated.json`
  - `package.json`
  - `README.md`
  - `net.hovancik.Pauza.metainfo.xml`
  - `docs/CHANGELOG.md`
  - 本任务 specs / plans / logs
- Out-of-scope:
  - `LICENSE`
  - 项目 URL、邮箱、捐赠链接
  - 运行时代码逻辑

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `package.json`
  - `README.md`
  - `net.hovancik.Pauza.metainfo.xml`
  - `app/preferences.html`
  - `apps/desktop/src/locales/messages/{en,zh-CN,tr}.json`
- Related docs/specs/logs reviewed:
  - `docs/CHANGELOG.md`
  - 当日 `docs/plans/2026-04-09.md` / `docs/logs/2026-04-09.md`
- Why these are sufficient:
  - 已覆盖开发者显示名的主要产品出口、locale 结构和仓库元数据入口，足以完成本轮统一替换。

## Acceptance Criteria (AC)
- AC1: `apps/desktop/src/locales/messages/*.json` 和重新生成后的 `registry.generated.json` 中，当前开发者显示名统一为 `Clarke Young`。
- AC2: `app/locales/*.json` 中对应显示名也同步更新，避免归档链路残留旧值。
- AC3: `package.json`、`README.md` 和 `net.hovancik.Pauza.metainfo.xml` 的当前开发者显示名已更新。
- AC4: `LICENSE` 不变。

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
- Required Roles: orchestrator,architect,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限开发者显示文案、应用元数据、desktop/archived locale 与相关 docs
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 Git diff、locale sync、翻译测试与 docs validator
- rollback_plan: 回退 locale JSON、`package.json`、`README.md`、metainfo 与重新生成的 registry
- destructive_operations: none
- operator_approval_required: no
- rationale: 无运行时行为改动、无外部副作用、无历史重写

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 统一替换 locale source 中的开发者显示名
  - DoD: desktop / archived locale 都更新完成，并重新生成 registry
- [x] Task-2: 同步更新仓库元数据与文档
  - DoD: `package.json`、`README.md`、metainfo、CHANGELOG 与任务文档均已同步

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260409-developer-name-unification/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: N/A
  - disabled: N/A
  - success: N/A

## Observability / Debug Plan
- Logs:
  - 用 `rg` 审计 `Jan Hovancik` 与 `Clarke Young` 的替换范围
- Error codes:
  - N/A
- Trace/metrics (optional):
  - N/A
- Debug flags (optional):
  - N/A

## Risks & Rollback
- Risks:
  - registry 未重生成
  - Turkish locale 文案顺序错位
- Rollback plan:
  - 回退 locale JSON、package/metainfo/README 与 registry 产物

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 审计开发者显示名出现位置。
  2. 更新 locale source、package/README/metainfo。
  3. 重新生成 registry 并跑校验。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: no  <!-- yes | no -->
- Approved: N/A（trivial 默认直行；如需审批请手动填写）
