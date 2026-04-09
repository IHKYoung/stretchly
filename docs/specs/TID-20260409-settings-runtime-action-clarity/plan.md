# Task-ID: TID-20260409-settings-runtime-action-clarity

## Summary
- Title: 整理设置页中的恢复提醒与重置节奏
- Date: 2026-04-09
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 把设置页中语义混乱的运行时动作整理清楚，让“恢复提醒”只在暂停态出现，而“重置节奏”变成独立的节奏控制入口。
- In-scope:
  - 调整 `apps/desktop/src/App.tsx` 的设置页结构与运行时动作显隐规则。
  - 为“暂停态恢复 / 专注态清除 / 节奏重置”补齐清晰文案。
  - 如有必要，为浏览器 preview 增加最小可验证的 paused/focus mock path，便于采集 UI 证据。
  - 同步本任务 specs、daily plans/logs、`docs/UI.md`、`docs/SettingsInventory.md` 与 `docs/CHANGELOG.md`。
- Out-of-scope:
  - 修改 Tauri host 的 reminder state machine、pause/reset 命令语义或 tray 菜单结构。
  - 新增快捷键编辑器、overview 仪表盘或更多运行时动作。
  - 改动 break prompt 视觉层。
- Assumptions:
  - “恢复提醒”只对应手动 pause 的解除，不覆盖 focus / DND / app exclusion 等其他阻塞态。
  - “重置节奏”继续调用现有 `reset_breaks` 命令，并保持“不自动恢复暂停”的行为。
  - 当前用户希望继续把设置页做成更清晰的偏好窗口，而不是重新做回运行时控制台。
- Risks:
  - 若把 `focus` 也继续叫“恢复提醒”，仍会保留原本的语义混淆。
  - 若 `重置节奏` 没写清楚“不解除暂停”，用户仍可能误解它和恢复提醒是同类动作。
  - 若 preview fallback 无法覆盖暂停态证据，UI 验证会偏弱。
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: 用户打开设置页时，只在“已暂停”状态看到顶部 `恢复提醒`；`重置节奏` 单独出现在节奏控制区，并附带“不会自动恢复暂停”的说明。
- Fallback / secondary flow: 用户进入 focus 状态时，顶部上下文动作切换为 `结束专注`；未暂停时不显示 `恢复提醒`。
- User-visible boundary: 仅影响设置页主窗口中的运行时动作入口和说明文案，不改 tray、宿主命令或实际提醒投递逻辑。
- Key visible states / transitions: `running -> no top resume action`、`paused -> top resume action visible`、`focus -> top clear-focus action visible`、`schedule page -> reset action isolated from resume`.

## Goal
- 让设置页里的运行时动作和用户心智对齐，不再把“解除暂停”和“重新计时”混成一组。

## Scope
- In-scope:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/locales/overrides/{zh-CN,en}.json`
  - `apps/desktop/src/locales/registry.generated.json`
  - `docs/{UI,SettingsInventory,CHANGELOG}.md`
  - 本任务 specs / evidence / daily plans / daily logs
- Out-of-scope:
  - `apps/desktop/src-tauri/src/state.rs` / `commands.rs` 的命令语义
  - tray 菜单信息架构
  - break prompt 页面与壁纸/声音设置

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/i18n.ts`
  - `apps/desktop/src/locales/overrides/{zh-CN,en}.json`
  - `apps/desktop/src-tauri/src/commands.rs`
  - `apps/desktop/src-tauri/src/state.rs`
- Related docs/specs/logs reviewed:
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `docs/ReminderScheduling.md`
  - `docs/SettingsInventory.md`
  - `docs/UI.md`
  - `docs/plans/2026-04-09.md`
  - `docs/logs/2026-04-09.md`
- Why these are sufficient:
  - 已覆盖当前设置页结构、浏览器 preview fallback、运行时命令面、pause/focus/reset 的真实语义，以及最近 reminder 模型的产品边界；本轮不需要继续深入 Rust 调度实现。

