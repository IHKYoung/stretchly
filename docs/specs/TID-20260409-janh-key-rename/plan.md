# Task-ID: TID-20260409-janh-key-rename

## Summary
- Title: 统一开发者 about key 为 clarkeY
- Date: 2026-04-09
- Level: trivial
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 把仓库里开发者姓名相关 locale key 从 `preferences.about.janH` 统一重命名为 `preferences.about.clarkeY`，并同步 legacy HTML 引用、desktop registry 与同日文档，保证后续 grep 不再出现旧 key。
- In-scope:
  - `apps/desktop/src/locales/messages/*.json`
  - `app/locales/*.json`
  - `apps/desktop/src/locales/registry.generated.json`
  - `app/preferences.html`
  - 同日相关 docs / specs / plans / logs
- Out-of-scope:
  - 开发者显示值、URL、README、package、metainfo、LICENSE
  - 任何前台/宿主运行时逻辑
- Assumptions:
  - 当前不需要保留 `janH` 兼容 alias
  - 旧 Electron 目录虽已归档，但其 locale 和 HTML 引用仍要保持自洽
- Risks:
  - locale key 重命名后若 registry 未重生成，会残留新旧混杂产物
  - 同日旧任务 docs 若不更新，会继续记录错误的当前 key 名
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 统一开发者姓名 about key 为 `clarkeY`，并清零仓库内的 `janH` 残留。

## Scope
- In-scope:
  - `apps/desktop/src/locales/messages/*.json`
  - `app/locales/*.json`
  - `apps/desktop/src/locales/registry.generated.json`
  - `app/preferences.html`
  - `docs/specs/TID-20260409-developer-name-unification/{README.md,arch.md,testplan.md}`
  - `docs/specs/TID-20260409-janh-key-rename/*`
  - `docs/{plans,logs}/2026-04-09.md`
- Out-of-scope:
  - `package.json`
  - `README.md`
  - `net.hovancik.Pauza.metainfo.xml`
  - `LICENSE`

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/locales/messages/*.json`
  - `app/locales/*.json`
  - `apps/desktop/src/locales/registry.generated.json`
  - `app/preferences.html`
- Related docs/specs/logs reviewed:
  - `docs/specs/TID-20260409-developer-name-unification/{README.md,arch.md,testplan.md}`
  - `docs/specs/TID-20260409-janh-key-rename/*`
  - `docs/{plans,logs}/2026-04-09.md`
- Why these are sufficient:
  - 已覆盖当前 key 真源、生成产物、legacy 引用与需要同步的历史说明，足以完成这次纯 key rename。

## Acceptance Criteria (AC)
- AC1: desktop locale、archived locale 与 generated registry 中不再出现 `janH`。
- AC2: `app/preferences.html` 改为 `preferences.about.clarkeY`。
- AC3: 同日相关 docs 不再把当前 key 写成 `janH`。
- AC4: `python3 scripts/sync_desktop_locales.py`、`npm test`、`python3 scripts/validate_workflow_docs.py --mode manual` 通过，且 `rg -n "janH"` 返回 0。

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
- service_impact: 仅限 locale key 名、desktop registry、legacy HTML 引用与相关 docs
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 Git diff、locale sync、tests、docs validator 与 grep 审计作为回退边界
- rollback_plan: 回退 locale JSON、`app/preferences.html`、registry 与相关 docs
- destructive_operations: none
- operator_approval_required: no
- rationale: 无线上服务、提权、外部副作用、依赖新增或数据迁移

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 批量重命名 locale source 与 legacy HTML 引用
  - DoD: `messages/*.json`、`app/locales/*.json` 与 `app/preferences.html` 全部使用 `clarkeY`
- [x] Task-2: 重生成 registry 并同步相关 docs
  - DoD: generated registry、同日 docs/specs 与验证结果全部收口到新 key

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260409-janh-key-rename/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: N/A
  - disabled: N/A
  - success: N/A

## Observability / Debug Plan
- Logs:
  - `rg -n "janH"` 审计旧 key 残留
  - `sync_desktop_locales.py` 输出 registry 重建结果
- Error codes:
  - N/A
- Trace/metrics (optional):
  - N/A
- Debug flags (optional):
  - N/A

## Risks & Rollback
- Risks:
  - 批量改 key 后若遗漏一个 legacy 引用，会导致旧壳 HTML 与 locale 不一致
  - 旧 docs 仍记录旧 key，会误导后续维护
- Rollback plan:
  - 回退 locale JSON、legacy HTML、registry 与 docs 改动，再重新跑 sync / tests / validator

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 审计 `janH` 的 source / generated / docs 分布。
  2. 批量重命名 locale key 与 legacy HTML 引用。
  3. 重生成 desktop registry，并同步同日相关 docs。
  4. 运行 grep / sync / test / docs validator 验证。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: no  <!-- yes | no -->
- Approved: N/A（trivial 默认直行；如需审批请手动填写）
