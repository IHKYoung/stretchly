# Task-ID: TID-20260403-tauri-migration-foundation

## Summary
- Title: Tauri 2 迁移基础骨架
- Date: 2026-04-03
- Level: complex
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 为 Pauza 的跨平台重构建立一套独立于 Electron 的 Tauri 2 基础壳，先把宿主层和新的主工作台跑起来，再承接后续领域逻辑迁移。
- In-scope: 新建 `apps/desktop`；接入 Tauri 2 基础配置、Rust host、tray/global shortcut/notification/autostart、最小 React dashboard、浏览器 preview 兜底、根脚本入口与 docs 同步。
- Out-of-scope: 迁移 break planner、idle、DND、app exclusions 的完整产品逻辑；移除 Electron 主体；签名、公证与正式分发产线。
- Assumptions: 用户已明确选择跨平台路线，并接受以 Tauri 2 替代继续投资 Electron；旧仓库代码暂时作为迁移 source basis，而不是保留为未来宿主架构。
- Risks: Tauri 官方插件尚未覆盖全部宿主信号；短期内存在 Electron/Tauri 双壳并存；浏览器 preview 只能验证 UI 和交互文案，不能替代原生宿主能力验证。
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: 用户启动新的 Tauri 主窗口，查看宿主层能力、迁移队列和 legacy source basis，并可触发 focus session / toggle autostart / hide dashboard。
- Fallback / secondary flow: 在纯浏览器打开 Vite dev URL 时进入 preview 模式，只用于 UI 预览与交互文案验证。
- User-visible boundary: 变化只发生在新增的 `apps/desktop` 迁移壳，不替换现有 Electron 入口。
- Key visible states / transitions: 初始 dashboard、focus session active、autostart toggle、preview runtime、hide dashboard 动作反馈。

## Goal
- 为 Pauza 2.0 建立跨平台 Tauri 2 桌面基础骨架，并完成一次可运行、可验证、可继续迁移的落地。

## Scope
- In-scope:
  - `apps/desktop` Tauri 2 + React + TypeScript scaffold 定制
  - `src-tauri/src/lib.rs`、`commands.rs`、`shell.rs`、`state.rs` 的宿主壳与状态快照
  - 根目录 `package.json` 的 Tauri 入口脚本
  - 浏览器 preview 兼容与最小交互模拟
  - 迁移 specs / plans / logs / code map / architecture / UI / changelog 更新
- Out-of-scope:
  - 替换现有 Electron 生产入口
  - Rust 侧真正接管 break planner / idle / DND / app exclusions
  - 完整自动化桌面 E2E、签名、公证与正式安装包策略

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `app/main.js`
  - `app/breaksPlanner.js`
  - `app/utils/naturalBreaksManager.js`
  - `app/utils/dndManager.js`
  - `app/utils/appExclusionsManager.js`
  - `package.json`
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/styles.css`
  - `apps/desktop/src-tauri/tauri.conf.json`
  - `apps/desktop/src-tauri/src/lib.rs`
  - `apps/desktop/src-tauri/src/commands.rs`
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src-tauri/src/state.rs`
- Related docs/specs/logs reviewed:
  - `AGENTS.md`
  - `docs/CodeMap.md`
  - `docs/RepositoryGuidelines.md`
  - `docs/Architecture.md`
  - `docs/UI.md`
  - `docs/specs/TID-20260403-repo-cleanup-foundation/*`
- Why these are sufficient:
  - 旧 Electron 代码覆盖了后续迁移必须保留的领域边界；新 `apps/desktop/**` 代码覆盖了 Tauri 壳的实现与交互；工作流文档足以约束本次基础骨架的交付边界。

