# Task-ID: TID-20260403-settings-minimal-ui

## Summary
- Title: 设置页极简重做：仅保留核心真设置
- Date: 2026-04-03
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 将当前杂乱的设置页彻底收成极简 split view，只保留用户确认的核心真设置，让 Pauza 看起来像安静的桌面工具，而不是展示型 dashboard。
- In-scope:
  - 重写 `apps/desktop/src/App.tsx` 的 settings IA 和布局。
  - 补齐精简设置页所需 locale 文案。
  - 收紧 desktop 前台的卡片/背景视觉语气，并保留 break window 运行路径。
  - 补齐本任务 specs、evidence、daily logs/plans、UI/CHANGELOG。
- Out-of-scope:
  - 不改 `PauzaSettings` schema 和 Rust host 调度逻辑。
  - 不新增设置项，不恢复概览/快捷动作/快捷键编辑。
  - 不修改 Electron legacy preferences。
- Assumptions:
  - 当前真正需要常驻设置页的仅有 9 类核心设置。
  - 低频/运行时动作应停留在 tray、shortcut 或 break window，而不是设置页。
  - 隐藏字段仍需跟随完整 `settings` 快照 round-trip 保存，避免破坏兼容性。
- Risks:
  - 只暴露核心字段后，低频能力暂时变得不可见。
  - 大幅重写 `App.tsx` 容易引入遗漏的前端依赖或文案键。
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: 进入设置页后，通过左侧 4 个分类切换核心设置，在右侧唯一主面板编辑并保存。
- Fallback / secondary flow: 浏览器 preview 下仍可浏览和保存模拟状态；`?window=break` 路径继续显示 break prompt，不受主设置页重构影响。
- User-visible boundary: 仅 `apps/desktop` 的主设置页与其文案/视觉发生变化。
- Key visible states / transitions:
  - `synced` / `pendingChanges`
  - 分类切换：`节奏` / `提醒与打断` / `智能暂停` / `通用`
  - 保存 / 撤销
  - 预览态的错误提示与状态文案

## Goal
- 让设置页回到桌面工具该有的克制状态：左侧导航，右侧单主面板，只摆真正需要设置的内容。

