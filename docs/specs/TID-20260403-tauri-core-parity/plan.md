# Task-ID: TID-20260403-tauri-core-parity

## Summary
- Title: Tauri 核心功能补齐并废弃默认 Electron 入口
- Date: 2026-04-03
- Level: complex
- Lane: deep
- Execution Profile: sequential-phases
- Status: DONE

## Requirement Brief
- Goal restatement: 将 `apps/desktop` 从“可运行的 Tauri MVP”推进到“可承担默认启动入口”的核心版本，把主入口切到 Tauri，并把 Electron 保留为 legacy 参考实现。
- In-scope:
  - 扩展 Rust host 的设置模型、调度状态、通知、pause/focus、skip/reset、manual finish、tray、shortcut 与 break window 策略。
  - 收敛前台到简洁设置页与 break prompt，恢复中文默认和资源化多语言。
  - 将根目录默认 npm scripts 切到 Tauri，Electron 改为 `legacy:*`。
  - 补齐对应 docs/specs/plans/logs/changelog/architecture/ui 记录。
- Out-of-scope:
  - 删除旧 Electron 代码主体。
  - Apple 生态联动、健康数据、日历集成、姿态检测。
  - 完整替换 Electron 分发资产与 CI。
- Assumptions:
  - 旧 Electron 继续作为 source basis 存在，便于后续比对和补 feature parity。
  - 当前阶段优先交付“默认入口可用”，不是一次性做完所有生态能力。
  - 允许把复杂设置折叠在 Advanced，而不是全部铺开在主界面。
- Risks:
  - 根脚本切换后，开发/构建链路容易与旧 Electron 构建配置并存，造成认知混乱。
  - tray、strict mode、manual-awaiting、multi-monitor break prompt 需要真实 runtime 证据，浏览器 preview 不能完全证明。
  - 轻量 `platform.rs` 探测精度仍弱于完全原生实现。
- Interaction impact: direct
- Primary visible flow: 打开设置窗口，调整休息/信号/高级策略，保存后立即反映到运行时；break 触发时进入独立 prompt。
- Fallback / secondary flow: 用户通过 tray 或全局快捷键执行 pause/focus/skip/reset/autostart，无需打开设置窗口。
- User-visible boundary: 主窗口、托盘菜单、break prompt、默认语言、根入口命令。
- Key visible states / transitions:
  - 主窗口 idle/paused/focus/dnd/app-rule 状态切换。
  - break prompt 普通倒计时与 `manualAwaiting` 状态切换。
  - strict break 下窗口关闭受限与 tray 菜单可见性切换。

## Goal
- 用 Tauri 2 接住旧 Electron 当前最重要的一组“核心产品能力”，让根仓库默认入口不再继续指向 Electron。

## Scope
- In-scope:
  - `apps/desktop/src-tauri/src/{state,shell,commands,engine,lib,platform,i18n}.rs`
  - `apps/desktop/src/{App.tsx,styles.css,locales/*,i18n.ts}`
  - 根 `package.json`
  - docs/specs/plans/logs/changelog/codemap/architecture/ui
