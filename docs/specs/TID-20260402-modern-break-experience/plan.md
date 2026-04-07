# Task-ID: TID-20260402-modern-break-experience

## Summary
- Title: 更温和的打断策略与现代化界面
- Date: 2026-04-02
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 把 Pauza 从“硬提醒工具”往“对深度工作更友好的久坐干预产品”推进，先解决强打断和视觉陈旧的问题。
- In-scope: break 打断策略、托盘主流程、welcome / preferences / break 界面现代化、必要设置项与文案。
- Out-of-scope: Apple 生态联动、健康数据同步、外部账号体系、复杂统计面板。
- Assumptions: 用户当前最痛的点是全屏/强打断导致的跳过；现有 idle / DND / exclusions 能力足够支撑单 app 内的第一阶段优化。
- Risks: 新增交互模式可能与既有 `fullscreen` / `showBreaksAsRegularWindows` 语义重叠；需要控制兼容性，避免破坏严格模式。
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: 托盘进入 Focus session 或等待 break 触发 -> 看到更轻量的现代卡片 -> 优先选择稍后再来 / 完成休息，而不是直接跳过。
- Fallback / secondary flow: 保留 strict mode 和既有 schedule 语义；用户仍可使用原有暂停/恢复路径。
- User-visible boundary: welcome、preferences、tray menu、mini break、long break。
- Key visible states / transitions: gentle/balanced/immersive 模式切换；mini/long break 卡片显示；focus session 激活；postpone/skip/finish 按钮状态变化。

## Goal
- 让默认体验更温和、更现代，同时保留高约束用户需要的严格模式。

## Scope
- In-scope:
  - 新增 `Interruption style` 设置与默认值
  - 托盘增加 `Focus session` 快捷入口
  - 调整 mini / long break 的窗口尺寸、位置、聚焦与展示策略
  - break / welcome / preferences 三个主要界面的现代化视觉
  - 更新必要英文与中文文案
- Out-of-scope:
  - Apple Health / 日历 / 设备联动
  - 新的存储后端或远程同步
  - 全量多语言翻译补齐

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `app/main.js`
  - `app/breaksPlanner.js`
  - `app/microbreak.html`
  - `app/break.html`
  - `app/microbreak-renderer.js`
  - `app/break-renderer.js`
  - `app/preferences.html`
  - `app/preferences-renderer.js`
  - `app/welcome.html`
  - `app/welcome-renderer.js`
  - `app/css/break.css`
  - `app/css/preferences.css`
  - `app/css/welcome.css`
  - `app/utils/defaultSettings.js`
  - `app/utils/displayManager.js`
  - `app/utils/statusMessages.js`
- Related docs/specs/logs reviewed:
  - `AGENTS.md`
  - `docs/RepositoryGuidelines.md`
  - `docs/Architecture.md`
  - `docs/UI.md`
  - `docs/logs/2026-04-02.md`
- Why these are sufficient:
  - 已覆盖 break 状态机、窗口创建链路、设置页绑定链路、样式入口和当前产品结构，足以支撑本次 UI/交互重构。

## Acceptance Criteria (AC)
- AC1: 用户可在设置页选择新的干预风格，且默认风格明显比当前更温和。
- AC2: mini break 不再默认呈现为大面积强打断，视觉上为更现代的轻量卡片。
- AC3: tray 中新增主动保护心流的入口，用户可一键暂停固定时长专注工作。
- AC4: break/welcome/preferences 三个关键界面视觉更统一、更现代，且不破坏现有核心功能。
- AC5: 现有测试与 lint 通过，新增逻辑具备最小自动化覆盖。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen-for-implementation
- Primary flow: 正常工作中收到 gentle/balanced/immersive 对应强度的 break 卡片，用户优先通过 postpone / finish / skip-for-now 处理。
- Fallback / secondary flow: strict mode、pause/resume、skip-to-next 等既有 tray 路径继续可用。
- Interaction authority / ownership boundary: 本次仅调整本地桌面 UI 与干预方式，不改变 breaksPlanner 的基础排程语义。
- Visible entrypoints / handoff cues: welcome 首次印象、preferences 配置入口、tray 快捷入口、break action button。
- In-scope interactions: 选择干预风格、进入 focus session、查看并处理 mini/long break 卡片。
- Out-of-scope interactions: contributor auth / sync、远端页面、外部设备联动。
- Interaction acceptance criteria: gentle 模式下用户不再被默认全屏强打断；break action 语义更柔和清晰；界面视觉统一。
- Validator expectation: 交互相关字段已具备明确边界，不保留占位值。

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
- service_impact: local desktop UI / UX only
- touches_running_service: no
- backup_required: no
- backup_plan: 以当前 git diff 为回滚边界
- rollback_plan: 回滚本任务修改的 `app/**`、`test/**`、`docs/**`
- destructive_operations: none
- operator_approval_required: no
- rationale: 不涉及用户数据迁移、网络副作用或破坏性系统操作

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 实现更温和的打断策略与托盘 focus session
  - DoD: 设置项、默认值、tray menu、break window 行为全部接通
- [x] Task-2: 重做 break / welcome / preferences 的现代化视觉
  - DoD: 三个界面视觉统一、按钮与排版焕新、核心交互不回退
- [x] Task-3: 补测试与文档
  - DoD: lint/test 通过，spec/logs/plans 无占位

## Evidence Plan (UI / E2E)
- Evidence required: partial  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260402-modern-break-experience/evidence/
- Interaction validation note: 当前以本地验证命令和文件级变更说明为主，不单独采集 UI 证据文件；若后续需要发布评审，再补截图。
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: lint/test fail 时记录
  - disabled: strict mode / hidden action button
  - success: gentle 模式 break card、tray focus session、modernized preferences/welcome

## Observability / Debug Plan
- Logs: 继续复用 `electron-log` 中的 break 生命周期、settings 变更与 tray 更新日志
- Error codes: 无新增错误码
- Trace/metrics (optional): 暂不新增
- Debug flags (optional): 复用现有 debug info 面板

## Risks & Rollback
- Risks:
  - gentle 模式尺寸和位置在多显示器上可能需要微调
  - 新样式可能影响旧的 DOM 选择器或窗口高度计算
- Rollback plan:
  - 回滚到本任务前的 break/welcome/preferences 样式和 `app/main.js` 行为逻辑

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 新增 interruption style 设置与默认值，并接通 tray / break 窗口行为
  2. 改造 break HTML/CSS/renderer 为现代卡片界面
  3. 改造 welcome / preferences 样式与必要结构
  4. 跑 lint/test 并补 logs/plans

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: yes（用户在讨论后明确要求“你来改一下，并且我希望整体的设计更优雅更现代”）