## Scope
- In-scope:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/styles.css`
  - `apps/desktop/src/components/ui/card.tsx`
  - `apps/desktop/src/locales/{zh-CN,en}.json`
  - `docs/specs/TID-20260403-settings-minimal-ui/*`
  - `docs/plans/2026-04-03.md`
  - `docs/logs/2026-04-03.md`
  - `docs/UI.md`
  - `docs/CHANGELOG.md`
- Out-of-scope:
  - `apps/desktop/src-tauri/src/**`
  - `app/**`
  - 新依赖、系统权限与分发链

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/styles.css`
  - `apps/desktop/src/components/ui/{card,badge,button,input,select,segmented-control}.tsx`
  - `apps/desktop/src/locales/{zh-CN,en}.json`
  - `apps/desktop/src/i18n.ts`
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src-tauri/src/commands.rs`
- Related docs/specs/logs reviewed:
  - `AGENTS.md`
  - `docs/UI.md`
  - `docs/CHANGELOG.md`
  - `docs/plans/2026-04-03.md`
  - `docs/logs/2026-04-03.md`
  - `docs/specs/TID-20260403-settings-fixed-split/*`
- Why these are sufficient:
  - 已覆盖当前设置真源、前端实现、现有视觉基座和相邻改版历史，足以在不改 host 契约的前提下重做前端。

## Acceptance Criteria (AC)
- AC1: 设置页主结构收敛为左侧导航 + 右侧唯一主内容面板，桌面宽度下维持约 `1/4 : 3/4` 分栏。
- AC2: 默认只显示 4 个类别：`节奏`、`提醒与打断`、`智能暂停`、`通用`；其中只包含用户确认的核心真设置。
- AC3: 概览、快捷动作、运行时控制、快捷键编辑和多余介绍性文案不再出现在设置页主结构。
- AC4: 保存仍通过 `update_settings` 提交完整 `PauzaSettings`，未暴露字段不会因前端重构而丢失。
- AC5: `typecheck`、`build` 和浏览器预览截图通过，截图能证明页面已从“多卡片 dashboard”收敛为极简分栏。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen
- Primary flow: 左侧点击分类，右侧编辑对应设置，顶部执行保存或撤销。
- Fallback / secondary flow: preview 模式下允许用模拟数据预览保存态；break window 保持独立。
- Interaction authority / ownership boundary: 设置页只负责配置，不负责 pause/focus/skip/reset 等运行时动作。
- Visible entrypoints / handoff cues: 左侧分类按钮、右侧标题、顶部 `保存 / 撤销`、底部状态摘要。
- In-scope interactions:
  - 分类切换
  - switch / segmented control / select / textarea / number input 编辑
  - 保存 / 撤销
- Out-of-scope interactions:
  - 概览卡片
  - 快捷动作按钮
  - 快捷键录入
  - 运行时 pause / focus / skip / reset
- Interaction acceptance criteria:
  - 进入任意分类时，用户只看到与该分类相关的核心设置。
  - 页面没有额外第三列、hero 文案或 overview dashboard。
  - 信息层级在 `1440x810` 下稳定可读。
- Validator expectation: 本任务为 direct interaction 变更，已补齐 primary flow、boundary 与 evidence。

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
- service_impact: 仅改 Tauri 前端设置页、locale 与 docs，不改 Rust host 契约或持久化结构。
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 VCS、`typecheck/build`、Playwright 截图与相邻 specs 作为回退和旁证。
- rollback_plan: 回退 `apps/desktop/src/{App.tsx,styles.css,locales/*,components/ui/card.tsx}` 与本任务 docs。
- destructive_operations: 替换当前设置页布局与视觉表达。
- operator_approval_required: no
- rationale: 用户已明确要求前端“直接推翻”，且本轮没有数据变更、外部副作用或新增依赖。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 重写设置页 IA，只保留 4 个分类和核心真设置
  - DoD: `App.tsx` 中不再保留 overview / quick actions / advanced shortcut editor 作为主结构。
- [x] Task-2: 收紧视觉语言，恢复 `1/4 : 3/4` 分栏并减少卡片感
  - DoD: 设置页截图中只剩左侧导航区与右侧单主面板，背景和圆角明显收敛。
- [x] Task-3: 补齐 locale、spec、evidence、logs/plans 和总览文档
  - DoD: 不存在 `INIT/TBD/N/A` 占位残留在本任务必填字段中，workflow validator 通过。

## Evidence Plan (UI / E2E)
- Evidence required: yes  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260403-settings-minimal-ui/evidence/
- Interaction validation note: 以 `1440x810` 桌面截图、DOM snapshot 与 console log 证明新设置页的主结构和状态层级。
- Required states to capture:
  - loading: 非核心，本轮不单独截图
  - empty: N/A（设置页非列表型界面）
  - error: 保留顶部错误条路径，不单独造错截图
  - disabled: `保存 / 撤销` 在无改动时为 disabled
  - success: `settings-minimal-1440x810.png` 证明成功态主布局

## Observability / Debug Plan
- Logs: 复用前台 `error` 区与 preview/runtime 状态文案，不新增新日志面。
- Error codes: 无新增错误码；仍由 Tauri command 错误文案直接上浮。
- Trace/metrics (optional): 无。
- Debug flags (optional): 无。

## Risks & Rollback
- Risks:
  - 低频设置暂时不可见，可能需要后续独立高级页或二级入口承接。
  - 大文件 `App.tsx` 全量重写时容易遗漏旧功能引用。
- Rollback plan:
  - 回退上述前端文件与本任务 docs，恢复到 `TID-20260403-settings-fixed-split` 之后的状态。

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 以“真设置清单”为唯一范围，重写 `App.tsx` 的类别与内容区。
  2. 去掉 overview/quick actions/save rail 三段式结构，恢复双栏 split view。
  3. 补齐 locale 键并收紧样式基座。
  4. 运行 `typecheck`、`build`、Playwright 截图并补全 docs。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: 用户在当前线程明确确认核心设置清单并要求直接推翻前端重做。