## Acceptance Criteria (AC)
- AC1: 仓库中存在独立的 `apps/desktop` Tauri 2 桌面应用，并能通过 `typecheck`、前端构建和 `cargo check`。
- AC2: 新壳在 Rust host 侧提供可运行的 tray、global shortcut、notification、autostart 和主窗口命令边界。
- AC3: 新 dashboard 在 Tauri runtime 与浏览器 preview 下都能正常渲染，不再因缺失 `invoke` 环境而直接报错。
- AC4: 仓库索引、迁移文档、daily logs/plans 与 changelog 完整反映这次 Tauri 基础迁移。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen for foundation scope
- Primary flow: 打开 Tauri dashboard，查看 shell 能力与迁移队列，触发 focus session / autostart / hide dashboard 等宿主层动作。
- Fallback / secondary flow: 用浏览器访问 `http://127.0.0.1:1420` 时进入 preview mode，仅展示 UI 和本地模拟动作。
- Interaction authority / ownership boundary: 本轮只定义 dashboard 级交互与宿主层动作，不定义真正 break session、planner 规则或系统信号流。
- Visible entrypoints / handoff cues: 主窗口 hero CTA、metrics、capability list、migration queue、footer 状态文案；tray 和全局快捷键作为原生入口。
- In-scope interactions: 查看状态、启动/清除 focus session、切换 autostart、隐藏 dashboard、查看 legacy sources。
- Out-of-scope interactions: 完整休息流程、久坐统计、真实 idle/DND/app exclusion 反馈、正式设置中心。
- Interaction acceptance criteria: Tauri runtime 下动作可触发相应命令；preview 下动作至少给出一致的 UI 状态反馈；布局在桌面和窄屏下都保持可读。

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
- service_impact: 新增平行 Tauri 壳层与根命令入口，不替换当前 Electron 主体。
- touches_running_service: no
- backup_required: no
- backup_plan: 旧 `app/**`、`test/**` 与 Electron 打包链保持原样；新工作通过 VCS 可回退。
- rollback_plan: 删除 `apps/desktop/**`、回退根脚本入口与 docs 更新即可恢复到纯 Electron 状态。
- destructive_operations: 删除 Tauri 脚手架残留文件（`.vscode/extensions.json`、未使用图标）与此前确认无价值的外围资产。
- operator_approval_required: no
- rationale: 用户已明确要求以跨平台为前提迁移到 Tauri 2，本轮不碰现有主应用的数据与运行时。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 建立 `apps/desktop` Tauri 2 宿主壳
  - DoD: `tauri.conf.json`、Rust host、React dashboard、根命令入口与图标资源可用。
- [x] Task-2: 补浏览器 preview 兜底与交互状态
  - DoD: 纯浏览器访问 Vite dev URL 不再报 `invoke` 运行时错误，CTA 能给出本地模拟反馈。
- [x] Task-3: 同步 docs / specs / code map / architecture / changelog
  - DoD: 所有相关文档不再保留 `TBD/INIT`，并可通过 workflow validator。

## Evidence Plan (UI / E2E)
- Evidence required: partial  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260403-tauri-migration-foundation/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: dashboard 首次进入的基础布局和数据占位
  - empty: 无 focus session 时的默认态
  - error: 浏览器 preview 兜底后不再抛 runtime 崩溃，只显示 preview 文案
  - disabled: busyAction 期间按钮禁用态
  - success: 触发 focus session 后的剩余时间与 footer 状态

## Observability / Debug Plan
- Logs: `DesktopSnapshot.lastAction` 作为轻量可视化状态日志；Rust commands 统一返回 `Result<_, String>`，便于前端直接展示错误。
- Error codes: 当前阶段未引入独立错误码，使用字符串错误与前端 error banner；后续迁移 planner 时再补结构化错误模型。
- Trace/metrics (optional): 通过 `cargo check` / `tauri dev` stdout 和 Vite preview 观察宿主层启动情况。
- Debug flags (optional): 当前无额外 debug flag，保留最小骨架。

## Risks & Rollback
- Risks:
  - 宿主层能力与后续领域迁移之间仍有一层待实现的信号服务
  - 浏览器 preview 只能验证 UI，不等于真实桌面 runtime
  - 双壳阶段会增加仓库结构复杂度
- Rollback plan:
  - 回退 `apps/desktop/**`、根 `package.json` 的 desktop scripts，以及本次 docs 变更
  - 继续以 Electron 作为唯一桌面入口，直到下一轮路线重启

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 从官方 Tauri 2 React scaffold 建立 `apps/desktop` 基础目录，并替换 branding、package scripts 与配置。
  2. 在 Rust host 内建立 `PauzaState`、commands、tray/shortcut/autostart 壳层。
  3. 用 React dashboard 承接新的产品工作台，并显示迁移队列与 legacy source basis。
  4. 为浏览器 preview 增加兜底快照与本地模拟动作，避免前端预览直接报错。
  5. 更新 docs、收集证据并完成验证。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: 用户于 2026-04-03 明确确认跨平台方向，并接受直接迁移到 Tauri 2。
