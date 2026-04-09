# Task-ID: TID-20260408-adaptive-reminder-state-machine

## Summary
- Title: 设计并实现低打断久坐提醒状态机
- Date: 2026-04-08
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 把久坐提醒从“固定时间一到就打断”改成“到点后先判断是否适合提醒，尽量避开用户正在输入或深度专注的时段，再在合适空档投递”。
- In-scope: Tauri host 的 break delivery 状态机；等待空档时的状态/动作文案；必要的隐藏阈值设置与 Rust 测试；本任务 docs、变更日志和证据。
- Out-of-scope: 重做设置页布局；新增 OS 级全局键盘监听；修改 break window CTA 或 strict/manual finish 规则；回填 Electron legacy 的同等功能。
- Assumptions: 现有 `idle_ms` 已足以作为“当前仍在持续输入/操作”的近似代理；`focus / DND / app exclusion / natural break` 继续作为强阻塞态；当前 session 无需新增依赖或权限。
- Risks: 仅靠 idle 阈值无法 100% 识别所有专注场景；等待空档时间过长时如果完全沉默，用户可能感知不到提醒已到期；因此需要一层只发送一次的 soft nudge。
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: break 到点时若用户仍在持续输入，Pauza 不立刻弹出 break window，而是进入等待空档状态；一旦检测到短暂停顿，再开始 break。
- Fallback / secondary flow: 若到点时用户已经处于短空档，则仍立即开始 break；若进入 focus / DND / app exclusion / natural break，则沿用原有阻塞与重置逻辑。
- User-visible boundary: 影响 tray 状态文案、主设置页状态摘要、OS notification 文案和 break window 的出现时机；不改设置页结构和 break 窗口视觉层。
- Key visible states / transitions: `scheduled -> due-but-protected -> soft-nudged -> break-running`，以及 `blocked -> reset-schedule`。

## Goal
- 让提醒更像“体感聪明的助手”，而不是“只会看表的闹钟”。

## Scope
- In-scope:
  - 在 `apps/desktop/src-tauri/src/state.rs` 中引入显式的“等待空档 / 已温和提醒”中间态。
  - 让 due break 在用户活跃输入时先挂起，不直接开 break window。
  - 当 idle gap 达到阈值时才开始 break；若等待过久，仅发一次 soft nudge。
  - 调整 tray / snapshot 状态文案与 locale 文案，使用户看得出当前是在“等空档”而不是普通倒计时。
  - 补齐 Rust 单测、Evidence README 和文档。
- Out-of-scope:
  - 在设置页新增可编辑控件。
  - 修改 break prompt UI、动作按钮文案或视觉主题。
  - 新增全局输入捕获、埋点服务或外部通知渠道。

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src-tauri/src/engine.rs`
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src/App.tsx`
  - `app/breaksPlanner.js`
  - `app/utils/naturalBreaksManager.js`
  - `app/utils/defaultSettings.js`
  - `apps/desktop/src/locales/zh-CN.json`
  - `apps/desktop/src/locales/en.json`
- Related docs/specs/logs reviewed:
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `docs/Architecture.md`
  - `docs/UI.md`
  - `docs/SettingsInventory.md`
  - `docs/logs/2026-04-08.md`
  - `docs/plans/2026-04-08.md`
- Why these are sufficient:
  - 已覆盖当前默认运行时的真实调度链路、旧 Electron 的节奏参考实现、前台消费的 snapshot 契约，以及当日已有执行上下文；本轮不涉及新的平台能力或独立 UI 系统。

