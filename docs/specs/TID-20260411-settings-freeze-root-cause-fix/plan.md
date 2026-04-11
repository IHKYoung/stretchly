# Task-ID: TID-20260411-settings-freeze-root-cause-fix

## Summary
- Title: 修复设置点击与时间修改卡死
- Date: 2026-04-11
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 修复设置页“点击设置项仍会卡死、改时间后仍会卡死”的问题，并找到更本质的冻结来源，而不是继续补单个输入控件。
- In-scope:
  - `apps/desktop/src/App.tsx` 的 autosave 调度
  - `apps/desktop/src-tauri/src/{commands.rs,state.rs}` 的设置应用与宿主刷新策略
  - 必要的测试与 docs 闭环
- Out-of-scope:
  - 设置页视觉布局调整
  - 新设置 schema / 数据迁移
  - break 调度规则本身
- Assumptions:
  - 真正的冻结来源不是“时间输入框仍然支持不好”，而是设置 autosave 在前端会重入，Rust host 又会在每次保存时无差别触发快捷键/tray 全量刷新
  - 如果把保存串行化，并把 host 刷新改成差异驱动，点击设置项和改时间两条路径都会显著稳定
- Risks:
  - autosave 串行化后，连续多次操作会被合并成顺序保存，体感上可能比之前更“晚一点落盘”，但应更稳定
  - 如果 tray menu rebuild 条件收得过窄，可能出现某些设置变了但菜单未即时重建；因此需要保留语言变更时的完整 rebuild，并让其他情况走 `refresh_tray_if_needed`
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: 用户点击设置开关、选择器或调整时间时，设置页不再因为 host 连续重操作而卡死。
- Fallback / secondary flow: 用户在一次保存尚未完成前继续修改其它设置时，新的草稿会排队到下一轮保存，而不是与上一轮并发执行。
- User-visible boundary: 仅限设置页交互与对应 host 应用链路；不改视觉结构与 break 文案。
- Key visible states / transitions:
  - idle -> debounce save scheduled
  - save in flight -> later edits merged into next save
  - no-op setting update -> 不再触发多余 host refresh
  - language change -> tray menu full rebuild
  - ordinary setting change -> tray/light refresh only

## Goal
- 从根因层面降低设置页冻结概率：避免 autosave 并发叠加，同时避免每次设置变更都触发宿主全量刷新。