## Acceptance Criteria (AC)
- AC1: 设置页顶部不再长期并列展示“恢复提醒 / 重置节奏”；`恢复提醒` 只在暂停态显示为上下文动作。
- AC2: `重置节奏` 被移动到独立的节奏控制区，并用文案明确“只重算计时，不自动解除暂停”。
- AC3: 当存在 focus session 时，顶部动作不再误用“恢复提醒”，而是显示专门的 focus 退出动作或保持无歧义状态。
- AC4: 浏览器 preview 或可用 UI 证据能够展示至少一个暂停态和一个默认节奏态界面，证明动作已被拆层。
- AC5: `python3 scripts/sync_desktop_locales.py`、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build` 与 `python3 scripts/validate_workflow_docs.py --mode manual` 通过。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen for implementation
- Primary flow: 设置页顶部只在 pause active 时显示 `恢复提醒`；点击后解除 pause 并回到普通节奏态。
- Fallback / secondary flow: focus active 时改为 `结束专注`；普通运行态顶部不显示恢复动作；`重置节奏` 始终独立存在于节奏控制区。
- Interaction authority / ownership boundary: 本轮只整理设置页的运行时动作入口，不重做 tray / shortcut / break CTA，也不新增新的 pause 语义。
- Visible entrypoints / handoff cues: 设置页顶部状态卡、`节奏` 分类中的独立节奏控制区、状态说明文案。
- In-scope interactions: pause -> resume、focus -> clear focus、schedule reset。
- Out-of-scope interactions: DND / app exclusion / natural break 的宿主自动恢复逻辑；break window 的完成/跳过/延后操作。
- Interaction acceptance criteria: 用户不再把“恢复提醒”和“重置节奏”误认为同类动作；恢复只在暂停态出现，重置始终被理解为独立计时控制。

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
- service_impact: 仅限设置页前台、浏览器 preview fallback 和 locale 覆盖层；不改宿主调度逻辑。
- touches_running_service: no
- backup_required: no
- backup_plan: 以 Git diff、typecheck/build、workflow docs validator 与 UI 证据为回滚边界。
- rollback_plan: 回退 `App.tsx`、locale overrides/registry、docs 与 evidence。
- destructive_operations: 替换设置页中运行时动作的层级表达。
- operator_approval_required: no
- rationale: 无线上服务、权限、依赖、外部副作用或数据迁移风险。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 收紧运行时动作的信息架构
  - DoD: `恢复提醒` 只在暂停态出现；focus 态不再复用该命名；`重置节奏` 独立成组。
- [x] Task-2: 补齐文案、preview 验证与长期文档
  - DoD: locale、UI 文档、SettingsInventory、CHANGELOG、specs 与 daily plans/logs 同步完成，且有可追溯验证。

## Evidence Plan (UI / E2E)
- Evidence required: yes  <!-- yes | no | partial -->
- Owner: orchestrator（single-agent-fallback）
- Artifact path: docs/specs/TID-20260409-settings-runtime-action-clarity/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: N/A（本轮只需证明设置页无报错并能稳定渲染）
  - disabled: 默认运行态顶部不显示 `恢复提醒`
  - success: 暂停态顶部出现 `恢复提醒`；节奏页中 `重置节奏` 独立成组并带说明

## Observability / Debug Plan
- Logs: 复用 `DesktopSnapshot.status/statusDetail` 与 `lastAction` 作为设置页里的运行时可视反馈，不新增持久化日志点。
- Error codes: N/A
- Trace/metrics (optional):
- Debug flags (optional): 如需证据，允许浏览器 preview 读取最小 query 参数来模拟 paused / focus 状态；不影响 Tauri runtime。

## Risks & Rollback
- Risks:
  - 顶部动作若仍在默认运行态常驻出现，用户会继续把它理解成设置。
  - 若 `重置节奏` 的说明不够明确，仍可能被误认为“恢复提醒”的别名。
  - 若 preview fallback 的状态模拟写得太重，可能污染正常浏览器预览路径。
- Rollback plan:
  - 回退设置页动作区、preview mock 与 locale 变更，再重新运行前端构建和 docs validator。

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 补齐本任务 specs / daily plans / daily logs 的边界、交互冻结和验证计划。
  2. 在 `App.tsx` 中加入顶部上下文状态卡，并把 `恢复提醒` 限制为 pause-only action。
  3. 在 `节奏` 分类中加入独立的 `重置节奏` 控制区，明确“不会自动解除暂停”。
  4. 补齐所需 locale 文案，必要时增加最小 preview mock 以采集暂停态证据。
  5. 运行 locale sync、typecheck、build、docs validator，采集 UI 证据并完成 docs/changelog 结案。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: user direct request in-thread（用户已明确指出“恢复提醒”和“重置节奏”有冲突，需要整理）
