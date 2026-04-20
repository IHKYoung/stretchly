# Task-ID: TID-20260413-fullscreen-break-current-space-fix

## Summary
- Title: 修复全屏工作区下 break 未覆盖当前界面
- Date: 2026-04-13
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 修复 macOS 下“用户正在别的应用全屏工作时，Pauza 到点后没有覆盖当前全屏界面，只在别的屏幕或 Space 自己开始休息”的回归，让 break 能回到当前工作区被用户直接看到。
- In-scope:
  - `apps/desktop/src-tauri/src/shell.rs` 的 macOS break window native collection behavior / 激活路径
  - 对应 Rust 回归测试
  - 本任务 docs、Architecture / CHANGELOG / CodeMap 同步
- Out-of-scope:
  - `engine.rs` 调度状态机
  - React break 页面视觉布局与 CTA
  - 新增设置项、持久化字段或外部依赖
- Assumptions:
  - `engine.rs` 仍然会在到点时触发 `show_break_window()`，问题主要在 macOS Space / fullscreen overlay 行为，而不是调度没有发起
  - 当前工作树里把 `MoveToActiveSpace` 与 `FullScreenAuxiliary` 从 break window 原生策略中移除，是更接近本次回归的直接原因
  - 仅靠 `NSApplication.activate` / `activateIgnoringOtherApps` 不能完全替代正确的 `collectionBehavior`
- Risks:
  - 恢复更激进的原生 overlay 策略后，普通非 strict break 可能更容易抢到当前 Space
  - fullscreen/windowed close path 若和新的 collection behavior 配合不当，可能带回先前的宿主层副作用
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: 用户正在浏览器、编辑器等全屏应用里工作时，到点后的 Pauza break prompt 会直接浮到当前全屏工作区，而不是留在别的屏幕或后台 Space。
- Fallback / secondary flow: 用户不在全屏 Space 时，现有 windowed/fullscreen break 流程继续按当前设置显示，不回退到“只在别处自己开始”的无感状态。
- User-visible boundary: 仅 break window 的宿主显示层与当前 Space 浮出行为。
- Key visible states / transitions:
  - due -> `show_break_window()`
  - native collection behavior patched
  - current fullscreen Space visible overlay
  - break close / skip / finish teardown

## Goal
- 让 break 在 macOS 全屏工作流下重新覆盖当前用户正在看的界面，而不是在别的屏幕或 Space 静默开始。

## Scope
- In-scope:
  - `apps/desktop/src-tauri/src/shell.rs`
  - `docs/specs/TID-20260413-fullscreen-break-current-space-fix/*`
  - `docs/{plans,logs}/2026-04-13.md`
  - `docs/{Architecture,CHANGELOG,CodeMap}.md`
- Out-of-scope:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src-tauri/src/{engine.rs,state.rs,commands.rs}` 的行为修改
  - Playwright 或系统级全屏自动化

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src-tauri/src/engine.rs`
  - `apps/desktop/src-tauri/src/state.rs`
- Related docs/specs/logs reviewed:
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `docs/Architecture.md`
  - `docs/ReminderScheduling.md`
  - `docs/specs/TID-20260408-tauri-window-fullscreen-fix/*`
  - `docs/specs/TID-20260409-break-window-macos-fullscreen-coverage/*`
  - `docs/specs/TID-20260411-break-activation-settings-input-fix/*`
  - `docs/logs/2026-04-09.md`
  - `docs/logs/2026-04-11.md`
- Why these are sufficient:
  - `engine.rs` 已确认 break 触发入口仍是 `show_break_window()`
  - `shell.rs` 已覆盖 monitor 选择、native patch、show/present/focus/activate 全链路
  - 近三轮 fullscreen 相关 spec 已完整记录过 Space overlay 的历史策略与回归轨迹，足以锁定本轮根因而不重写 scheduler