## Scope
- In-scope:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src-tauri/src/{commands.rs,state.rs}`
  - `docs/specs/TID-20260411-settings-freeze-root-cause-fix/*`
  - `docs/{plans,logs,CHANGELOG,Architecture,UI}.md`
- Out-of-scope:
  - `apps/desktop/src/styles.css`
  - `apps/desktop/src-tauri/src/{engine.rs,shell.rs}`
  - `apps/site/**`

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/lib/settings-controls.ts`
  - `apps/desktop/src-tauri/src/{commands.rs,state.rs,shell.rs}`
  - `test/desktopSettingsControls.js`
- Related docs/specs/logs reviewed:
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `docs/specs/TID-20260409-break-schedule-input-redesign/plan.md`
  - `docs/specs/TID-20260410-settings-language-switch-freeze/plan.md`
  - `docs/specs/TID-20260411-break-activation-settings-input-fix/plan.md`
  - `docs/plans/2026-04-11.md`
  - `docs/logs/2026-04-11.md`
- Why these are sufficient:
  - 已覆盖前端数字输入/语言保存的既有修复、当前 autosave 实现、Rust `update_settings` 链路，以及近期与设置页冻结直接相关的历史任务，足以定位“真正重”的路径在哪里。

## Acceptance Criteria (AC)
- AC1: `App.tsx` 的 autosave 不再让多个 `update_settings` 并发执行；当一轮保存未完成时，后续编辑会被合并到下一轮。
- AC2: Rust `update_settings` 不再每次都无条件重绑快捷键和整棵 tray `set_menu()`；只有相关设置确实变化时才做对应 host refresh。
- AC3: no-op 设置更新不会重复写回同一份 settings，也不会重复触发 host refresh。
- AC4: `npm test`、`npm run typecheck`、`npm --prefix apps/desktop run build`、`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`python3 scripts/validate_workflow_docs.py --mode manual` 通过。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen before code
- Primary flow: 点击设置项或修改时间后，设置页保持响应，保存异步完成但不出现冻结
- Fallback / secondary flow: 保存进行中若继续修改设置，新的草稿排队到下一轮保存
- Interaction authority / ownership boundary: 本轮只修 autosave 和 host refresh 行为，不改设置页布局和 break 交互
- Visible entrypoints / handoff cues: `节奏` preset / stepper、`偏好` 开关/下拉/语言/托盘选项
- In-scope interactions:
  - 点击设置页开关、分段控件、下拉
  - 调整节奏页时间 preset / stepper
  - 保存进行中的继续编辑
- Out-of-scope interactions:
  - break 页面 CTA
  - 设置页布局与视觉层
  - 官网交互
- Interaction acceptance criteria: 点击设置项和修改时间都不再同步触发重度 host refresh；当一轮保存未完成时，后续编辑不会并发发起第二个宿主设置更新。

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
- service_impact: 仅限桌面端设置 autosave 与 Rust host 设置应用链路
- touches_running_service: no
- backup_required: no
- backup_plan: `npm test` + `npm run typecheck` + `npm --prefix apps/desktop run build` + `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` + `python3 scripts/validate_workflow_docs.py --mode manual`
- rollback_plan: 回退 `apps/desktop/src/App.tsx`、`apps/desktop/src-tauri/src/{commands.rs,state.rs}` 与本任务 docs
- destructive_operations: none
- operator_approval_required: no
- rationale: 纯本地桌面端交互/宿主修复，不涉及数据迁移、提权、外部服务或历史改写

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 确认设置页冻结的主因是 autosave 并发 + host 全量刷新
  - DoD: 通过源码路径审查明确 `App.tsx` 与 Rust `update_settings` 的问题边界，不再把问题表述成单一输入控件 bug。
- [x] Task-2: 将前端 autosave 改成串行/合并保存
  - DoD: 保存进行中不会并发发起第二个 `update_settings`；后续编辑排到下一轮。
- [x] Task-3: 将 Rust host 设置刷新改成差异驱动
  - DoD: no-op 更新直接短路；快捷键只在绑定变化时重绑；tray 仅在语言变化时整棵重建，其余情况走按需轻量刷新。
- [x] Task-4: 跑验证并同步 docs
  - DoD: 新增测试通过，plans/logs/spec/changelog/architecture/ui 不留占位。

## Evidence Plan (UI / E2E)
- Evidence required: partial  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260411-settings-freeze-root-cause-fix/evidence/
- Required states to capture:
  - save in flight but UI remains editable
  - no-op update short-circuit
  - host refresh diff path（shortcut/tray）
  - success（test/build/validator）

## Observability / Debug Plan
- Logs: 通过 `docs/logs/2026-04-11.md` 记录根因分析、验证命令与后续风险；运行时继续沿用既有 save/load error surfaced in UI
- Error codes: N/A（当前设置链路仍以字符串错误上抛）
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 如果 autosave 串行化实现有误，可能出现保存遗漏或保存时机过晚
  - 如果 tray rebuild 条件过窄，某些菜单文案可能不会即时刷新
- Rollback plan:
  - 回退 `App.tsx` 的保存串行化逻辑与 Rust `update_settings` 差异判断，再复测

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 审查现有 `App.tsx` autosave effect 和 Rust `update_settings` 链路。
  2. 让前端 autosave 改成串行/合并保存，避免并发宿主调用。
  3. 让 Rust `update_settings` 只在必要时刷新快捷键和 tray。
  4. 补测试并同步 docs。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: yes（用户已明确要求继续修复设置页卡死问题；本任务不涉及高风险动作）
