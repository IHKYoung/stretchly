# Task-ID: TID-20260411-icon-alt-candidate

## Summary
- Title: 透明化网页图标并导出桌面端备选图标
- Date: 2026-04-11
- Level: trivial
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 把网页端当前 `favicon.svg` 这版图标改成透明底，并导出到桌面端 `icons` 目录作为不接线的候选图标。
- In-scope:
  - 修改 `apps/site/favicon.svg`
  - 在 `apps/desktop/src-tauri/icons/` 新增候选图标资产
  - 最小化 docs / logs / plans 同步
- Out-of-scope:
  - 替换当前正式 `icon.png`、`icon.icns`、`icon.ico`
  - 重跑整套正式图标生成链
  - 改动站点首页布局或桌面端运行逻辑
- Assumptions:
  - 用户当前要的是“备选图标素材”，不是立即切换正式应用图标
  - `apps/desktop/src-tauri/icons/` 中新增候选文件不会影响现有打包配置
  - 本机工具可用于从 SVG 导出透明 PNG
- Risks:
  - 透明底后在浅色背景上发光圈层会更明显，需要保持足够克制
  - 开口角度如果转得太多，可能偏离现有 Pauza 视觉语义
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 在不动当前正式图标链路的前提下，做出一版透明底、开口朝西北 45 度的备选图标。

## Scope
- In-scope:
  - `apps/site/favicon.svg`
  - `apps/desktop/src-tauri/icons/icon-alt-nw45.svg`
  - `apps/desktop/src-tauri/icons/icon-alt-nw45.png`
  - `docs/specs/TID-20260411-icon-alt-candidate/*`
  - `docs/{plans,logs}/2026-04-11.md`
  - `docs/{CodeMap,CHANGELOG}.md`
- Out-of-scope:
  - `apps/desktop/src-tauri/icons/icon.png`
  - `apps/desktop/src-tauri/icons/icon.icns`
  - `graphics/generate_icon_assets.py`

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/site/favicon.svg`
  - `apps/desktop/src-tauri/icons/*`
- Related docs/specs/logs reviewed:
  - `docs/CodeMap.md`
  - `docs/CHANGELOG.md`
  - `docs/specs/TID-20260411-site-typewriter-redesign/README.md`
- Why these are sufficient:
  - 已覆盖当前网页图标几何、桌面端图标目录现状和站点最新视觉上下文，足以支撑这次候选图标导出。

## Acceptance Criteria (AC)
- AC1: `apps/site/favicon.svg` 改为透明底，且开口朝向西北 45 度附近。
- AC2: `apps/desktop/src-tauri/icons/` 下新增不接线的候选图标资产，便于后续评估。
- AC3: 候选 PNG 为透明背景，尺寸符合桌面端图标预览使用。
- AC4: workflow docs validator 通过。

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
- service_impact: 仅限图标素材文件更新与桌面端 icons 目录新增候选资产
- touches_running_service: no
- backup_required: no
- backup_plan: 通过文件 diff、图像尺寸/透明通道检查与 workflow docs validator 验证
- rollback_plan: 回退 `apps/site/favicon.svg` 与新增候选图标文件
- destructive_operations: none
- operator_approval_required: no
- rationale: 不改当前正式打包图标链路，不涉及运行中服务、数据、权限或外部付费动作

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 调整网页图标为透明底与西北朝口
  - DoD: `apps/site/favicon.svg` 几何更新完成
- [x] Task-2: 导出桌面端备选图标并校验透明通道
  - DoD: `icon-alt-nw45.svg/png` 落盘，尺寸与 alpha 检查通过

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260411-icon-alt-candidate/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading:
  - empty:
  - error:
  - disabled:
  - success:

## Observability / Debug Plan
- Logs: N/A（静态素材）
- Error codes: N/A
- Trace/metrics (optional): 使用 `sips` / `identify` 校验尺寸与 alpha
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 开口角度不符合预期
  - 透明 PNG 导出失败或 alpha 丢失
- Rollback plan:
  - 回退 `favicon.svg`
  - 删除候选图标文件

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 用明确弧线路径替换现有 favicon 的开口定义
  2. 将该版本图标复制到桌面端 icons 目录作为候选 SVG
  3. 导出透明 PNG 并做尺寸/alpha 校验

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: no  <!-- yes | no -->
- Approved: N/A（trivial 默认直行；如需审批请手动填写）