- Out-of-scope:
  - 删除 `app/**`
  - electron-builder 构建配置清理
  - CI/workflow 与平台分发资产裁剪

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `package.json`
  - `app/main.js`
  - `app/breaksPlanner.js`
  - `app/utils/defaultSettings.js`
  - `app/utils/naturalBreaksManager.js`
  - `app/utils/dndManager.js`
  - `app/utils/appExclusionsManager.js`
  - `app/utils/displayManager.js`
  - `apps/desktop/src-tauri/src/lib.rs`
  - `apps/desktop/src-tauri/src/commands.rs`
  - `apps/desktop/src-tauri/src/engine.rs`
  - `apps/desktop/src-tauri/src/platform.rs`
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/styles.css`
- Related docs/specs/logs reviewed:
  - `docs/specs/TID-20260403-tauri-migration-foundation/*`
  - `docs/specs/TID-20260403-tauri-usable-core/*`
  - `docs/specs/TID-20260403-tauri-i18n-resource-layer/*`
  - `docs/plans/2026-04-03.md`
  - `docs/logs/2026-04-03.md`
  - `docs/Architecture.md`
  - `docs/UI.md`
- Why these are sufficient:
  - 它们覆盖了旧 Electron 的调度/信号/窗口边界，以及新 Tauri 端当前 host、前台、默认入口和相邻迁移阶段的上下文。

## Acceptance Criteria (AC)
- AC1: Tauri host 覆盖核心运行时能力，包括设置持久化、调度、pre-break notification、pause/focus、自然休息、DND、应用排除、tray、shortcut、break prompt 生命周期。
- AC2: Tauri 前台不再是展示页，而是极简设置页与独立 break prompt，默认中文并通过 i18n 资源读取文案。
- AC3: 根仓库默认 `start/dev/build/pack/dist` 指向 Tauri，旧 Electron 入口改为 `legacy:*`，但代码主体保留。
- AC4: 文档链条能追溯本次 parity 变更、验证结果、风险与剩余证据缺口。

## Interaction Freeze
- Freeze status: locked
- Primary flow: 简洁设置页只负责查看状态、修改设置、执行 pause/focus/resume。
- Fallback / secondary flow: tray 与快捷键直接操作运行时，不经过展示型中间页。
- Interaction authority / ownership boundary: 所有用户可见入口仅限主设置页、tray、break prompt。
- Visible entrypoints / handoff cues: 主窗口标题/状态条、toolbar、Advanced 折叠、tray submenu、break prompt action buttons。
- In-scope interactions:
  - General / Schedule / Signals / Advanced 设置修改
  - break done / postpone / skip
  - tray open/skip/focus/pause/resume/reset/autostart/quit
- Out-of-scope interactions:
  - dashboard 卡片陈列
  - 欢迎页/介绍页
  - 统计面板和 Today dashboard
- Interaction acceptance criteria:
  - 主设置页默认信息密度低于展示页版本。
  - break prompt 在普通与 manual-awaiting 两种状态下均能给出清晰的单一主 CTA。
  - tray 操作与窗口操作更新同一份运行时 snapshot。

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: sequential-phases
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,architect,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; 本任务实际因当时 developer policy 未获得用户显式授权而采用 `single-agent-fallback`
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 切换默认启动/构建入口到 Tauri，并在 Tauri host 中补齐核心久坐干预能力。
- touches_running_service: no
- backup_required: no
- backup_plan: 保留 `app/**` 与 `legacy:*` 脚本，依赖 VCS 回退。
- rollback_plan: 恢复根 `package.json` 默认脚本指向 Electron，并回退 `apps/desktop/**` 与对应 docs。
- destructive_operations: 废弃默认 Electron 入口；移除/覆盖展示型前台。
- operator_approval_required: no
- rationale: 用户已明确要求迁移到 Tauri，并继续迁全功能直到 Electron 可以退为 legacy。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 扩展 Rust host 设置模型与运行时状态
  - DoD: `PauzaSettings`、`DesktopSnapshot`、break action、manual finish、notification、shortcut bindings 与 reset/skip/pause/focus 能通过命令层暴露。
- [x] Task-2: 托盘、快捷键和 break window 行为补齐
  - DoD: tray 包含 open/skip/focus/pause/resume/reset/autostart/quit；strict break 下关闭行为与 tray 可见性正确。
- [x] Task-3: 前台收敛为简洁设置页
  - DoD: 主窗口只保留设置工作台与状态工具条；break 界面不再是迁移展示页。
- [x] Task-4: 默认入口切到 Tauri
  - DoD: 根 `package.json` 的 `start/dev/build/pack/dist/postinstall` 默认走 `apps/desktop`。
- [x] Task-5: 完成验证与文档收口
  - DoD: 有构建/类型/测试记录；docs 链条完整；证据缺口被显式记录。

## Evidence Plan (UI / E2E)
- Evidence required: partial
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260403-tauri-core-parity/evidence/
- Interaction validation note: 本任务直接影响主窗口、tray、break prompt 与默认入口，因此至少需要一份可视化/操作级证据报告；本轮文档补齐了报告，但尚缺独立截图/录屏。
- Required states to capture:
  - loading: 主窗口首次加载或浏览器 preview 初始态
  - empty: 无当前 break 的主设置页
  - error: `loadState` 或命令调用失败 banner
  - disabled: busy action 期间按钮/checkbox disabled
  - success: break prompt 正常态、manual-awaiting 态、tray submenu 可见态

## Observability / Debug Plan
- Logs:
  - `last_action` 记录 pause/focus/skip/reset/autostart/break action 来源。
  - `DesktopSnapshot.status/statusDetail` 暴露当前调度状态。
- Error codes:
  - 当前以字符串错误为主，`app_error()` 将宿主层异常上传给前台。
- Trace/metrics (optional):
  - 暂无独立 trace id；后续可在 `engine.rs` tick 与 `platform.rs` 探测层加结构化采样日志。
- Debug flags (optional):
  - 浏览器 preview 通过缺省 `invoke` 兜底，便于独立调试前台。

## Risks & Rollback
- Risks:
  - 仍存在 Electron 构建配置与 Tauri 默认构建链并存的认知成本。
  - 多显示器、strict break、tray-only 场景需要真实桌面交互验证。
  - `platform.rs` 的 DND/app exclusion 仍是轻量实现，不代表最终上限。
- Rollback plan:
  - 恢复根脚本到 Electron。
  - 回退 `apps/desktop/**` 的 parity 变更。
  - 文档层回退到 `TID-20260403-tauri-usable-core` 的描述边界。

## Sequential Phases
- phase_execution: sequential
- phase_confirmation_policy: no-intermediate-confirmation
- phase_stop_conditions:
  - 根入口切换影响超出开发环境
  - Tauri host 无法接住旧 Electron 的核心调度行为
  - 用户要求改变“极简设置页”交互冻结边界

## Execution Plan
- Steps:
  1. 读取旧 Electron 核心调度与新 Tauri 可用核心实现，识别仍缺的运行时能力。
  2. 扩展 `state.rs`、`shell.rs`、`commands.rs`、`engine.rs` 以接住 parity 所需状态和入口。
  3. 收敛前台为极简设置页与 break prompt。
  4. 将根仓库默认脚本切到 Tauri，保留 Electron 为 legacy。
  5. 运行构建/类型/测试并同步文档。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 默认入口已切到 Tauri，且旧 Electron 仍作为 legacy/source basis 可回退。
- 验证记录、风险与未补证据均可在 logs/plans/evidence 中追溯。

## Approval
- Approval needed: yes
- Approved: yes（用户在 2026-04-03 连续明确要求“继续”“把所有核心功能都迁移过来”“直到能用为止”“Electron 就可以废弃了”）
