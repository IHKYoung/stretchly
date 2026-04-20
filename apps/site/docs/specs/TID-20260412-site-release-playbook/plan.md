# Task-ID: TID-20260412-site-release-playbook

## Summary
- Title: 沉淀站点 release 一键发布流程
- Date: 2026-04-12
- Level: trivial
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 将本次站点 release 发布经验固化为“一条命令 + 一份手册”的可复用能力，避免下次重复手工更新 pinned URL、检查 gh 状态和创建 GitHub release。
- In-scope: 新增本地发布脚本、发布 playbook、README / Guidelines / CodeMap 更新、忽略规则与 workflow 文档。
- Out-of-scope: 自动生成 workflow task docs、自动 commit/push、支持多平台或多资产矩阵发布。
- Assumptions: 目标资产继续遵循 `Pauza_<version>_aarch64.dmg` 命名；发布仓库仍是 `IHKYoung/Pauza`；`gh auth login` 已完成。
- Risks: 正则替换目标文件时若结构改动过大，脚本可能失效；如果用户传入错误资产路径，release 可能被错误覆盖。
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 把“更新站点下载入口 + 调用 GitHub release”这套动作收敛成稳定、可回放、可 dry-run 的命令行工作流。

## Scope
- In-scope: `.gitignore`、`scripts/publish_site_release.py`、`docs/ReleasePlaybook.md`、`README.md`、`docs/RepositoryGuidelines.md`、`docs/CodeMap.md`、`docs/CHANGELOG.md`
- Out-of-scope: 站点 UI 行为、release 资产打包本身、上层 monorepo 的 CI/CD 编排

## Source Basis (Read Before Code)
- Related code/files reviewed: `index.html`、`download/targets.js`、`README.md`
- Related docs/specs/logs reviewed: `docs/RepositoryGuidelines.md`、`docs/CodeMap.md`、`docs/specs/TID-20260412-site-release-0-1-2/*`、`scripts/validate_workflow_docs.py`
- Why these are sufficient: 已覆盖发布目标文件、现有站点下载约束、上一次 release 任务结论与本仓 workflow 校验规则。

## Acceptance Criteria (AC)
- AC1: `python3 scripts/publish_site_release.py --asset /abs/path/Pauza_<version>_aarch64.dmg` 能自动推导版本并更新 `index.html` 与 `download/targets.js` 的 pinned URL。
- AC2: 脚本支持 `gh release create` 与“已存在 release 时的 `upload --clobber`”两条路径。
- AC3: README / Repository Guidelines / Release Playbook 清晰记录标准命令与常见变体。
- AC4: 新增 `.gitignore` 后，`__pycache__` 与 `.playwright-mcp` 不再污染工作树。

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
- Required Roles: scribe,coder,tester
- Delegation Policy: after the user grants orchestration authority, `orchestrator` may autonomously `spawn_agent`
- Automatic Delegation Triggers: `moderate/complex` | long-running task | independent sidecar tasks
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: no；当前任务只新增本地脚本、文档与忽略规则。
- touches_running_service: no
- backup_required: no
- backup_plan: 需要回退时可直接 `git revert` 本次能力沉淀 commit。
- rollback_plan: 删除 `publish_site_release.py`、`ReleasePlaybook.md`、`.gitignore` 规则并回退文档修改。
- destructive_operations: none
- operator_approval_required: no
- rationale: 不引入新依赖、不改数据、不触发新的线上副作用；只把未来发布步骤封装为本地命令。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 实现发布脚本
  - DoD: 支持 asset 版本解析、站点 pinned URL 更新、release create/upload、`--skip-release` 与 `--dry-run`
- [x] Task-2: 补齐发布手册与仓库入口文档
  - DoD: README、Repository Guidelines、CodeMap 与 Release Playbook 全部可直接指导下次复用
- [x] Task-3: 补齐忽略规则与 workflow 文档
  - DoD: `.gitignore` 生效，task docs / daily plan / log / changelog 全部完成

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260412-site-release-playbook/evidence/
- Interaction validation note: 本次无 UI/交互变更，仅保留命令行验证结果。
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: 脚本对 asset 路径错误、命名不匹配、gh 未认证等情况直接退出
  - disabled: N/A
  - success: `--dry-run` 与 `--skip-release` 路径都能跑通

## Observability / Debug Plan
- Logs: 标准输出打印 `[INFO]` / `[UPDATE]` / `[SKIP]` / `[DONE]` 与底层 `gh` 命令。
- Error codes: 通过非零退出码传播错误。
- Trace/metrics (optional): 不新增。
- Debug flags (optional): `--dry-run`

## Risks & Rollback
- Risks: 文件结构变化导致正则替换失效；误把错误 asset 覆盖到已有 release；`gh` 未登录时脚本中断。
- Rollback plan: 回退本次 commit；重新运行脚本用正确 asset 覆盖 release；必要时手工恢复站点 pinned URL。

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 新增本地发布脚本与忽略规则。
  2. 把标准命令和变体用法写进 README / Release Playbook。
  3. 用当前 `0.1.2` dmg 执行 `--skip-release` 与 `--dry-run` 验证脚本路径。
  4. 跑 workflow validator 并提交。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: no  <!-- yes | no；仅高风险动作写 yes -->
- Approved: no
