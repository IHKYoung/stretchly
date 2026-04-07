# Task-ID: TID-20260403-tauri-usable-core

## Summary
- Title: Tauri 可用核心闭环
- Date: 2026-04-03
- Level: complex
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 把 Tauri 2 从“可启动的基础壳”推进到“能实际承担久坐干预”的最小可用产品闭环。
- In-scope: Rust host 内的设置持久化、调度引擎、pause/focus、自然休息、DND、应用排除、tray/shortcut、break prompt；前台改成极简设置页和 break prompt；保留浏览器 preview 兜底；docs/evidence 同步。
- Out-of-scope: 替换 Electron 正式入口；完整统计 dashboard；平台级健康生态集成；桌面自动化 E2E。
- Assumptions: 用户接受 Tauri 端先提供可用核心，而不是一次性追平 Electron 全量 feature；跨平台信号检测允许先用 `sysinfo + 系统命令` 这种较轻实现。
- Risks: DND/idle 的跨平台探测精度暂不如原 Electron 侧成熟；preview 与原生 runtime 仍有差异；dev 态首轮 Rust 编译较慢。
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: 用户打开设置页，查看当前状态与下一次休息，调整节奏、启用自然休息/DND/应用排除，并用 Pause/Focus/Resume 快捷操作控制干预强度。
- Fallback / secondary flow: break prompt 独立窗口在休息开始时弹出，用户可完成、推迟或跳过；浏览器 preview 使用同一套页面结构做静态预览。
- User-visible boundary: 所有可见改动仅发生在 `apps/desktop/**`；旧 Electron UI 不在本轮替换范围内。
- Key visible states / transitions: running -> paused/focus -> running；next break waiting -> break prompt active -> done/postpone/skip；DND/app exclusion/natural break 触发的自动暂停状态。

## Goal
- 为 Pauza 的跨平台产品重构交付一个可实际使用的 Tauri 可用核，而不是继续停留在迁移展示层。

## Scope
- In-scope:
  - `apps/desktop/src-tauri/src/state.rs` 作为设置与调度真源
  - `engine.rs` / `platform.rs` 负责后台 tick、系统信号探测与 break 触发
  - `commands.rs` / `shell.rs` 负责 tray、shortcut、主窗口与 break 窗口命令边界
  - `src/App.tsx` / `src/styles.css` 改为极简设置页与 break prompt
  - 相关 docs / evidence / changelog / codemap 同步
