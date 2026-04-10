# Task-ID: TID-20260409-tray-menu-live-status-and-option

## Summary
- Title: 修复托盘菜单实时状态并开放倒计时开关
- Date: 2026-04-09
- Level: moderate
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 让右键 tray 菜单中的状态时间也实时跳动，同时把顶部 tray 倒计时做成可配置开关。
- In-scope:
  - tray 菜单 `status` / `status-detail` 原地更新
  - `showTimeToBreakInTray` 设置项接入前后端
  - 英文 locale 补齐与 registry 重生成
  - 对应测试与文档
- Out-of-scope:
  - tray 菜单结构和动作
  - 调度状态机语义
  - 新增更多 tray 样式选项
- Assumptions:
  - 顶部倒计时开关只控制 tray title，不影响右键菜单状态信息
  - 既有 `showTimeToBreakInTray` 命名可直接作为统一真源
- Risks:
  - 若 menu item handle 保存方式不对，可能导致原地更新失效
  - 若设置字段默认值不兼容，老 settings 文件可能错误回退
- Interaction impact: none
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 让 tray 顶部 title 和右键菜单信息项都按各自职责更新，同时给用户一个明确的倒计时显示开关。

## Scope
- In-scope:
  - `apps/desktop/src-tauri/src/{shell,state}.rs`
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/locales/messages/en.json`
  - `apps/desktop/src/locales/registry.generated.json`
  - `docs/SettingsInventory.md`
  - 本任务 specs / plans / logs / changelog / commits
- Out-of-scope:
  - 其它语言翻译内容新增
  - tray icon 资源
  - break 页面 UI

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src-tauri/src/commands.rs`
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/locales/messages/{en,zh-CN}.json`
  - `/Users/changkunyang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tauri-2.10.3/src/menu/menu.rs`
  - `/Users/changkunyang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tauri-2.10.3/src/menu/normal.rs`
- Related docs/specs/logs reviewed:
  - `docs/SettingsInventory.md`
  - `docs/specs/TID-20260409-tray-live-countdown/*`
  - 当日 `docs/plans/2026-04-09.md` / `docs/logs/2026-04-09.md`
- Why these are sufficient:
  - 已覆盖 tray 刷新链路、settings 真源、前台设置页绑定点和 menu item 原地改文案 API。

## Acceptance Criteria (AC)
- AC1: 右键 tray 菜单展开后，`status-detail` 时间会实时变化，不再停在打开那一刻。
- AC2: 设置页提供 `showTimeToBreakInTray` 开关，关闭后顶部 tray title 立即隐藏，打开后恢复实时倒计时。
- AC3: 老 settings 文件在缺少该字段时默认保持开启，不会意外变成关闭。
- AC4: `python3 scripts/sync_desktop_locales.py`、`npm --prefix apps/desktop run typecheck`、`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`npm --prefix apps/desktop run build`、`python3 scripts/validate_workflow_docs.py --mode manual` 通过。

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
- service_impact: tray 菜单信息项、tray title 开关与对应设置持久化
- touches_running_service: no
- backup_required: no
- backup_plan: 以 locale sync、TypeScript、Rust tests、desktop build 与 docs validator 为边界
- rollback_plan: 回退 `shell.rs`、`state.rs`、`App.tsx`、locale 变更与本任务 docs
- destructive_operations: none
- operator_approval_required: no
- rationale: 宿主展示与本地设置修复，不涉及外部副作用

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 让 tray 菜单状态项原地实时更新
  - DoD: 菜单打开后状态文本也会跳秒，无需每秒重建整个菜单。
- [x] Task-2: 接入 `showTimeToBreakInTray` 设置项
  - DoD: 前后端 schema、默认值、设置页和 tray title 行为一致。
- [x] Task-3: 同步 locale / tests / docs
  - DoD: 英文文案补齐，registry 重生成，验证和文档门禁通过。

## Evidence Plan (UI / E2E)
- Evidence required: no
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260409-tray-menu-live-status-and-option/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: N/A
  - disabled: N/A
  - success: N/A

## Observability / Debug Plan
- Logs: 不新增日志，避免 tray 高频刷新带来噪声。
- Error codes: N/A
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 持有的 menu item handle 若失效，菜单文案不会实时更新
  - 设置默认值若处理错误，旧配置可能把 title 意外关闭
- Rollback plan:
  - 回退 menu item updater、settings 字段与前台开关绑定

## Sequential Phases
- phase_execution: N/A
- phase_confirmation_policy: N/A
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 接入 tray 菜单 item 原地更新机制。
  2. 将 `showTimeToBreakInTray` 接入 Rust/TS settings schema 与设置页。
  3. 重生成 locale registry，跑验证并补文档。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes
- Approved: yes（orchestrator 已按当前 session 授权路由批准执行）
