# Task-ID: TID-20260409-apps-single-source-refactor

## Summary
- Title: 收口到 apps 单真源并停用 app 目录
- Date: 2026-04-09
- Level: complex
- Lane: deep
- Execution Profile: sequential-phases
- Status: DONE

## Requirement Brief
- Goal restatement: 把默认桌面端运行链路彻底收口到 `apps/desktop`，停止 `app/` 目录继续参与当前产品；同时去掉 locale `overrides` 架构，并把 break 消息页提示语变成 `apps/desktop` 内可直接编辑的单独文件。
- In-scope:
  - 根级 scripts / tests 对 `app/` 的剩余引用迁移
  - `apps/desktop` locale 结构扁平化
  - 主文档、架构索引和任务文档口径统一
- Out-of-scope:
  - 物理删除 `app/` 目录
  - 重写 Tauri host 行为或补新产品能力
  - 新增依赖或外部服务
- Assumptions:
  - `apps/desktop` 已具备当前产品的默认运行能力
  - `app/` 当前可退化为 archived reference
  - `break-message-copy.json` 足以承接 break 页面专属文案编辑入口
- Risks:
  - 测试路径迁移后出现 import 漏改
  - locale 扁平化后出现 message/config 漏配
  - 文档没同步完全，导致仓库继续表现为“实现和说明不一致”
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 完成 `apps/desktop` 单真源重构，让默认运行、构建、测试和 locale 资产都不再依赖 `app/`。

## Scope
- In-scope:
  - `package.json`
  - `apps/desktop/src/locales/**`
  - `apps/desktop/legacy-utils/**`
  - `test/**`
  - `scripts/sync_desktop_locales.py`
  - `README.md`、`apps/desktop/README.md`、`docs/{RepositoryGuidelines,CodeMap,Architecture,SettingsInventory,UI,CHANGELOG}.md`
  - 本任务 specs / plans / logs
- Out-of-scope:
  - 删除 `app/**`
  - 改变 break reminder 交互契约
  - 提交 commit / push

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `package.json`
  - `scripts/sync_desktop_locales.py`
  - `apps/desktop/src/{App.tsx,i18n.ts,locales/break-message-copy.ts,locales/break-message-copy.json}`
  - `apps/desktop/src-tauri/src/i18n.rs`
  - `test/{translations.js,utils.js}` 及其余根级测试 import
  - `apps/desktop/README.md`
- Related docs/specs/logs reviewed:
  - `README.md`
  - `docs/{RepositoryGuidelines,CodeMap,Architecture,SettingsInventory,UI,CHANGELOG}.md`
  - `docs/{plans,logs}/2026-04-09.md`
  - `docs/specs/TID-20260409-break-message-copy-centralization/*`
- Why these are sufficient:
  - 已覆盖根脚本、当前桌面端 locale / break copy 实现、测试入口与仓库主文档，足以完成“单真源重构”而不触碰无关运行时逻辑。

## Acceptance Criteria (AC)
- AC1: 根 `package.json` 与根级测试不再依赖 `app/main.js`、`app/utils/**` 或 `app/locales/**`。
- AC2: `scripts/sync_desktop_locales.py` 只读取 `apps/desktop/src/locales/{messages,config}`，不再读取 `overrides/` 或 legacy `app/` 资源。
- AC3: break 消息页提示语可在 `apps/desktop/src/locales/break-message-copy.json` 中统一编辑，运行时代码只通过 helper 读取该文件。
- AC4: `README.md`、`apps/desktop/README.md` 与 `docs/{RepositoryGuidelines,CodeMap,Architecture,SettingsInventory,UI}.md` 明确声明 `apps/desktop` 为当前单真源。
- AC5: `python3 scripts/sync_desktop_locales.py`、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`、`npm test` 与 `python3 scripts/validate_workflow_docs.py --mode manual` 全部通过。

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
- Execution Profile: sequential-phases
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,architect,coder,tester,scribe,reality_checker
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限本地桌面端源码归属、根脚本、测试引用、locale 生成链路与文档
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 Git diff、locale sync、typecheck/build/test 与 docs validator 作为回退前边界
- rollback_plan: 回退 `package.json`、`apps/desktop/**`、`test/**`、`scripts/sync_desktop_locales.py` 与相关 docs
- destructive_operations: none
- operator_approval_required: no
- rationale: 无线上服务、提权、付费成本或外部副作用，也不直接删除 `app/` 目录

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 审计并切断根级 scripts / tests / locale generator 对 `app/` 的默认依赖
  - DoD: 根入口、测试 import 与 locale 生成脚本都改为只依赖 `apps/desktop`
- [x] Task-2: 扁平化桌面端 locale 真源，移除 `overrides` 叠加结构
  - DoD: message/config 一一对应，registry 可直接生成，`overrides/` 不再是默认架构的一部分
- [x] Task-3: 更新仓库 README、架构索引、desktop README 与 workflow 文档
  - DoD: 主文档明确声明 `apps/desktop` 单真源，`app/` 只保留 archived reference 语义
- [x] Task-4: 完成验证闭环
  - DoD: locale sync、typecheck、build、test、docs validator 全部通过

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260409-apps-single-source-refactor/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: N/A
  - disabled: N/A
  - success: N/A

## Observability / Debug Plan
- Logs:
  - 用 `rg` 审计剩余 `app/` / `overrides` 引用
  - 用 locale sync 的错误消息暴露配置缺失
- Error codes:
  - N/A（脚本失败直接退出）
- Trace/metrics (optional):
  - N/A
- Debug flags (optional):
  - N/A

## Risks & Rollback
- Risks:
  - 根级测试对 `apps/desktop/legacy-utils` 的新路径耦合可能漏改
  - locale source 扁平化后，历史字段若遗漏会直接导致构建失败
  - 文档若仍残留旧口径，后续维护者会继续误判 `app/` 仍在默认链路上
- Rollback plan:
  - 逐项回退根 scripts、tests、locale 目录与 docs 改动，再重新生成 registry 并跑验证命令

## Sequential Phases
- phase_execution: sequential  <!-- sequential | N/A -->
- phase_confirmation_policy: no-intermediate-confirmation  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - 发现仍有默认运行、构建或测试链路直接依赖 `app/**`
  - locale 扁平化后无法通过 sync / typecheck / build / test 验证
  - 若用户要求物理删除 `app/`，则升级为新的高风险清理任务

## Execution Plan
- Steps:
  1. 审计剩余 `app/` / `overrides` 引用，区分真实运行链路与历史文档。
  2. 迁移根 tests、locale generator 与 break copy 入口到 `apps/desktop` 单真源结构。
  3. 同步 README / docs / workflow 文档口径。
  4. 跑完整验证命令并结案。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: yes（用户于 2026-04-09 明确要求“我要的是重构！”并要求停止继续混用 `app/`）
