# Task-ID: TID-20260411-break-activation-settings-input-fix

## Summary
- Title: 修复全屏场景 break 不浮出与设置手输卡死
- Date: 2026-04-11
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 修复两个当前可复现的桌面端问题：一是在 macOS 全屏工作区中，到点后 break prompt 没有切到当前 Space；二是设置页数字输入仍是“每击键即 clamp + autosave”，导致手动输入时看起来卡住/难以编辑。
- In-scope:
  - `apps/desktop/src/App.tsx` 的 `CompactNumber` 手动输入交互
  - `apps/desktop/src-tauri/src/shell.rs` 的 break window 显示/激活链路
  - 必要的构建/测试与 docs 同步
- Out-of-scope:
  - break 页面视觉排版与 CTA 文案
  - 调度状态机规则、locale 内容、设置 schema
  - 新增设置项或新增 Tauri command
- Assumptions:
  - macOS 全屏 Space 漏浮出更接近 break window 在非 strict/windowed 路径下未显式激活应用，而不是 `tick()` 没有发起 `open_break_window`
  - 设置手输“卡死”更接近受控数字输入每击键立即 clamp，导致多位数/临时空值无法自然编辑，而不是 Rust 持久化死锁
- Risks:
  - macOS 激活应用的补丁若过猛，可能让非 strict 的 break 更容易抢焦点
  - `CompactNumber` 改成草稿式提交后，按钮点击与 blur 的先后顺序需要避免互相覆盖
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: 用户在其它应用的全屏工作区里到点后，break prompt 会浮到当前 Space，而不是停留在后台或无感。
- Fallback / secondary flow: 用户在设置页手动输入分钟/秒数字时，可以临时清空、输入多位数，并在 blur/Enter/加减按钮时再提交，不再每击键回弹。
- User-visible boundary: 仅 break window 的 macOS 浮出行为与设置页数字输入交互。
- Key visible states / transitions:
  - due -> break window visible on current Space
  - settings number input editing draft
  - blur / Enter commit
  - Escape restore current value

## Goal
- 去掉“break 到点了但当前全屏 Space 看不到提示”和“数字输入一敲就回弹”的两条阻塞体验。

