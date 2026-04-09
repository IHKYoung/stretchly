# Task-ID: TID-20260408-reminder-mode-simplification

## Summary
- Title: 收敛提醒模式并实现智能强制自然休息模型
- Date: 2026-04-08
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 把当前 Tauri 端“adaptive + strict + style”混合模型，真正收敛为用户能理解且一致的三件事：智能提醒、强制提醒、自然休息。
- In-scope:
  - 新增统一的 `reminder_mode` 设置真源。
  - 把 `强制提醒` 实现为“到点即打断 + break 期间不可跳过/延后/关闭”。
  - 把 `智能提醒` 实现为“到点后等待短空档再开始”，移除 soft nudge。
  - 把 `idle_ms` 探测从 `natural_breaks` 开关中解耦。
  - 更新设置页、locale、测试、Architecture/UI/SettingsInventory 与 task docs。
- Out-of-scope:
  - 回填 Electron legacy 同步功能。
  - 引入更复杂的专注推理或内容级输入判断。
  - 新增 per-break-kind 的 reminder mode。
- Assumptions:
  - 当前产品只需要一个全局 reminder mode。
  - `强制提醒` 既代表定时打断，也代表严格不可绕过。
  - `自然休息` 继续只是“长时间离开后重置节奏”的规则。
- Risks:
  - 若迁移逻辑没处理旧 strict 设置，已存在用户配置会被意外改回智能提醒。
  - 若删除 `break_prompt_style` 时没有保留稳定的默认 window profile，可能引起 break window 表现回归。
  - 强制提醒语义变严后，现有 postpone 配置在该模式下将失效，需要确保运行时行为和 UI 文案一致。
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: 用户在设置页只看到 `智能提醒 / 强制提醒 / 自然休息`；到点后智能提醒等待空档，强制提醒则立即严格打断。
- Fallback / secondary flow: 仍保留 pause / focus / DND / app exclusion / natural break 阻塞逻辑；break window 只剩单一默认 window profile，不再存在独立 break style 设置。
- User-visible boundary: 设置页、tray 状态摘要、休息开始时机、break 窗口是否可关闭/跳过/延后。
- Key visible states / transitions: `Timing -> WaitingForOpportunity -> BreakActive`、`Timing -> BreakActive`、`Timing/Waiting -> NaturalBreak`。

## Goal
- 让提醒策略和用户心智重新对齐，去掉重复设置和隐藏的实验性分支。

## Scope
- In-scope:
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src-tauri/src/platform.rs`
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/locales/{zh-CN,en}.json`
  - `docs/{Architecture,UI,SettingsInventory,CodeMap}.md`
  - 本任务 specs、daily logs/plans
- Out-of-scope:
  - `app/**` legacy Electron 设置页与运行时
  - 新增任何外部依赖或系统权限
  - break window 视觉重做

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src-tauri/src/platform.rs`
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/locales/{zh-CN,en}.json`
- Related docs/specs/logs reviewed:
  - `docs/ReminderScheduling.md`
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `docs/Architecture.md`
  - `docs/SettingsInventory.md`
  - `docs/specs/TID-20260408-adaptive-reminder-state-machine/plan.md`
  - `docs/specs/TID-20260408-reminder-state-machine-doc/plan.md`
  - `docs/plans/2026-04-08.md`
  - `docs/logs/2026-04-08.md`
- Why these are sufficient:
  - 已覆盖当前 Tauri host 的真实调度链路、设置页真源、当前 adaptive/strict 设计遗留，以及用户刚确认的新目标文档；本轮改动范围不超出这些模块。