## Acceptance Criteria (AC)
- AC1: macOS break window 的 native overlay policy 会重新允许当前全屏 Space 可见，不再把 `MoveToActiveSpace` / `FullScreenAuxiliary` 当作必须移除的冲突项。
- AC2: `show_break_window()` 现有“native patch before show + present + activate/focus”链路保留，并与修正后的 collection behavior 一起工作，不回退当前通知权限与 close/fullscreen helper 改动。
- AC3: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`npm --prefix apps/desktop run build`、`python3 scripts/validate_workflow_docs.py --mode manual` 通过。
- AC4: 证据文档明确记录“代码路径与构建验证已通过，但真实 fullscreen Space 视觉确认仍需本机手动 spot-check”。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen before code
- Primary flow: 到点后，当前正在使用的全屏应用所在 Space 能直接看到 Pauza break prompt
- Fallback / secondary flow: 非全屏场景下，break window 继续按当前 fullscreen / windowed 设置与 monitor 选择规则显示
- Interaction authority / ownership boundary: 只改 Tauri shell 的宿主浮出策略；React break 页面内容、scheduler 和 settings 不改
- Visible entrypoints / handoff cues: 自动到点 break；当前屏幕被 Pauza 覆盖或浮出
- In-scope interactions:
  - fullscreen app active -> break due -> current Space visible
  - break prompt show/present/activate
  - break close / skip / finish teardown after overlay fix
- Out-of-scope interactions:
  - break CTA 文案和布局
  - settings 页操作
  - tray 菜单结构
- Interaction acceptance criteria:
  - 当前全屏工作区不再“无提示”
  - break 不再只在别的屏幕或后台 Space 自己开始
  - close path 不因 overlay policy 修复而回退

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
- service_impact: 仅限本地桌面端 `shell.rs` 的 macOS break window overlay 行为、对应测试与 workflow docs
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖当前 Git worktree、`cargo test`、desktop build、workflow docs validator 与代码路径对照
- rollback_plan: 回退 `apps/desktop/src-tauri/src/shell.rs`、相关 docs 与测试断言
- destructive_operations: none
- operator_approval_required: no
- rationale: 纯本地桌面端 bug 修复，不涉及提权、数据变更、外部副作用或运行中服务

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 修正 `shell.rs` 的 macOS break window native overlay policy
  - DoD: 当前代码不再移除对全屏 Space 可见性必需的 collection behavior 位，并保留现有 show/present/activate 链路
- [x] Task-2: 补齐 Rust 回归测试与验证
  - DoD: 测试断言与当前策略一致，`cargo test` / desktop build / docs validator 通过
- [x] Task-3: 同步 docs 与证据说明
  - DoD: spec、daily docs、Architecture / CHANGELOG / CodeMap 与 evidence README 完成，不保留 `TBD/INIT`

## Evidence Plan (UI / E2E)
- Evidence required: partial  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260413-fullscreen-break-current-space-fix/evidence/
- Interaction validation note: 本轮以 native code path、Rust 构建测试与 workflow validator 为主；真实 fullscreen Space 可见性仍需用户本机手动 spot-check。
- Required states to capture:
  - current fullscreen Space overlay expectation
  - fixed native behavior bits / test coverage
  - build / validator outputs
  - manual reality gap note

## Observability / Debug Plan
- Logs:
  - 本轮不新增 runtime logger；后续若仍复现，应优先在 `configure_break_window_native_behavior()` 与 `show_break_window()` 周围打点 `collectionBehavior` / `monitor target` / `fullscreen mode`
- Error codes:
  - 继续沿用 `Result<(), String>` 宿主错误上抛
- Trace/metrics (optional):
  - N/A
- Debug flags (optional):
  - N/A

## Risks & Rollback
- Risks:
  - macOS 原生 overlay 策略仍可能受系统版本差异影响
  - 更主动的 Space 浮出策略可能让非 strict break 在某些工作流里显得更强势
- Rollback plan:
  - 回退 `shell.rs` 的 collection behavior 修复和相应测试/文档，再重新执行验证

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 对照当前 `shell.rs` 与历史 spec，确认本轮回归点和不该动的行为边界
  2. 修正 macOS native overlay policy，并保留现有通知权限、show-before-level、activate helper 等未提交改动
  3. 更新 Rust 回归测试断言
  4. 运行 `cargo test`、desktop build 与 workflow docs validator
  5. 同步 evidence、Architecture、CHANGELOG、CodeMap 与当日 logs/plans

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: yes（用户已明确报告该严重 bug 并要求修复；本任务不涉及额外高风险动作）
