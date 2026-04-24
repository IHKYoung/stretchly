# Task-ID: TID-20260424-structure-cleanup

## Meta
- Title: 整理 reminder 与 break ideas 的结构残留
- Date: 2026-04-24
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-24.md
- Log (daily): ../../logs/2026-04-24.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 2026-04-24: 不做大规模拆文件；本轮只清理“兼容字段还挂在主模型里”和“`tick()` 主流程可读性不足”这两类结构残留。
- 2026-04-24: `idle_opportunity_seconds` 保留为 host 读旧配置时的兼容字段，但不再序列化回新的 settings/snapshot，也不再让前台设置模型继续携带它。
- 2026-04-24: `tick()` 中的“到点通知 / 到点开休息”分支抽成 helper，减少主调度路径里的重复判断与状态写回噪音。

## Governance Notes
- Requirement Brief: 用户在确认 break ideas 和 reminder 的新边界后，要求继续整理现有代码结构；本任务聚焦于去掉仍在主模型中漂浮的兼容字段、收敛 `state.rs` 的 due 分支、并把文档更新到当前真实实现。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: 仅影响 desktop 前台设置模型、Rust host 调度 helper、settings 序列化边界与文档；无服务影响、无外部副作用，回滚方式是恢复 `idle_opportunity_seconds` 的序列化、还原 `tick()` 分支内联实现与相关 docs。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Warmup Required Roles: architect,coder,tester,scribe
- Warmup Ready Roles: single-agent-fallback
- Warmup Agent IDs: single-agent-fallback（当前 session 未获用户显式 delegation）
- Warmup Verification: BLOCKED: 当前 session 未获用户显式授权调用 `spawn_agent`，按上层工具策略只能走 `single-agent-fallback`，因此由单 agent 在限定范围内完成结构整理与验证。
- Fallback Reason Code: N/A（仅 fallback 时填写）
- Escalation Summary: 当前 session 未获用户显式 delegation，按工具策略保持单 agent 闭环完成整理与验证。
- Retention Decision: keep

## Notes
- 已落地：
  - `apps/desktop/src/App.tsx` 不再把 `idleOpportunitySeconds` 视为前台设置模型字段
  - `apps/desktop/src-tauri/src/state.rs` 将 `idle_opportunity_seconds` 降级为“仅反序列化、不再序列化”的兼容字段
  - `state.rs::tick()` 的 due notification / due break 分支已抽到独立 helper
  - `docs/SettingsInventory.md`、`docs/Architecture.md`、`docs/CHANGELOG.md` 已同步到当前真实实现
- 已验证：
  - `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - `npm run typecheck`
  - `npm test`
  - `python3 scripts/validate_workflow_docs.py --mode manual`
