# Task-ID: TID-20260403-repo-cleanup-foundation

## Summary
- Title: 仓库精简第一轮：剔除非核心外围资产
- Date: 2026-04-03
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 先做一轮无争议的仓库精简，删除与产品主体无关的外围资产，为后续二次开发降低噪音。
- In-scope: 删除 Docker/Snap 辅助壳、独立调试页、本地生成缓存，并同步最少量的 docs 索引。
- Out-of-scope: 不调整 `app/**` 核心逻辑，不删除 Linux/Windows 分发主资产与构建目标，不做框架迁移。
- Assumptions: 当前阶段需要的是“保核心、降噪”，而不是立刻按某个平台重写分发链。
- Risks: 如果现在过度裁剪平台文件，会影响后续 Electron/Tauri 路径对比与迁移。
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 在不影响产品主体的前提下，先把仓库里明显无关的外围资产清掉。

## Scope
- In-scope:
  - `Dockerfile`
  - `docker-compose.yml`
  - `publish-snap.md`
  - `danger-color-tester.html`
  - 本地生成目录 `coverage/` 与 `scripts/__pycache__/`
  - `docs/CodeMap.md`、`docs/RepositoryGuidelines.md`、`docs/CHANGELOG.md`
- Out-of-scope:
  - `app/**`
  - `test/**`
  - `package.json`
  - Linux/Windows 分发文件、CI 工作流与构建资源

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `package.json`
  - `Dockerfile`
  - `docker-compose.yml`
  - `publish-snap.md`
  - `danger-color-tester.html`
  - `app/process-renderer.js`
- Related docs/specs/logs reviewed:
  - `AGENTS.md`
  - `docs/CodeMap.md`
  - `docs/RepositoryGuidelines.md`
- Why these are sufficient:
  - 已覆盖当前删除项本身、构建配置、少量资源引用点与仓库索引文档，足以判断第一轮安全删除边界。

## Acceptance Criteria (AC)
- AC1: 删除 Docker/Snap 辅助壳、独立调试页与本地生成缓存后，核心应用代码主体保持不变。
- AC2: 文档索引与仓库现状一致，且本任务的执行边界可通过 diff 与 workflow docs 校验追溯。

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
- service_impact: repo hygiene only
- touches_running_service: no
- backup_required: no
- backup_plan: 以 git diff 和 VCS 历史为边界
- rollback_plan: 恢复本任务删除的 tracked 文件，并按需重新运行 `npm run coverage`
- destructive_operations: 删除非核心仓库文件与本地生成目录
- operator_approval_required: no
- rationale: 用户已明确要求清理不必要的外围资产，本轮不触碰产品主体代码

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 识别并删除无争议的外围资产
  - DoD: Docker/Snap 辅助壳、调试页与本地生成缓存从工作区移除
- [x] Task-2: 同步 docs 与执行记录
  - DoD: CodeMap/RepositoryGuidelines/CHANGELOG 与 specs/plans/logs 全部完成更新

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260403-repo-cleanup-foundation/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading:
  - empty:
  - error:
  - disabled:
  - success:

## Observability / Debug Plan
- Logs: 通过 daily logs/plans 记录删除边界、验证命令与后续待裁剪项
- Error codes: N/A
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 误删对后续跨平台分发仍有价值的资产
- Rollback plan:
  - 用 VCS 恢复 tracked 文件；对生成目录按需重新生成

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 盘点仓库外围资产与引用关系
  2. 删除无争议文件与生成目录
  3. 同步 docs 索引与执行记录
  4. 运行 workflow 门禁校验

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: yes（用户明确要求进行删除操作）
