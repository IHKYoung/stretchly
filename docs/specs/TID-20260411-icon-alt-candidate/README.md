# Task-ID: TID-20260411-icon-alt-candidate

## Meta
- Title: 透明化网页图标并导出桌面端备选图标
- Date: 2026-04-11
- Level: trivial  <!-- trivial | moderate | complex -->
- Lane: fast    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-11.md
- Log (daily): ../../logs/2026-04-11.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 直接修改 `apps/site/favicon.svg`，把网页端当前这版图标转为透明底，避免维护两套几何。
- 在 `apps/desktop/src-tauri/icons/` 中新增不接线的备选图标素材，而不是替换现有 `icon.png` / `icon.icns` 链路。
- 开口方向不再通过 `stroke-dasharray` 碰角度，改为明确弧线路径，稳定指向西北 45 度附近。

## Governance Notes
- Requirement Brief: 用户希望把网页端现有 `favicon.svg` 这版图标抽出来当作桌面端的另一个候选方案；要求去掉背景、保持透明，并把开口方向再向左转一些，使其朝向西北 45 度。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: service_impact=仅限图标素材文件更新与桌面端 icons 目录新增候选资产；touches_running_service=no；backup_required=no；backup_plan=通过文件 diff、图像尺寸/透明通道检查与 workflow docs validator 验证；rollback_plan=回退 `apps/site/favicon.svg` 与新增候选图标文件；destructive_operations=none；operator_approval_required=no；rationale=不改当前正式打包图标链路，不涉及运行中服务、数据、权限或外部付费动作。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: BLOCKED: 当前 session 未获用户显式 delegation 授权，遵循上层工具策略不调用 `spawn_agent`；本轮由单 agent 在限定范围内完成图标素材调整、导出与文档闭环。
- Retention Decision: keep

## Notes
- 新增的桌面端候选图标素材不会自动参与当前 Tauri 打包；它只是一个备选资产，便于后续替换或继续打磨。