## Acceptance Criteria (AC)
- AC1: 到点时若 `idle_ms` 仍低于机会阈值，Pauza 不立刻打开 break window，而是保持 break pending 并进入等待空档状态。
- AC2: pending break 在检测到短暂停顿后才开始；若等待超过配置阈值，只发送一次 soft nudge，不会每秒或每 tick 重复提醒。
- AC3: tray / snapshot / `last_action` 能明确区分普通倒计时、等待空档和已 soft nudge 三种状态。
- AC4: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` 和 `python3 scripts/validate_workflow_docs.py --mode manual` 通过。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen for implementation
- Primary flow: 用户专注输入时到点，系统只把 break 标记为 due，不弹窗口；检测到 `8~20s` 量级的短暂停顿后再开始 break。
- Fallback / secondary flow: 到点时本就空闲则直接开始；若等待期间进入 DND / focus / app exclusion / natural break，则走现有阻塞/重置路径。
- Interaction authority / ownership boundary: 仅调整“提醒何时真正投递”的运行时策略和对应状态文案；不改设置页结构、break CTA、strict/manual finish 和 tray 菜单项。
- Visible entrypoints / handoff cues: 设置页顶部状态摘要、tray 状态详情、OS notification 文案、break window 出现时机。
- In-scope interactions: 固定节奏倒计时、等待空档、soft nudge、break 自动开始。
- Out-of-scope interactions: 用户手动 pause/resume/focus、skip/postpone/finish 的命令面语义；这些动作只需继续与新状态机兼容。
- Interaction acceptance criteria: 用户在持续输入时不再被 modal break 突然打断；仍能在合适空档收到提醒；不会进入无穷重复通知。

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,ui_designer,architect,coder,tester,scribe,evidence_collector,reality_checker
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限本地调度状态机、tray/status 文案与提醒投递时机。
- touches_running_service: no
- backup_required: no
- backup_plan: 以 Git diff、现有 Tauri Rust 单测、`cargo check` 和 docs 变更为回滚边界。
- rollback_plan: 回退 `apps/desktop/src-tauri/src/state.rs`、locale 文案和本任务 docs，即可恢复“到点即提醒”的旧行为。
- destructive_operations: 用新的 pending-delivery 状态机替换默认 break delivery 逻辑。
- operator_approval_required: no
- rationale: 本轮不涉及运行中服务、数据结构迁移、历史重写、外部成本或新增依赖。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 在 `state.rs` 引入 due-hold / soft-nudge 状态机
  - DoD: due break 能在活跃输入时挂起，等待空档后再投递，且保持与 focus/DND/app exclusion/natural break 兼容。
- [x] Task-2: 更新 locale 与 snapshot 状态文案
  - DoD: 设置页状态摘要、tray 详情和 `last_action` 可区分“普通倒计时 / 等待空档 / 已温和提醒”。
- [x] Task-3: 补齐 Rust 单测与 Evidence README
  - DoD: 核心状态迁移有自动化测试覆盖，Evidence README 说明主路径与验证证据。
- [x] Task-4: 完成 docs / changelog / logs / plans 结案
  - DoD: workflow 文档校验通过且无 `TBD/INIT` 占位残留进入 DONE。

## Evidence Plan (UI / E2E)
- Evidence required: partial  <!-- yes | no | partial -->
- Owner: orchestrator（single-agent-fallback）
- Artifact path: docs/specs/TID-20260408-adaptive-reminder-state-machine/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A（无新增加载态）
  - empty: N/A（无空数据态）
  - error: postpone/skip/finish 语义保持不变，仅以测试覆盖不回归
  - disabled: blocker 生效时仍不投递 break
  - success: due-hold、idle opportunity release、soft nudge once 三条主路径的测试与状态文本证据

## Observability / Debug Plan
- Logs: 不新增持久化日志系统，复用 `DesktopSnapshot.status/statusDetail` 与 `last_action` 作为用户可感知调试面；关键状态新增等待空档/soft nudge 文案。
- Error codes: N/A
- Trace/metrics (optional): N/A
- Debug flags (optional): 预留为隐藏 settings 字段阈值，不新增独立 debug flag。

## Risks & Rollback
- Risks:
  - `idle_ms` 只能近似表示“当前没有输入”，无法区分思考停顿与真实离开。
  - 若机会阈值过短，可能仍会在用户短暂停顿时打断；过长则可能让 break 长时间挂起。
  - 挂起中的 break 只保留一个 outstanding item，极长的连续工作会导致后续节奏暂不推进。
- Rollback plan:
  - 回退 `state.rs` 的 pending-delivery 逻辑和 locale 文案。
  - 重新运行 Rust tests / cargo check / workflow docs validator，确认恢复旧行为。

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 补齐本任务 spec / testplan / daily logs/plans 的边界和验收标准。
  2. 在 Tauri `PauzaSettings` 与 `RuntimeState` 中加入 adaptive delivery 所需阈值和 pending 状态字段。
  3. 修改 `tick()`、状态文案和 notification 逻辑，使 due break 先等待空档，再择机投递，并在等待过久时只 soft nudge 一次。
  4. 增补 Rust 单测，覆盖主状态迁移与边界行为。
  5. 跑验证命令，补齐 evidence / changelog / architecture / UI / settings inventory / logs / plans。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: user direct request in-thread（moderate 任务按 validator 要求显式记录审批位，当前以用户线程内直接需求作为批准依据）