## Scope
- In-scope:
- Out-of-scope:
- In-scope:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src-tauri/src/shell.rs`
  - `test/desktopSettingsControls.js`（如需）
  - `docs/specs/TID-20260411-break-activation-settings-input-fix/*`
  - `docs/{plans,logs,CHANGELOG,UI,Architecture}.md`
- Out-of-scope:
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src/locales/**`
  - `apps/site/**`

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/lib/settings-controls.ts`
  - `test/desktopSettingsControls.js`
  - `apps/desktop/src-tauri/src/{shell.rs,commands.rs,state.rs,engine.rs}`
  - `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tao-0.34.8/src/platform_impl/macos/{window.rs,util/async.rs}`
- Related docs/specs/logs reviewed:
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `docs/SettingsInventory.md`
  - `docs/ReminderScheduling.md`
  - `docs/specs/TID-20260408-tauri-window-fullscreen-fix/*`
  - `docs/specs/TID-20260409-break-window-macos-fullscreen-coverage/*`
  - `docs/specs/TID-20260409-break-schedule-input-redesign/*`
  - `docs/specs/TID-20260410-settings-language-switch-freeze/*`
  - `docs/specs/TID-20260411-jump-to-next-break-freeze/*`
  - `docs/logs/2026-04-10.md`
  - `docs/logs/2026-04-11.md`
- Why these are sufficient:
  - 已覆盖前台数字输入组件、保存链路、break window 宿主逻辑、macOS 原生焦点语义，以及最近几轮同域修复记录，足以在不重写调度/设置 schema 的前提下局部修复。

## Acceptance Criteria (AC)
- AC1: macOS 下，到点显示的 break window 即使走 windowed / non-strict 路径，也能切到当前全屏工作区，不再只停留在后台 Space。
- AC2: 设置页 `CompactNumber` 类数字输入允许临时空值和多位数草稿编辑，只在 blur / Enter / 加减按钮时提交 clamp 后的值。
- AC3: 不破坏当前 `shell.rs` 中已存在的 tray refresh workaround 与 break close/fullscreen 行为。
- AC4: `npm test`、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`、`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`python3 scripts/validate_workflow_docs.py --mode manual` 通过。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen before code
- Primary flow: break 到点后，当前全屏工作区应直接看到 Pauza 的 break prompt
- Fallback / secondary flow: 数字输入允许草稿编辑，提交节点统一落在 blur / Enter / step button
- Interaction authority / ownership boundary: 本轮只改宿主显示/激活与数字输入交互，不重做 settings 视觉结构或 break 页面文案
- Visible entrypoints / handoff cues: 自动到点 break；设置页提醒/延后/自然休息的数字输入
- In-scope interactions:
  - macOS fullscreen Space 中 break prompt 浮出
  - `CompactNumber` 输入、清空、回车、失焦、Escape、加减按钮
- Out-of-scope interactions:
  - break CTA 文案与版式
  - 语言切换
  - tray 菜单结构与 shortcut 映射
- Interaction acceptance criteria: 当前全屏 Space 可以看到 break prompt；数字输入不再在每次击键时回弹到 clamp 后的值；`Escape` 恢复当前值，`Enter/blur` 提交当前草稿

### Validator Expectation
当 `interaction_impact != none` 时，本节与 Requirement Brief 中的交互字段不得继续保留 `N/A/TBD`

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
- service_impact: 仅限 desktop 前台数字输入与 macOS break window 宿主激活链路
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖现有 Git worktree、前端/Rust 构建测试、workflow docs validator 与代码路径审查
- rollback_plan: 回退 `App.tsx`、`shell.rs`、可能新增的测试与 docs 改动
- destructive_operations: none
- operator_approval_required: no
- rationale: 纯本地桌面端交互修复，不涉及数据迁移、历史改写、权限提升或外部副作用

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 修复 `CompactNumber` 的草稿输入链路
  - DoD: 手动输入可以临时清空/输入多位数，提交时再 clamp，不再每击键触发最终值回弹
- [x] Task-2: 修复 macOS break window 到点浮出/激活链路
  - DoD: windowed/non-strict break 也会显式激活 app/当前 Space，而不只依赖 `set_focus()`
- [x] Task-3: 跑验证并同步 docs
  - DoD: 构建/测试/validator 通过，specs / daily docs / changelog 不保留占位

## Evidence Plan (UI / E2E)
- Evidence required: partial  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260411-break-activation-settings-input-fix/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - current fullscreen space break visibility（代码路径 + 本机 spot-check 缺口说明）
  - settings number draft editing
  - blur / Enter commit
  - build / test / validator outputs

## Observability / Debug Plan
- Logs:
  - 继续沿用 `runtime.last_action`、tray status 与现有前台 save/load error
- Error codes:
  - 无新增错误码；继续使用 `String` 错误上抛
- Trace/metrics (optional):
  - N/A
- Debug flags (optional):
  - N/A

## Risks & Rollback
- Risks:
  - macOS `activateIgnoringOtherApps` 可能让 break 在普通场景下更主动抢前台
  - 输入草稿与按钮交互若处理不当，可能出现 blur 先提交再被按钮覆盖
- Rollback plan:
  - 回退 `shell.rs` 的激活补丁和 `App.tsx` 的数字输入草稿逻辑，再重新执行构建/测试

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 先把 spec / daily docs 补到可执行状态，明确交互冻结与 AC。
  2. 在 `App.tsx` 把 `CompactNumber` 改成本地草稿式输入。
  3. 在 `shell.rs` 为 macOS windowed/non-focusable break 增加显式 app activation。
  4. 跑前端/Rust/validator 验证，必要时补充最小测试。
  5. 同步 logs / plans / changelog / UI / Architecture 并结案。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: yes（用户已明确报告这两个 bug，并要求修复；本任务不涉及额外高风险动作）