## Acceptance Criteria (AC)
- AC1: 设置页只暴露 `智能提醒 / 强制提醒 / 自然休息` 三个核心概念，不再暴露独立 strict mode 或 break style 设置，主实现里也不再保留 `break_prompt_style`。
- AC2: `state.rs` 的提醒投递逻辑收敛为：智能提醒等待空档，强制提醒到点即开始，且不再发送 soft nudge。
- AC3: `platform.rs` 始终探测 `idle_ms`，不再与 `natural_breaks` 开关耦合。
- AC4: 强制提醒开始后的 break 不能被关闭、跳过或延后。
- AC5: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`、`npm --prefix apps/desktop run build` 与 `python3 scripts/validate_workflow_docs.py --mode manual` 通过。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen for implementation
- Primary flow: 用户在设置页选择 `智能提醒 / 强制提醒`；smart 模式下到点后等空档，forced 模式下到点直接严格开始 break。
- Fallback / secondary flow: `自然休息` 继续只做重置；DND / focus / pause / app exclusions 仍优先阻塞。
- Interaction authority / ownership boundary: 只改提醒模型、break 动作权限和设置页入口，不重做 break 视觉层。
- Visible entrypoints / handoff cues: `App.tsx` 偏好页 segmented control、break window CTA、tray/status 文案。
- In-scope interactions: reminder mode 切换、waiting for opportunity、forced break 动作禁用。
- Out-of-scope interactions: Electron legacy、break window 视觉改版、额外智能策略档位。
- Interaction acceptance criteria: 用户不再看到重复的 strict 设置；强制提醒真正严格，智能提醒只做等待空档。

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
- service_impact: 仅限本地提醒投递逻辑、设置页入口和 break 可绕过性。
- touches_running_service: no
- backup_required: no
- backup_plan: 以 Git diff、Rust tests/check、前端 build 和 workflow docs validator 为回滚边界。
- rollback_plan: 回退 `state.rs`、`platform.rs`、`App.tsx`、locale 与 docs 改动即可恢复现状。
- destructive_operations: 替换当前 adaptive/strict 设置模型，并收紧强制提醒下的 break 动作权限。
- operator_approval_required: no
- rationale: 不涉及数据迁移、权限、网络副作用或新增依赖；用户已在线确认目标模型。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 收敛 host 设置真源与调度状态机
  - DoD: `reminder_mode` 落地，soft nudge 移除，强制提醒严格生效，idle 探测解耦 natural breaks。
- [x] Task-2: 收敛设置页和用户文案
  - DoD: 设置页只暴露新模型，locale 文案与 snapshot 状态一致，`break_prompt_style` 从主实现和设置 schema 中移除。
- [x] Task-3: 补齐测试与长期文档
  - DoD: 自动化测试覆盖关键迁移路径，Architecture/UI/SettingsInventory 与 task docs 同步。

## Evidence Plan (UI / E2E)
- Evidence required: partial  <!-- yes | no | partial -->
- Owner: orchestrator（single-agent-fallback）
- Artifact path: docs/specs/TID-20260408-reminder-mode-simplification/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: 旧 strict 设置迁移后不应产生反序列化错误或 UI 保存错误
  - disabled: 强制提醒下 skip/postpone/window close 失效
  - success: 智能提醒等待空档、强制提醒立即严格打断、自然休息重置三条主路径

## Observability / Debug Plan
- Logs: 复用 `last_action`、`DesktopSnapshot.status/statusDetail` 与 tests 作为主要调试面。
- Error codes: N/A
- Trace/metrics (optional): N/A
- Debug flags (optional): 不新增；隐藏阈值仅保留一个全局 idle opportunity 秒数。

## Risks & Rollback
- Risks:
  - 旧 settings.json 迁移逻辑可能把已有 strict 用户错误降级成智能提醒。
  - 强制提醒若仍允许 postpone，会与产品定义冲突。
  - `break_prompt_style` 移除后若 shell 没有落到稳定默认 profile，可能导致 window 模式 break 表现回归。
- Rollback plan:
  - 回退 host、设置页和 locale 改动，重新运行 Rust/前端构建与 workflow docs validator。

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 为 Tauri settings 引入 `reminder_mode`，并在加载 settings.json 时兼容旧 strict 字段迁移。
  2. 简化 tick 状态机：去掉 soft nudge，保留等待空档主路径，并让强制提醒直接严格开始。
  3. 把 `idle_ms` 探测从 `natural_breaks` 开关里解耦。
  4. 更新设置页和 locale，只保留用户认可的三项模型。
  5. 补齐测试与长期文档，执行验证。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: user direct request in-thread（用户已连续确认新模型并明确要求“可以，那就来改吧”）
