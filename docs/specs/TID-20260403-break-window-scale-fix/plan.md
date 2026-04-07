# Task-ID: TID-20260403-break-window-scale-fix

## Summary
- Title: 修复休息窗口过小与双层卡片
- Date: 2026-04-03
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 修掉 break 页里“窗口已经不大，里面还再放一张小卡片”的套娃结构，让休息页真正吃满宿主窗口，并把 window 模式下的默认尺寸拉回正常水平。
- In-scope:
  - 重写 `App.tsx` 的 `BreakWindow` 结构，让内容直接填满 break window。
  - 在 preview break route 下提供模拟当前 break，方便直接校验 UI。
  - 调整 `shell.rs` 中三档 break profile 的默认 window-mode 尺寸。
  - 补齐本任务 docs 和证据。
- Out-of-scope:
  - 不改 break scheduler、strict/manual 逻辑和操作按钮语义。
  - 不重做主设置页。
  - 不新增新的 break surface 设置项。
- Assumptions:
  - 用户吐槽的“这个小窗口”主要来自 break 页的双层容器和过小默认尺寸。
  - 让 break 页直接占满宿主窗口，比继续微调内层卡片宽度更有效。
  - 浏览器 preview 需要有模拟 break 才能稳定校验该界面。
- Risks:
  - window-mode 尺寸拉大后，gentle 模式会比之前更明显。
  - preview break mock 仅用于浏览器预览，不应误当成运行时真源。
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: break 触发后，用户直接看到一个填满当前 break window 的页面，而不是外层窗体里再嵌套一张小卡片。
- Fallback / secondary flow: 浏览器 preview 的 `?window=break` 现在也能显示模拟中的微休息，用于肉眼校验 break 页尺度。
- User-visible boundary: 仅 break prompt 页面和宿主窗口默认尺寸发生变化。
- Key visible states / transitions:
  - active break 倒计时
  - 标题/说明
  - 完成 / 稍后 / 跳过按钮
  - preview break route 可直接可视化

## Goal
- 让 break window 看起来像一个真正的休息窗口，而不是“窗口里再塞个小浮层”。

## Scope
- In-scope:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src-tauri/src/shell.rs`
  - `docs/specs/TID-20260403-break-window-scale-fix/*`
  - `docs/plans/2026-04-03.md`
  - `docs/logs/2026-04-03.md`
  - `docs/UI.md`
  - `docs/CHANGELOG.md`
- Out-of-scope:
  - `apps/desktop/src-tauri/src/state.rs`
  - `app/**`
  - 新增 break 配置项

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src/components/ui/{card,button}.tsx`
  - `docs/specs/TID-20260403-hig-tailwind-redesign/evidence/break-snapshot.md`
- Related docs/specs/logs reviewed:
  - `AGENTS.md`
  - `docs/UI.md`
  - `docs/plans/2026-04-03.md`
  - `docs/logs/2026-04-03.md`
  - `docs/specs/TID-20260403-break-surface-mode-restore/*`
- Why these are sufficient:
  - 已覆盖当前 break 页 React 结构、Tauri host 尺寸策略、相邻 break 改版证据和用户刚刚指出的问题边界。

## Acceptance Criteria (AC)
- AC1: break 页面不再渲染“外层窗口里再套一张固定宽度卡片”的结构。
- AC2: `?window=break` 在浏览器 preview 下可直接看到模拟中的微休息页面。
- AC3: break 内容区域宽度明显增大，标题、倒计时和按钮直接使用宿主窗口空间。
- AC4: window 模式下 gentle / balanced / immersive 的默认 break window 尺寸整体大于之前版本。
- AC5: `cargo check`、`typecheck`、`build` 通过，并有 preview DOM snapshot 证明 break 页结构已变化。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen
- Primary flow: break window 打开后直接看到完整 break 页面，无额外内层卡片容器。
- Fallback / secondary flow: `?window=break` preview 使用 mock current break 进行 UI 校验。
- Interaction authority / ownership boundary: 本任务只改 break 页展示结构和宿主尺寸，不改 action 行为。
- Visible entrypoints / handoff cues: break 页面顶部 badge/clock、中部倒计时和底部 CTA。
- In-scope interactions:
  - 完成 / 稍后 / 跳过按钮
  - preview break 页面渲染
- Out-of-scope interactions:
  - 调度状态机
  - strict/manual 规则
  - 新设置项
- Interaction acceptance criteria:
  - DOM snapshot 中 break 页直接由顶部信息、主体、按钮三段组成。
  - 不再存在内层 `Card` 包裹的窄面板结构。
- Validator expectation: direct interaction 的 primary flow 与 evidence 已补齐。

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
- service_impact: 仅改 break 页 React 结构和宿主默认尺寸，不改调度逻辑。
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 VCS、cargo/typecheck/build 和 preview snapshot。
- rollback_plan: 回退 `App.tsx`、`shell.rs` 与本任务 docs。
- destructive_operations: 替换当前 break 页双层容器结构与宿主默认尺寸。
- operator_approval_required: no
- rationale: 用户明确要求修掉 break 小窗口观感，本轮无数据与外部副作用风险。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 重写 `BreakWindow` 结构，使内容直接占用宿主窗口
  - DoD: break 页面不再使用固定 `max-w-[460px]` 内层卡片。
- [x] Task-2: 调整 Tauri host 默认尺寸
  - DoD: `shell.rs` 中 window-mode 的三档默认尺寸整体拉大。
- [x] Task-3: 补齐 preview 证据与 docs
  - DoD: preview DOM snapshot 可见新的 break 页面结构，workflow validator 通过。

## Evidence Plan (UI / E2E)
- Evidence required: partial  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260403-break-window-scale-fix/evidence/
- Interaction validation note: 由于 screenshot 工具持续卡在 fonts wait，本轮以 preview break DOM snapshot 为主。
- Required states to capture:
  - loading: 非核心，本轮不单独采集
  - empty: N/A
  - error: console 无新错误
  - disabled: N/A
  - success: `break-window-preview-snapshot.md` 证明 break 页面已直接填满宿主窗口结构

## Observability / Debug Plan
- Logs: 无新增日志，依赖 build/cargo/console 输出。
- Error codes: 无新增错误码。
- Trace/metrics (optional): 无。
- Debug flags (optional): 无。

## Risks & Rollback
- Risks:
  - 某些小尺寸屏幕上，window-mode gentle 仍可能需要进一步微调。
  - preview break mock 只用于 UI 校验，需要避免误导后续实现。
- Rollback plan:
  - 回退 `App.tsx`、`shell.rs` 与 docs。

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 确认 break 页的双层容器和固定宽度限制。
  2. 让 break 页直接吃满宿主窗口，并在 preview 下提供 mock current break。
  3. 调整 host 的 window-mode 默认尺寸。
  4. 跑 `cargo check`、`typecheck`、`build`，补充 preview snapshot 与 docs。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: 用户在当前线程明确指出 break 小窗口效果“太扯淡”，要求修正。
