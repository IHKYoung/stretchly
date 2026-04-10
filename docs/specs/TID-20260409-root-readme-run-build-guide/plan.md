# Task-ID: TID-20260409-root-readme-run-build-guide

## Summary
- Title: 在根 README 汇总运行与打包命令
- Date: 2026-04-09
- Level: trivial
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 在根 `README.md` 重建一个当前有效的命令入口，集中说明如何运行、测试、打包和查找 macOS 产物。
- In-scope: 新建精简根 README；汇总根脚本与 `apps/desktop` 实际生效的运行/打包命令；补充 macOS 当前架构打包、universal 打包与产物路径；同步 `docs/RepositoryGuidelines.md`、`docs/CodeMap.md` 与本任务文档。
- Out-of-scope: 恢复旧 README 展示素材、重写品牌介绍、修改运行/构建脚本本身、引入新命令或新依赖。
- Assumptions: 当前真实命令入口以根 `package.json` 与 `apps/desktop/package.json` 为准；根 README 目前缺失，需要重新创建。
- Risks: 若 README 直接手写命令而不以现有 scripts 为依据，后续容易再次漂移；若不更新索引文档，会继续存在“README 已清理”的过期说明。
- Interaction impact: none
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 恢复一个准确、简洁、可直接复制命令的根 README，降低后续运行和打包的查找成本。

## Scope
- In-scope:
  - 新建根 `README.md`
  - 记录 `npm start` / `npm run dev` / `npm run desktop:dev`
  - 记录 `npm test`、`typecheck`、locale/icon 辅助命令
  - 记录 `npm run desktop:build`、`npm run pack`、`npm run dist`
  - 记录 macOS universal 打包命令与产物路径
  - 同步 `docs/RepositoryGuidelines.md` 与 `docs/CodeMap.md`
- Out-of-scope:
  - 修改任何构建脚本实现
  - 修改 Tauri bundle 配置
  - 修改签名、公证环境变量本身
  - 恢复历史 root metadata 文件

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `package.json`
  - `apps/desktop/package.json`
  - `apps/desktop/src-tauri/tauri.conf.json`
- Related docs/specs/logs reviewed:
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `docs/logs/2026-04-08.md`
- Why these are sufficient:
  - 这组文件已经覆盖根脚本真源、desktop 子应用脚本真源、Tauri bundle 配置、现有 macOS 打包结果和仓库索引口径，足以支撑 README 汇总。

## Acceptance Criteria (AC)
- AC1: 根 `README.md` 存在，并准确列出当前实际可用的运行、测试和打包命令。
- AC2: README 明确指出 `apps/desktop` 是当前唯一有效主实现，并给出 macOS 产物路径。
- AC3: `docs/RepositoryGuidelines.md` 与 `docs/CodeMap.md` 不再保留“根 README 已清理”的过期口径。
- AC4: 本任务 docs 无 `INIT/TBD` 占位，且 workflow docs validator 通过。

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
- service_impact: 仅限仓库入口文档与索引文档
- touches_running_service: no
- backup_required: no
- backup_plan: 以 Git diff、README grep 与 workflow docs validator 为边界
- rollback_plan: 回退 `README.md`、`docs/RepositoryGuidelines.md`、`docs/CodeMap.md` 与本任务文档
- destructive_operations: none
- operator_approval_required: no
- rationale: 纯文档改动，不涉及运行时逻辑、数据、外部副作用或提权

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 核对当前有效运行/打包命令真源
  - DoD: 根 `package.json`、`apps/desktop/package.json` 与 `tauri.conf.json` 已审阅，README 中不写虚假命令。
- [x] Task-2: 新建根 README 并汇总运行、测试、打包与 macOS 产物说明
  - DoD: `README.md` 可作为当前仓库的快速入口直接使用。
- [x] Task-3: 同步索引文档与任务文档
  - DoD: `docs/RepositoryGuidelines.md`、`docs/CodeMap.md`、specs/plans/logs 口径一致且无占位。

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260409-root-readme-run-build-guide/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading:
  - empty:
  - error:
  - disabled:
  - success:

## Observability / Debug Plan
- Logs: 不涉及运行时日志
- Error codes: N/A
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - README 与真实脚本不一致，导致用户复制命令失败
  - 索引文档未同步，继续出现 README 缺失/已清理的过期说明
- Rollback plan:
  - 若 README 口径不正确，直接回退本次新增/修改的文档文件即可

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 审阅当前脚本与 Tauri 配置，确认运行与打包真源。
  2. 创建精简根 README，收口运行、测试、打包与 macOS 出包说明。
  3. 同步更新索引与任务文档，并跑文档门禁校验。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: no  <!-- yes | no -->
- Approved: N/A（trivial 默认直行；如需审批请手动填写）
