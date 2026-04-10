# Task-ID: TID-20260410-pre-break-heads-up-restoration

## Summary
- Title: 恢复并重构提前提示为可见 heads-up 状态
- Date: 2026-04-10
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 检查“提前提醒”当前是否仍在实际运行；如果只是系统通知链残留但体感无效，就把它恢复为真正可见的 heads-up 状态，而不是保留一个名存实亡的设置。
- In-scope:
- 复核 `state.rs`、`engine.rs`、设置页文案和调度文档中的 pre-break heads-up 链路。
- 将 heads-up 改为运行态可见状态，并同步到 `DesktopSnapshot.status/status_detail` 以及 tray 文本。
- 为系统通知失败补日志，避免继续静默失败。
- 更新 locale registry 和相关设计文档。
- Out-of-scope:
- 不新增独立的预提醒弹窗、悬浮条或系统权限引导。
- 不改 break 的节奏计算、due 后 smart reminder 的递减阈值曲线或 forced 行为。
- 不改 break CTA、托盘菜单结构或多屏显示策略。
- Assumptions:
- 用户说“没有用到”更接近产品可见性失效，而不是字段完全失联。
- 对 Pauza 这种常驻 tray 的桌面工具，更优雅的 heads-up 主信号应是运行态和托盘可见 cue，而不是只靠一次性系统通知。
- 继续保留系统通知作为辅助并无问题，但它不应是唯一有效出口。
- Risks:
- 若 heads-up 文案过强，可能和正式 break 的等待空档提示混淆。
- tray 是系统级表面，自动化证据较弱；最终真实观感仍需用户本机确认。
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: break 还未到点但进入 lead time 时，设置页状态、tray 菜单与 tooltip 会显示 `即将开始 / Up next`；若系统通知可用，仍会额外投递一次 heads-up。
- Fallback / secondary flow: 系统通知失败时，运行态 heads-up 仍可见；break 到点后继续按 smart/forced 既有逻辑进入 `等待空档` 或直接开始。
- User-visible boundary: 仅影响 break 开始前的提前提示语义与可见状态，不影响 break 页面本体。
- Key visible states / transitions:
  - scheduled
  - heads-up window entered
  - optional OS notification
  - due
  - waiting for opportunity or break active

## Goal
- 把“提前提醒”恢复为用户真的能感知到的功能，并明确它的主信号是 heads-up 状态而不是单次系统通知。

## Scope
- In-scope:
- `apps/desktop/src-tauri/src/state.rs`
- `apps/desktop/src-tauri/src/engine.rs`
- `apps/desktop/src/locales/messages/en.json`
- `apps/desktop/src/locales/messages/zh-CN.json`
- `apps/desktop/src/locales/registry.generated.json`
- `docs/specs/TID-20260410-pre-break-heads-up-restoration/*`
- `docs/plans/2026-04-10.md`
- `docs/logs/2026-04-10.md`
- `docs/Architecture.md`
- `docs/UI.md`
- `docs/ReminderScheduling.md`
- `docs/SettingsInventory.md`
- `docs/CHANGELOG.md`
- Out-of-scope:
- `apps/desktop/src/App.tsx` 的布局结构
- `apps/desktop/src/lib/break-prompt.ts`
- `apps/desktop/src-tauri/src/shell.rs`
- 新的 Tauri 命令、权限流程或外部依赖

