# Task-ID: TID-20260423-smart-reminder-v2-scheduling

## Summary
- Title: 重构智能提醒状态机与恢复结算
- Date: 2026-04-23
- Level: complex
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement:
  - 将当前智能提醒改成更稳定的 v2：break active 生命周期优先；pause/focus/DND/app exclusion 只冻结投递、不重置节奏；用户短暂离开时先做恢复 credit 结算，再决定是否真正开始 break。
- In-scope:
  - `state.rs` reminder host 状态机
  - `commands.rs` 中 pause/focus 相关冻结/恢复边界
  - 运行时状态文案与 locale
  - 与新状态机直接相关的 docs / tests
- Out-of-scope:
  - 新系统信号、活动强度模型、云端或 ML 判定
  - 设置页新增配置项
  - break window 视觉重构
- Assumptions:
  - 当前唯一稳定信号仍是 `idle_ms`
  - `smart / forced / naturalBreaks` 用户概念保持不变
  - 用户已确认 blocker freeze + recovery credit 的方向是合理的
- Risks:
  - 多 blocker 重叠时 due 平移重复累计
  - microbreak credit 误推进 cycle
  - 文案口径与真实状态不一致导致体感更随机
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: smart mode 下 due 后先等待空档；若用户离开足够久则进入 recovery hold，回来时自动抵扣或顺延，而不是直接弹 break。
- Fallback / secondary flow: forced mode 仍到点直接进入 break；pause/focus/DND/app exclusion 仅冻结并在解除后恢复剩余节奏。
- User-visible boundary: 设置页 runtime status、tray 状态文本、break prompt 弹出时机。
- Key visible states / transitions: heads-up -> waiting for opportunity -> recovery hold -> break active / recovery credit / full reset。

## Goal
- 让 Pauza 的“智能提醒”从一组松散阈值修补，收敛成可以解释、可验证、不会在 blocker 前后漂移的 reminder 状态机。

## Scope
- In-scope:
  - 修正 passive blocker 会 reset schedule / 打断 active break 的语义问题
  - 为 smart reminder 增加恢复 credit 与 recovery hold
  - 在 waiting 状态中补出剩余等待预算
- Out-of-scope:
  - 新增平台探测能力
  - 引入更复杂的 activity score
  - 大规模前台重构

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src-tauri/src/{state.rs,commands.rs,engine.rs,platform.rs}`
  - `apps/desktop/src/locales/messages/{zh-CN,en}.json`
- Related docs/specs/logs reviewed:
  - `docs/{RepositoryGuidelines,CodeMap,Architecture,ReminderScheduling,SettingsInventory,UI}.md`
  - `docs/logs/2026-04-08.md`
  - `docs/logs/2026-04-09.md`
  - `docs/specs/TID-20260408-adaptive-reminder-state-machine/*`
  - `docs/specs/TID-20260409-smart-reminder-wait-guard/*`
  - `docs/specs/TID-20260409-smart-reminder-decay-thresholds/*`
  - `docs/specs/TID-20260409-smart-reminder-ui-simplification/*`
- Why these are sufficient:
  - 已覆盖当前 host 调度真源、命令面、平台信号来源，以及 reminder 状态机最近几轮演进文档，足以在不引入新能力的前提下完成本轮 v2 收敛。

## Acceptance Criteria (AC)
- AC1: passive blocker 不会中断已开始的 break；用户主动 pause/focus/skip/reset 之外的 blocker 只冻结 delivery。
- AC2: pause/focus/DND/app exclusion 开始后，pending due / notification / smart wait timer 会被冻结；解除后按阻塞时长恢复，不再整轮 reset。
- AC3: smart mode 下，若用户离开达到恢复 credit 区间，不会在离开时直接开 break；返回后会自动执行 microbreak 抵扣、long break 顺延或 full reset。
- AC4: snapshot/status 能向用户解释当前是在“等待空档”还是“检测到离开并等待恢复结算”，且 waiting 状态包含剩余等待预算。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: confirmed
- Primary flow: smart mode due 后优先礼貌等待；若 idle 已达到恢复 credit 起点则保持 recovery hold，等待用户返回时结算。
- Fallback / secondary flow: forced mode 立即 break；manual pause/focus 继续作为强上下文动作，结束后恢复冻结前节奏。
- Interaction authority / ownership boundary: 本轮只改 host 派生出的状态与触发时机，不改前台结构或新加控件。
- Visible entrypoints / handoff cues: settings runtime status、tray 状态文本、last_action、break prompt 开窗。
- In-scope interactions: waiting 解释、recovery hold、credit/reset 说明、blocker freeze 恢复。
- Out-of-scope interactions: settings 布局、tray 菜单项、break CTA 视觉。
- Interaction acceptance criteria: 用户能从现有状态区分“继续等一下”“你已经离开，回来后会自动结算”“真正开始 break”。

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,architect,coder,tester,scribe,evidence_collector,reality_checker
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限本地 desktop runtime reminder 状态机、状态文案、测试与 docs。
- touches_running_service: no
- backup_required: no
- backup_plan: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`npm test`、`npm run typecheck`、`npm --prefix apps/desktop run build`、`python3 scripts/validate_workflow_docs.py --mode manual`
- rollback_plan: 回退 `state.rs`、`commands.rs`、locale 与相关 docs/specs 后重跑上述验证。
- destructive_operations: none
- operator_approval_required: no
- rationale: 无线上服务、提权、外部副作用、付费成本或数据迁移。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 重构 reminder host 状态机
  - DoD: blocker freeze / active break 优先 / recovery credit 在 `state.rs` 收敛，并与 `commands.rs` 对齐
- [x] Task-2: 补齐状态文案、测试与 docs
  - DoD: locale、单测、Architecture/UI/ReminderScheduling/CHANGELOG 与 daily docs 完整闭环

## Evidence Plan (UI / E2E)
- Evidence required: partial  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260423-smart-reminder-v2-scheduling/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A
  - empty: idle / no break scheduled 保持不变
  - error: N/A
  - disabled: forced break 锁定 / blocker freeze
  - success: waiting for opportunity、recovery hold、microbreak credited、long break deferred、full reset

## Observability / Debug Plan
- Logs:
  - 复用 `last_action` 记录 waiting、recovery hold、microbreak credited、long break deferred、natural break reset。
- Error codes:
  - 无新增错误码
- Trace/metrics (optional):
  - N/A
- Debug flags (optional):
  - 无

## Risks & Rollback
- Risks:
  - blocker 叠加时 due 平移或 waiting timer 平移可能重复。
  - 恢复 credit 可能与 long break 周期推进打架。
  - 用户可见状态如果不够清楚，体感会从“更聪明”退化成“更随机”。
- Rollback plan:
  - 若任一主路径不稳定，优先整体回退 reminder v2 相关代码与 locale/docs 改动，不保留半套语义。

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 补齐 spec / daily plan / daily log，冻结本轮 interaction 边界与 AC。
  2. 在 `state.rs` 引入 delivery freeze、recovery hold / credit 与更清晰的 waiting 预算文案。
  3. 对齐 `commands.rs` 的 pause/focus 边界，避免命令路径继续 reset schedule。
  4. 补充 Rust 单测与 locale。
  5. 运行验证，回填 docs / evidence / changelog。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: 用户已在当前会话明确同意执行该方案
