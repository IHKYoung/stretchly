# Task-ID: TID-20260403-macos-tray-menu-crash

## Summary
- Title: 修复 macOS 托盘右键菜单闪退
- Date: 2026-04-03
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: REVIEW

## Requirement Brief
- Goal restatement: 修复 `apps/desktop` 在 macOS 上顶部菜单栏 tray icon 右键后的闪退问题，保持既有 tray 信息架构和动作语义不变。
- In-scope: `apps/desktop/src-tauri/src/shell.rs` 的 tray context menu 构造，任务 docs 与 changelog/architecture/ui 索引同步。
- Out-of-scope: Electron legacy tray、React/Tailwind 前台、break 调度逻辑、文案重设计。
- Assumptions: 闪退由 macOS tray root menu 类型不符合 `muda` 约束触发；现有 menu action ids 和 i18n key 应继续复用。
- Risks: 共享填充函数或 tray refresh key 可能引入菜单更新回归；缺少 assistive access 导致自动化右键复测受阻。
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: macOS 用户右键 tray icon 时原生菜单稳定展开
- Fallback / secondary flow: 左键 tray icon 继续只显示主窗口
- User-visible boundary: 仅 `apps/desktop` 的 macOS tray icon context menu
- Key visible states / transitions: idle tray -> 右键展开菜单；strict mode -> 仅保留状态与退出；左键 -> reveal main window

## Goal
- 用平台分支修正 macOS tray context menu 根类型，使右键菜单不再闪退，同时不改变现有 tray actions 的语义和布局。

## Scope
- In-scope: `apps/desktop/src-tauri/src/shell.rs` 与本任务 specs/logs/plans/changelog/architecture/ui 索引同步
- Out-of-scope: `app/**` Electron legacy tray、`apps/desktop/src/**` React/Tailwind 前台、`apps/desktop/src-tauri/src/{state,engine,commands}.rs` 的运行时契约

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src-tauri/tauri.conf.json`
  - `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/muda-0.17.1/src/menu.rs`
  - `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tauri-2.10.3/src/tray/mod.rs`
  - `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tauri-2.10.3/src/menu/submenu.rs`
- Related docs/specs/logs reviewed:
  - `AGENTS.md`
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `docs/Architecture.md`
  - `docs/UI.md`
  - `docs/plans/2026-04-03.md`
  - `docs/logs/2026-04-03.md`
- Why these are sufficient:
  - 已覆盖 tray 构造、状态快照、Tauri menu/tray 泛型接口以及底层 `muda` 的 macOS 平台约束，足以收敛 root cause 并局部修复。

## Acceptance Criteria (AC)
- AC1: macOS 右键 tray icon 不再闪退，原生菜单可正常展开。
- AC2: tray 菜单动作 ID、文案与左键显示主窗口的语义保持不变。
- AC3: 后台 tick 不再每秒重建 tray menu，只在菜单内容有效变化时刷新。
- AC4: `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` 通过，`npm --prefix apps/desktop run tauri dev` 可拉起真实 desktop runtime。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen
- Primary flow: tray 右键后原生菜单展开
- Fallback / secondary flow: tray 左键显示主窗口
- Interaction authority / ownership boundary: 仅 `shell.rs` 的 Tauri tray 行为；不触碰 React 前台和 Electron legacy UI
- Visible entrypoints / handoff cues: macOS 顶部菜单栏的 Pauza tray icon
- In-scope interactions: tray 菜单展开、状态项、submenu 层级、动作项触发后的菜单刷新
- Out-of-scope interactions: 设置页视觉、break prompt、legacy tray、快捷键绑定语义
### Interaction acceptance criteria
- 右键展开不闪退
- 左键 reveal 主窗口语义保持不变
- strict mode 锁定时仍仅显示允许的状态与退出项

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
- service_impact: 仅影响本地 macOS 桌面 app 的 tray 右键菜单构造
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 VCS + 现有 dev runtime，可随时回退 `shell.rs` 本轮改动
- rollback_plan: 回退 macOS `Submenu` root 分支和共享 `populate_tray_menu` 提取，再重新 `cargo check` / `tauri dev`
- destructive_operations: none
- operator_approval_required: no
- rationale: 无数据/权限/外部副作用，仅修复本地桌面壳崩溃点

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 收敛 root cause，确认 macOS 顶层 tray menu 必须使用 `Submenu`
  - DoD: 阅读 `shell.rs`、`muda` 与 Tauri tray/menu 实现，并能给出可落地的平台分支方案
- [x] Task-2: 在 `shell.rs` 落地 macOS tray menu 修复
  - DoD: macOS 返回 `Submenu<R>`、其他平台保持 `Menu<R>`，共享菜单内容填充函数不改变动作 ID
- [x] Task-3: 收敛 tray 菜单一闪即逝的真实原因并修复后台刷新策略
  - DoD: `engine.rs` 不再每秒强制 `refresh_tray()`，而是改为按内容变化刷新
- [ ] Task-3: 完成现实验证与结案
  - DoD: 编译与 runtime 启动证据落盘；用户手工右键确认不再闪退后转 DONE

## Evidence Plan (UI / E2E)
- Evidence required: partial  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260403-macos-tray-menu-crash/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: 不适用（原生 tray 菜单）
  - empty: 不适用（至少保留状态项与退出项）
  - error: 右键 tray 时不应闪退或退出进程
  - disabled: `status` / `status-detail` 仍为禁用信息项
  - success: 右键菜单展开；左键 reveal 主窗口保持不变

## Observability / Debug Plan
- Logs: 记录 `cargo check`、`tauri dev`、`pgrep` 运行进程、`log show` 对进程仍存活的观察，以及 `osascript` assistive access 阻塞结果
- Error codes: macOS UI scripting 权限阻塞 `-25211`
- Trace/metrics (optional): 无
- Debug flags (optional): 无

## Risks & Rollback
- Risks: 缺少 assistive access 导致当前回合无法自动右键复测；tray refresh key 若设计过粗会让菜单状态更新不够及时
- Rollback plan: 回退 `shell.rs` 本轮 tray menu 平台分支与 docs，同步重新编译验证

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 阅读 `shell.rs` 与底层 `muda` / Tauri tray 文档，确认 macOS 顶层 menu 约束。
  2. 在 `shell.rs` 为 macOS 改用 `Submenu` 根菜单，并抽出共享菜单填充函数。
  3. 收敛后台 tick 秒级 `refresh_tray()` 导致菜单瞬时被重建的问题，并改为按内容变化刷新。
  4. 运行 `cargo check` 与 `npm --prefix apps/desktop run tauri dev`，保留进程与权限阻塞证据。
  5. 等待用户在已运行 app 上手工右键确认，随后结案。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: yes
