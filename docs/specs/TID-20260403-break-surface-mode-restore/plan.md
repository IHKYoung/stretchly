# Task-ID: TID-20260403-break-surface-mode-restore

## Summary
- Title: 恢复休息窗口/全屏设置
- Date: 2026-04-03
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 恢复 Tauri 端 break surface mode，让用户再次可以在设置页中选择休息提示以“窗口”还是“全屏”出现。
- In-scope:
  - 在 `PauzaSettings` 中恢复 `fullscreen` 持久化字段。
  - 更新 `shell.rs` 的 break window profile，使 `window / fullscreen` 成为独立配置轴。
  - 在设置页的“提醒与打断”分类重新暴露 `窗口 / 全屏` 入口。
  - 补齐本任务 docs 与 evidence。
- Out-of-scope:
  - 不改 Electron legacy 偏好页。
  - 不新增 `showBreaksAsRegularWindows`、`breakWindowWidth/Height` 等更细高级项。
  - 不改 break prompt React 内容布局。
- Assumptions:
  - 用户说的“丢失”指旧 Electron 中的 `showBreaksIn: window/fullscreen` 能力。
  - `breakPromptStyle` 与 `fullscreen` 应是两根不同设置轴，而不是互相替代。
  - 当前最合理的恢复方式是保持旧字段语义，减少迁移成本。
- Risks:
  - `fullscreen=true` 会让 microbreak 也变成全屏，这属于更强打断。
  - 如果 `immersive` 在窗口模式下尺寸过大，可能需要后续细调。
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: 用户进入“提醒与打断”，在“休息窗口 → 显示方式”里切换 `窗口 / 全屏` 并保存。
- Fallback / secondary flow: 未保存时只改变本地 dirty form；break prompt 页面继续使用原内容，只改变宿主窗口尺寸/全屏行为。
- User-visible boundary: 设置页和休息窗口的展示模式。
- Key visible states / transitions:
  - `窗口` -> break 以窗口形式出现
  - `全屏` -> break 铺满目标显示器
  - `已同步 / 有未保存更改`

## Goal
- 把“窗口 / 全屏”从丢失状态恢复为真正可设置、可保存、可影响 break window 行为的功能。

## Scope
- In-scope:
  - `apps/desktop/src-tauri/src/{state.rs,shell.rs}`
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/locales/{zh-CN,en}.json`
  - `docs/specs/TID-20260403-break-surface-mode-restore/*`
  - `docs/plans/2026-04-03.md`
  - `docs/logs/2026-04-03.md`
  - `docs/UI.md`
  - `docs/Architecture.md`
  - `docs/CHANGELOG.md`
- Out-of-scope:
  - `app/**`
  - 新增高级 break surface 细粒度配置
  - 重新设计 break prompt 前台内容

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `app/preferences.html`
  - `app/utils/defaultSettings.js`
  - `app/utils/breakWindowProfile.js`
  - `app/main.js`
  - `apps/desktop/src-tauri/src/{state.rs,shell.rs,commands.rs}`
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/locales/{zh-CN,en}.json`
- Related docs/specs/logs reviewed:
  - `AGENTS.md`
  - `docs/UI.md`
  - `docs/Architecture.md`
  - `docs/plans/2026-04-03.md`
  - `docs/logs/2026-04-03.md`
  - `docs/specs/TID-20260402-modern-break-experience/*`
  - `docs/specs/TID-20260403-settings-minimal-ui/*`
- Why these are sufficient:
  - 已覆盖旧功能真源、旧窗口策略、当前 Tauri host 和当前前端入口，足以判断功能丢失点并恢复兼容语义。

## Acceptance Criteria (AC)
- AC1: `PauzaSettings` 重新拥有 `fullscreen` 持久化字段，缺失旧值时回退到默认 `false`。
- AC2: Tauri 设置页在“提醒与打断”分类重新可见 `显示方式：窗口 / 全屏`。
- AC3: `update_settings` 保存后，`shell.rs` 会根据 `fullscreen` 决定 break window 是否全屏。
- AC4: `window` 模式下，`immersive` long break 不再被强制全屏，而是使用大尺寸窗口。
- AC5: `cargo check`、`typecheck`、`build` 通过，并有 DOM snapshot 证明入口已恢复。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen
- Primary flow: 在“提醒与打断”里切换 `窗口 / 全屏`，保存后影响 break window 宿主行为。
- Fallback / secondary flow: 不保存时只保留本地脏状态；break prompt 内容页本身不新增新控件。
- Interaction authority / ownership boundary: surface mode 由设置页决定，break content 由现有 break prompt 决定。
- Visible entrypoints / handoff cues: “提醒与打断”导航、`休息窗口` 分组、`显示方式` segmented control。
- In-scope interactions:
  - `窗口 / 全屏` 切换
  - 保存后宿主 break surface 生效
- Out-of-scope interactions:
  - show regular windows
  - break window size width/height
  - break prompt 内容重排
- Interaction acceptance criteria:
  - DOM snapshot 中可见 `显示方式` 和对应 tablist。
  - 保存链路仍为统一 `update_settings`。
- Validator expectation: direct interaction 变更已补齐 primary flow 和 evidence。

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
- service_impact: 仅改 Tauri 设置模型、break window profile 和前端入口，不改调度算法。
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 VCS、`cargo check`、`typecheck/build`、DOM snapshot 与 console log。
- rollback_plan: 回退 `state.rs`、`shell.rs`、`App.tsx`、locale 与 docs。
- destructive_operations: 替换当前 break window surface mode 逻辑。
- operator_approval_required: no
- rationale: 用户明确要求恢复功能，且本轮没有数据或外部副作用风险。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 恢复 `fullscreen` 设置真源和 Tauri shell break profile
  - DoD: Rust host 编译通过，`window/fullscreen` 不再只是前端占位。
- [x] Task-2: 在设置页恢复“窗口 / 全屏”入口并补文案
  - DoD: “提醒与打断”分类中重新出现 `显示方式` segmented control。
- [x] Task-3: 补齐验证证据和 docs
  - DoD: 本任务 spec/daily docs 不再保留占位，workflow validator 通过。

## Evidence Plan (UI / E2E)
- Evidence required: partial  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260403-break-surface-mode-restore/evidence/
- Interaction validation note: Playwright page screenshot在当前页面上卡在 fonts wait，本轮以 DOM snapshot + console + 编译链证明入口恢复，后续如需可再补真实截图。
- Required states to capture:
  - loading: 非核心，本轮不单独截图
  - empty: N/A
  - error: 控制台日志确认无新增前端错误
  - disabled: 保存按钮在未修改时保持 disabled
  - success: `settings-break-surface-snapshot-1440x810.md` 证明入口恢复

## Observability / Debug Plan
- Logs: 保留前台 error 条与 host action 文案，不新增新日志面。
- Error codes: 无新增错误码。
- Trace/metrics (optional): 无。
- Debug flags (optional): 无。

## Risks & Rollback
- Risks:
  - full screen 模式对 microbreak 也会更强打断。
  - window 模式下 immersive 长休息窗口可能还需要细调尺寸。
- Rollback plan:
  - 回退上述前端和 Rust host 文件，恢复到仅由 `breakPromptStyle` 控制的版本。

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 对照旧 Electron 代码确认 `fullscreen` 真语义。
  2. 在 Rust `PauzaSettings` 和 `shell.rs` 恢复这一设置轴。
  3. 在 React 设置页恢复入口并补 locale。
  4. 运行 `cargo check`、`typecheck`、`build`，补充 DOM snapshot 和 docs。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: 用户在当前线程明确指出“窗口 / 全屏”功能丢失并要求恢复。