- Out-of-scope:
  - 完整 today 面板与长期统计
  - Apple Health、日历、升降桌等生态集成
  - 替换旧 Electron 产品分发链

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `app/breaksPlanner.js`
  - `app/utils/naturalBreaksManager.js`
  - `app/utils/dndManager.js`
  - `app/utils/appExclusionsManager.js`
  - `app/utils/defaultSettings.js`
  - `apps/desktop/src-tauri/src/lib.rs`
  - `apps/desktop/src-tauri/src/commands.rs`
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/styles.css`
  - `apps/desktop/src-tauri/tauri.conf.json`
- Related docs/specs/logs reviewed:
  - `AGENTS.md`
  - `docs/specs/TID-20260403-tauri-migration-foundation/*`
  - `docs/CodeMap.md`
  - `docs/RepositoryGuidelines.md`
  - `docs/Architecture.md`
  - `docs/UI.md`
- Why these are sufficient:
  - Electron 侧文件覆盖了必须保留的行为边界；现有 Tauri 壳文件覆盖了宿主层对接点；前一阶段 spec 明确了迁移边界，足以支持本轮从“基础壳”进入“可用闭环”。

## Acceptance Criteria (AC)
- AC1: Tauri host 持久化设置并驱动可运行的调度状态机，覆盖 microbreak / long break、pause、focus、natural break、DND、app exclusions。
- AC2: 主窗口、tray、全局快捷键和 break prompt 之间的命令边界全部接通，`tauri:dev` 能实际跑到 `target/debug/pauza-desktop`。
- AC3: `apps/desktop/src/App.tsx` 不再是迁移展示页，而是极简设置工作台；`?window=break` 下展示独立 break prompt。
- AC4: `cargo check`、`typecheck`、前端构建与 workflow docs validator 全部通过，docs 能准确描述当前 Tauri 能力边界。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen for usable-core scope
- Primary flow: 打开设置页后，顶部看到当前状态和下一次休息时间；中部用少量字段配置节奏、行为、系统信号和 app rules；底部只保留 Revert / Defaults / Save。
- Fallback / secondary flow: break prompt 独立成小窗，只呈现标题、说明、倒计时和 Done / Postpone / Skip。
- Interaction authority / ownership boundary: 本轮只定义产品核心设置和 break prompt，不引入营销文案、迁移进度、legacy source 列表或复杂 dashboard 组件。
- Visible entrypoints / handoff cues: 主窗口、tray 菜单、快捷键、自动弹出的 break 窗口。
- In-scope interactions: 查看状态；保存设置；Pause / Focus / Resume；toggle launch on login；完成/推迟/跳过休息。
- Out-of-scope interactions: 今日统计、趋势分析、周报、多人协作、深度个性化卡片。
- Interaction acceptance criteria: 桌面设置页结构明显收敛；break prompt 动作可用；preview 不崩溃；用户不需要先看“迁移仪表盘”才能使用产品。

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
- service_impact: 在 Tauri 端新增可运行的久坐干预核心闭环，但不接管当前 Electron 正式入口。
- touches_running_service: no
- backup_required: no
- backup_plan: 保留旧 Electron 行为与前一阶段 Tauri foundation 为 source basis，所有更改通过 VCS 可回退。
- rollback_plan: 回退 `apps/desktop/**`、相关 docs 和 changelog，即可恢复到“只有基础壳”的状态。
- destructive_operations: 删除原 `apps/desktop/src/App.tsx` 里的迁移展示逻辑，并用极简产品设置页替换。
- operator_approval_required: no
- rationale: 用户明确要求继续推进到“能用为止”，而本轮仅改 Tauri 迁移壳，不触碰现有生产 Electron 主体。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 在 Rust host 中落地真实调度引擎和系统信号层
  - DoD: `state.rs`、`engine.rs`、`platform.rs` 能覆盖设置、调度、pause/focus、idle、DND、app exclusions。
- [x] Task-2: 把前台改成极简设置页与独立 break prompt
  - DoD: `App.tsx` 同时覆盖主设置窗口与 `?window=break` prompt，且去掉迁移展示内容。
- [x] Task-3: 完成运行验证和 docs/evidence 同步
  - DoD: build/check/dev runtime 均通过，spec/logs/plans/codemap/architecture/UI/changelog 与证据产物同步完成。

## Evidence Plan (UI / E2E)
- Evidence required: partial  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260403-tauri-usable-core/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: 主窗口首屏可见且不再渲染迁移仪表盘
  - empty: 无 active break 时的默认设置页
  - error: 浏览器 preview 缺失 Tauri runtime 时不崩溃，只显示 preview 文案
  - disabled: 未修改设置时 Save / Revert 禁用态
  - success: break prompt 独立窗口结构和可执行按钮

## Observability / Debug Plan
- Logs: `DesktopSnapshot.lastAction`、`status`、`statusDetail` 作为轻量可视化调试面；命令统一返回 `Result<_, String>`。
- Error codes: 当前仍使用字符串错误；复杂错误码模型留待后续统计与同步功能阶段引入。
- Trace/metrics (optional): `cargo check` / `npm run build` / `tauri:dev` stdout 作为启动与集成自证；浏览器 console 记录保存在 evidence。
- Debug flags (optional): 当前无额外 debug flag；浏览器 preview 是唯一保留的静态 debug 入口。

## Risks & Rollback
- Risks:
  - 平台探测当前依赖系统命令与 `sysinfo`，边缘平台可能存在精度差异
  - break prompt 窗口行为仍需后续真机细调
  - 双壳阶段会让仓库维护成本短期上升
- Rollback plan:
  - 回退 `apps/desktop/**` 到上一阶段基础壳版本
  - 删除新增 docs/specs/evidence 并恢复 `docs/Architecture.md` / `docs/UI.md` / `docs/CodeMap.md`

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 审阅 Electron planner / signal 模块，确定最小迁移闭环。
  2. 在 Tauri host 中实现设置、调度、系统信号、tray 和 break window。
  3. 重写前台为极简设置页与 break prompt。
  4. 编译、运行、采证并同步 docs。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: 用户于 2026-04-03 明确要求继续推进“直到能用为止”，并接受以 Tauri 2 作为新桌面主体。
