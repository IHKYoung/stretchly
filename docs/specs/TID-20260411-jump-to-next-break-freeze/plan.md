# Task-ID: TID-20260411-jump-to-next-break-freeze

## Summary
- Title: 修复顶部菜单跳到下一次休息导致卡死
- Date: 2026-04-11
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 修复当前顶部菜单选择“跳到下一次休息”后应用卡死的问题，优先收敛 tray 宿主刷新时机而不是重写 break 调度逻辑。
- In-scope:
  - `apps/desktop/src-tauri/src/shell.rs` 的 tray `on_menu_event` 和刷新链路
  - 必要的 Rust 编译/测试验证
  - 本任务 docs、daily plans/logs 与 `docs/CHANGELOG.md`
- Out-of-scope:
  - `state.rs` 的 break planner / schedule 规则
  - React 前台和 locale 内容
  - 新增设置项或新宿主能力
- Assumptions:
  - 根因是 tray 菜单点击后仍同步 `refresh_tray()`，该路径内部 `set_menu()` 与主线程阻塞桥接在 macOS 上触发宿主卡死/假死
  - `skip_to_next_scheduled_break` / `skip_to_long_break` 本身没有引入状态机死循环
- Risks:
  - 若真实根因在 break window 展示路径，本轮仅改刷新时机可能不足
  - 延迟刷新若设计不当，tray 状态会有短暂滞后
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 把 tray 菜单点击后的宿主刷新从当前点击临界区拆开，避免 `Jump to break` 这类动作在菜单关闭前重建原生 menu。

## Scope
- In-scope:
- `apps/desktop/src-tauri/src/shell.rs`
- `docs/specs/TID-20260411-jump-to-next-break-freeze/*`
- `docs/{plans,logs}/2026-04-11.md`
- `docs/CHANGELOG.md`
- Out-of-scope:
- `apps/desktop/src-tauri/src/state.rs`
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/locales/messages/*.json`

## Source Basis (Read Before Code)
- Related code/files reviewed:
- `apps/desktop/src-tauri/src/shell.rs`
- `apps/desktop/src-tauri/src/state.rs`
- `apps/desktop/src-tauri/src/engine.rs`
- `apps/desktop/src-tauri/src/commands.rs`
- `/Users/changkunyang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tauri-2.10.3/src/tray/mod.rs`
- `/Users/changkunyang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tauri-2.10.3/src/menu/{mod,normal}.rs`
- Related docs/specs/logs reviewed:
- `docs/RepositoryGuidelines.md`
- `docs/CodeMap.md`
- `docs/ReminderScheduling.md`
- `docs/specs/TID-20260403-macos-tray-menu-crash/{README,arch,plan,testplan,evidence/README.md}`
- `docs/logs/2026-04-03.md`
- `docs/plans/2026-04-10.md`
- Why these are sufficient:
- 已覆盖 tray action 分发、调度状态机、后台 tick、Tauri/muda 主线程菜单桥接实现，以及仓库里前一轮 macOS tray 生命周期修复记录，足以收敛当前 root cause 并局部修复。

## Acceptance Criteria (AC)
- AC1: tray 顶部菜单点击 skip 动作后，不再在同一路径立即整棵 `set_menu()` 重建。
- AC2: `skip-scheduled` / `skip-long-break` / 其他 tray action ids 与既有调度语义保持不变。
- AC3: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`npm --prefix apps/desktop run build` 与 `python3 scripts/validate_workflow_docs.py --mode manual` 通过。

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
- Required Roles: orchestrator,scribe,architect,coder,tester,reality_checker
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限 tray 菜单动作后的宿主刷新时机与对应 docs
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 Rust/前端构建、workflow docs validator 与 tray 生命周期源码审查
- rollback_plan: 回退 `apps/desktop/src-tauri/src/shell.rs` 与本任务 docs
- destructive_operations: none
- operator_approval_required: no
- rationale: 纯本地桌面端宿主修复，不涉及数据、权限、外部副作用或线上服务

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 调整 tray 菜单动作后的刷新策略
  - DoD: 菜单点击路径不再立即整棵重建 tray menu，且 state / shortcut 契约保持不变
- [x] Task-2: 完成验证与文档闭环
  - DoD: Rust/前端验证通过，specs / daily docs / changelog 不保留占位

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260411-jump-to-next-break-freeze/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading:
  - empty:
  - error:
  - disabled:
  - success:

## Observability / Debug Plan
- Logs:
- 沿用现有 `last_action`、tray snapshot 和 Tauri/muda 主线程桥接
- Error codes:
- N/A
- Trace/metrics (optional):
- N/A
- Debug flags (optional):
- N/A

## Risks & Rollback
- Risks:
- 若卡死源头是 break window 打开而不是 menu rebuild，本轮修复可能只能覆盖部分场景
- Rollback plan:
- 回退 `shell.rs` 中 tray 菜单动作刷新时机调整，并重新执行构建/validator

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 将 tray 菜单动作后的强制 `refresh_tray()` 改为延迟且按需刷新。
  2. 保持 shortcut / settings / engine 的刷新路径不变，避免把 tray 专属 workaround 扩散到其它入口。
  3. 跑 Rust/前端验证并更新 docs/changelog。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: yes（用户已明确要求修复该 bug，且本任务不涉及高风险动作）