## Source Basis (Read Before Code)
- Related code/files reviewed:
- `apps/desktop/src-tauri/src/state.rs`
- `apps/desktop/src-tauri/src/engine.rs`
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/locales/messages/zh-CN.json`
- `apps/desktop/src/locales/messages/en.json`
- Related docs/specs/logs reviewed:
- `docs/ReminderScheduling.md`
- `docs/SettingsInventory.md`
- `docs/Architecture.md`
- `docs/UI.md`
- `docs/logs/2026-04-09.md`
- Why these are sufficient:
- `state.rs` 已覆盖 heads-up 调度和 `DesktopSnapshot` 状态真源，`engine.rs` 是系统通知唯一出口，`App.tsx` 是设置文案唯一入口；再结合 ReminderScheduling / SettingsInventory 即可判断当前功能是“存在但体感无效”，并完成语义收口。

## Acceptance Criteria (AC)
- AC1: 在 break 到点前的 lead time 内，runtime 状态能进入明确的 heads-up 阶段，而不是只发送一次性系统通知后立刻从可见状态中消失。
- AC2: heads-up 阶段结束后，due 后的 smart reminder 仍会正常进入 `等待空档`，forced 仍会直接开始 break。
- AC3: 系统通知失败时不再静默；后台会输出可排查的错误日志。
- AC4: 设置文案与设计文档明确“提前提示”的主语义是可见 cue，不再误导为纯系统通知功能。
- AC5: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`npm --prefix apps/desktop run build` 和 `python3 scripts/sync_desktop_locales.py` 通过。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen
- Primary flow: `scheduled -> heads-up -> due -> waiting for opportunity / break active`
- Fallback / secondary flow: 若 OS 通知未显示或系统层吞掉通知，tray / settings runtime status 仍应持续表达 heads-up；due 后转入既有 break 流程。
- Interaction authority / ownership boundary: 只改 runtime heads-up 状态与设置语义，不改 break 页面布局、按钮和窗口展示形态。
- Visible entrypoints / handoff cues: 设置页状态卡、tray 标题/tooltip/菜单文本、可选系统通知。
- In-scope interactions:
- due 前 lead time 的 heads-up 文案与状态切换
- 系统通知失败日志
- due 后 smart/forced 的既有衔接
- Out-of-scope interactions:
- break 页面新增浮层
- 通知权限引导 UI
- 新的提醒声或动画
- Interaction acceptance criteria:
- due 前能看到 heads-up 状态
- due 后不再停留在 heads-up，而是转入 waiting 或 active break
- 系统通知失败不会让 heads-up 彻底失联
- Validator expectation: direct interaction 字段、Evidence required 与 Coverage 已补齐。

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,ui_designer,architect,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限 desktop runtime 的 heads-up 状态文案、系统通知失败日志与设计文档
- touches_running_service: no
- backup_required: no
- backup_plan: 以 Git diff、`cargo test`、desktop build、locale registry 重建和 docs 同步作为回归边界
- rollback_plan: 回退 `state.rs` / `engine.rs` / locale / docs 中与 heads-up restoration 相关的改动
- destructive_operations: none
- operator_approval_required: no
- rationale: 不涉及运行中服务、数据迁移、权限提升、新依赖或外部副作用

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 复核 pre-break 设置、调度与通知链路
  - DoD: 明确确认功能不是断线，而是一次性系统通知导致的低可见性问题
- [x] Task-2: 将 heads-up 恢复为运行态可见状态并补观察日志
  - DoD: `DesktopSnapshot.status/status_detail` 可表达 heads-up；`engine.rs` 对通知失败输出日志
- [x] Task-3: 同步 locale registry 与设计文档
  - DoD: 设置文案、ReminderScheduling、SettingsInventory、Architecture、UI 与 CHANGELOG 都不再把该功能描述成“只会弹通知”

## Evidence Plan (UI / E2E)
- Evidence required: partial  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260410-pre-break-heads-up-restoration/evidence/
- Interaction validation note: tray 属于系统级表面，当前主要依赖 runtime 单测、desktop build 与文档对照证明 heads-up 阶段存在；仍建议用户本机观察一次实际 tray / tooltip 观感。
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: 系统通知投递失败时仍保留 heads-up 状态，并输出 `failed to show desktop notification`
  - disabled: 关闭 microbreak/long break 提前提示时不进入 heads-up
  - success: due 前进入 `即将开始 / Up next`，due 后 smart 模式进入 `等待空档` 或直接进入 break

## Observability / Debug Plan
- Logs: `engine.rs` 在系统通知失败时输出 `failed to show desktop notification: ...`
- Error codes: 继续沿用 `Result<(), String>`；本轮不新增错误码
- Trace/metrics (optional): 无
- Debug flags (optional): 无

## Risks & Rollback
- Risks:
- heads-up 文案如果过于频繁，可能让用户误以为 break 已经开始。
- 因 tray 观感依赖系统表面，自动化验证仍偏间接。
- Rollback plan:
- 回退 `state.rs` 中的 `heads_up_kind()`、`pre_break_heads_up_detail()` 和状态分支
- 回退 `engine.rs` 的通知错误日志
- 回退 locale / 文档语义收口

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 审计提前提示设置、调度字段和通知投递链，确认是否存在真正的死逻辑。
  2. 把 heads-up 改成持久可见的 runtime 状态，并保持 due 后 smart/forced 行为不漂移。
  3. 运行 Rust 测试、desktop build、locale sync，补齐 evidence 和文档。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: yes（moderate 任务按 validator 要求记录；本轮不涉及运行中服务、破坏性操作、提权或外部副作用，由 orchestrator 在既定会话授权边界内执行）
