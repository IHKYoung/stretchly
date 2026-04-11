# Task-ID: TID-20260411-root-scripts-cleanup

## Summary
- Title: 整理根 package.json 重复脚本
- Date: 2026-04-11
- Level: trivial
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 收敛根 `package.json` 的 repo 级脚本入口，删除重复/失效别名，让当前开发、测试、打包命令只保留一套清晰口径。
- In-scope: 根 `package.json` scripts、依赖这些命令名的当前 CI 与 README/RepositoryGuidelines/CodeMap/CHANGELOG、当天 task 文档。
- Out-of-scope: `apps/desktop/package.json` 子应用脚本、Tauri 打包逻辑本身、历史 spec/logs 的追溯性记录。
- Assumptions: 根 scripts 的职责是 repo 级 convenience layer，不应继续保留 Electron 时代遗留的同义入口；当前有效自动化只依赖 `npm test` 和测试 coverage workflow。
- Risks: 删除旧别名后，少量本地习惯命令会失效，需要让当前有效文档同步切到新入口。
- Interaction impact: none
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 让根级 npm 命令入口满足“少而明确”：短入口只保留 `dev/build/typecheck`，其余通过 `desktop:*`、`site:*`、`test:*` 暴露。

## Scope
- In-scope:
  - 精简根 `package.json` scripts
  - 把 `coverage/tdd` 收敛为 `test:coverage/test:watch`
  - 移除未安装依赖 `pinst` 对应的 publish scripts
  - 同步 `README.md`、`docs/RepositoryGuidelines.md`、`docs/CodeMap.md`、`docs/CHANGELOG.md`
  - 同步 `.github/workflows/tests.yml`
- Out-of-scope:
  - 新增发布自动化
  - 修改 `apps/desktop` 子应用命令
  - 清理 `package.json` 中的历史依赖和 metadata

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `package.json`
  - `.github/workflows/tests.yml`
- Related docs/specs/logs reviewed:
  - `README.md`
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `docs/logs/2026-04-11.md`
  - `docs/plans/2026-04-11.md`
- Why these are sufficient:
  - 已覆盖根级命令真源、当前唯一直接消费 coverage 脚本的 CI，以及所有当前对外生效的命令说明入口，足以完成 repo 级脚本收敛而不触碰子应用运行逻辑。

## Acceptance Criteria (AC)
- AC1: 根 `package.json` 不再保留 `start/pack/dist/test-single/prepublishOnly/postpublish` 这类重复或失效脚本，测试命令收敛为 `test` / `test:coverage` / `test:watch`。
- AC2: 当前有效文档只介绍保留后的脚本入口，并明确根级短入口与显式命名空间的边界。
- AC3: 当前 CI 覆盖率 workflow 与新的脚本名保持一致。
- AC4: `npm test`、`npm run typecheck`、workflow docs validator 通过。

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
- service_impact: 仅限根 npm scripts、CI 命令名与文档命令说明
- touches_running_service: no
- backup_required: no
- backup_plan: 通过 package.json parse、脚本 grep、`npm test`、`npm run typecheck` 与 workflow docs validator 验证
- rollback_plan: 回退 `package.json`、`.github/workflows/tests.yml` 与相关 docs
- destructive_operations: none
- operator_approval_required: no
- rationale: 本轮不改运行中服务、数据、权限或外部系统，只整理 repo 级命令入口

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 收敛根 `package.json` 的 scripts 集合
  - DoD: 重复/失效脚本移除，测试命令归并，保留命名层次清晰
- [x] Task-2: 同步文档与 CI
  - DoD: README、RepositoryGuidelines、CodeMap、CHANGELOG 与 tests workflow 全部切到新口径

## Evidence Plan (UI / E2E)
- Evidence required: no
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260411-root-scripts-cleanup/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading:
  - empty:
  - error:
  - disabled:
  - success:

## Observability / Debug Plan
- Logs: 依赖 `npm`/Vitest/typecheck/stdout 与 workflow docs validator 输出
- Error codes: 不新增
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 本地若仍习惯使用 `npm start` 或 `npm run pack/dist`，会在切换后命中“missing script”
  - 若 CI 未同步到新脚本名，会导致 coverage job 直接失败
- Rollback plan:
  - 回退根 `package.json`、`.github/workflows/tests.yml` 与文档中的命令名即可恢复

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 审计根 scripts 与引用方，区分重复别名、低价值脚本和仍在使用的命令
  2. 收敛根 `package.json`，保留短入口与显式命名空间
  3. 同步 README / docs / tests workflow
  4. 执行验证并完成 logs/plans/spec 收口

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: no
- Approved: N/A（trivial 默认直行；如需审批请手动填写）
