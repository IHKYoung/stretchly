# Task-ID: TID-20260403-settings-sidebar-layout

## Summary
- Title: Tauri 设置页重构为侧边栏分类布局
- Date: 2026-04-03
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 将当前 Tauri 设置页改为“左侧分类导航 + 右侧分类详情”的单页结构，让设置内容更清楚，同时保留开源底座和当前 Tauri 已支持的设置边界。
- In-scope:
  - `apps/desktop/src/App.tsx` 的设置页布局重构
  - `apps/desktop/src/styles.css` 的三栏/单页信息架构样式补充
  - `apps/desktop/src/locales/{zh-CN,en}.json` 的分类与布局相关微文案
  - 本任务 specs / logs / plans / `docs/UI.md` / `docs/CHANGELOG.md`
- Out-of-scope:
  - Rust host / command contract / settings schema 变更
  - Electron legacy 偏好页实现
  - 新增设置项、主题系统、统计面板
- Assumptions:
  - “保留已有设置”以当前 Tauri `PauzaSettings` 字段为准，并参考旧 `preferences.html` 的分类结构
  - 用户当前要求的是信息架构重排，不是恢复旧底座所有尚未迁入 Tauri 的设置能力
  - break prompt 可以维持现状，只要主设置页完成重构
- Risks:
  - 分类重排时遗漏某些当前已存在的设置控件
  - 侧边栏布局在窄屏下可读性下降
- Interaction impact: direct
- Primary visible flow: 主窗口打开后，通过侧边栏选择分类，在单页内查看和编辑对应设置，再从右侧 save rail 保存
- Fallback / secondary flow: 缩窄窗口后分类区与内容区堆叠，但设置仍可浏览、修改、保存
- User-visible boundary: `apps/desktop/src/App.tsx` 主设置页
- Key visible states / transitions: loading、category-active、dirty、synced、error、narrow-layout

## Goal
- 让 Pauza 的主设置页更像桌面原生偏好页：左边知道“去哪里找”，中间知道“当前在改什么”，右边知道“现在是否已保存”。

## Scope
- In-scope:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/styles.css`
  - `apps/desktop/src/locales/zh-CN.json`
  - `apps/desktop/src/locales/en.json`
  - `docs/specs/TID-20260403-settings-sidebar-layout/*`
  - `docs/UI.md`
  - `docs/CHANGELOG.md`
- Out-of-scope:
  - `apps/desktop/src-tauri/src/*.rs`
  - `app/preferences.html`
  - 新依赖或新的运行时设置字段

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/styles.css`
  - `apps/desktop/src/locales/zh-CN.json`
  - `apps/desktop/src-tauri/src/state.rs`
  - `app/preferences.html`
  - `app/utils/defaultSettings.js`
- Related docs/specs/logs reviewed:
  - `AGENTS.md`
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `docs/UI.md`
  - `docs/logs/2026-04-03.md`
  - `docs/plans/2026-04-03.md`
  - `docs/specs/TID-20260403-hig-tailwind-redesign/{plan,ui}.md`
- Why these are sufficient:
  - 已覆盖当前 Tauri 设置入口、现有样式和文案、宿主支持字段、旧底座分类模式，以及相邻的前台改造历史，足以在不改 host 的前提下完成本轮侧边栏重构。

## Acceptance Criteria (AC)
- AC1: 主设置页改为明确的侧边栏分类导航，用户能在一个页面内切换类别查看设置。
- AC2: 当前 Tauri `PauzaSettings` 已暴露的设置项全部仍然保留，并归入合理分类，没有擅自新增新的设置能力。
- AC3: 保存、撤销、默认值和状态摘要仍可见，且 `dirty/synced` handoff 在新布局下依旧明确。
- AC4: `npm --prefix apps/desktop run typecheck` 与 `npm --prefix apps/desktop run build` 通过，主设置页完成一轮 UI 自检。

## Interaction Freeze
- Freeze status: frozen
- Primary flow: `打开设置页 -> 选择侧边栏分类 -> 编辑当前分类内容 -> 在右侧 save rail 保存`
- Fallback / secondary flow: `缩窄窗口 -> sidebar/content/rail 堆叠 -> 继续编辑与保存`
- Interaction authority / ownership boundary: 仅 React 前台视图层与微文案；设置 schema、状态读取和保存命令仍由 Rust host 提供
- Visible entrypoints / handoff cues:
  - sidebar category pills
  - section title + section description
  - save rail dirty/synced badge
- In-scope interactions:
  - 侧边栏分类切换
  - 既有设置控件的重排
  - save/revert/defaults
- Out-of-scope interactions:
  - break prompt 改版
  - tray / notification / shortcut backend 行为调整
  - Electron legacy 页面
- Interaction acceptance criteria:
  - 当前激活分类可明确识别
  - 分类切换后用户能在主内容区看到完整相关设置
  - save rail 在任何分类下都保持可见或可达

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,architect,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅重构 Tauri 主设置页的信息架构和前台文案，不修改 settings persistence、调度逻辑或外部依赖
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 VCS 回退，并用 build / UI 自检作为旁证
- rollback_plan: 回退 `apps/desktop/src/{App.tsx,styles.css,locales/*}` 与本任务 docs
- destructive_operations: 替换当前 settings page 的布局结构与部分微文案，但不删宿主支持字段
- operator_approval_required: no
- rationale: 本轮没有运行中服务、数据删除、提权或新增依赖风险，属于前台布局重排

## Task Breakdown & Definition of Done (DoD)
- [ ] Task-1: 设计并实现 sidebar category 信息架构
  - DoD: 主设置页形成稳定的左中右布局，分类切换可用
- [ ] Task-2: 将现有设置字段按分类重排且不丢项
  - DoD: 当前 `PauzaSettings` 中已暴露字段都仍可编辑
- [ ] Task-3: 跑构建验证并补文档
  - DoD: typecheck/build 通过，logs/plans/specs/UI/changelog 可追溯

## Evidence Plan (UI / E2E)
- Evidence required: partial
- Owner: orchestrator（single-agent-fallback）
- Artifact path: docs/specs/TID-20260403-settings-sidebar-layout/evidence/
- Required states to capture:
  - loading: 可用静态文案旁证，不强制截图
  - empty: N/A
  - error: error banner 不专门造假触发，保留代码审查
  - disabled: save 按钮在 `dirty=false` 时禁用
  - success: 主设置页 sidebar 布局截图或 snapshot

## Observability / Debug Plan
- Logs:
  - Vite/build/typecheck 输出
- Error codes:
  - 无新增错误码，前台继续使用 error banner 直显
- Trace/metrics (optional):
  - N/A
- Debug flags (optional):
  - 浏览器 preview 继续作为设置页快速自检入口

## Risks & Rollback
- Risks:
  - 某些高级项在新分类下可发现性不足
  - 侧边栏在窄屏下可能过于拥挤
- Rollback plan:
  - 回退 `apps/desktop/src/{App.tsx,styles.css,locales/*}` 到本轮前的 grouped settings 版本

## Sequential Phases
- phase_execution: N/A
- phase_confirmation_policy: N/A
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 落盘 Standard task docs，明确分类模型和 AC
  2. 重构 `App.tsx` 的 settings page 结构与分类导航
  3. 调整样式和文案
  4. 执行 typecheck/build 与必要的 UI 自检
  5. 回填 logs/plans/changelog/UI 文档

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT`
- 新侧边栏设置页能完整展示当前已支持的设置项
- 至少完成一轮构建验证和一次 UI 自检
- logs/plans/specs/UI/changelog 可追溯到本次实现

## Approval
- Approval needed: yes
- Approved: yes（用户本轮明确要求按侧边栏分类方式重构设置页；Execution Safety Block 无额外高风险人工门禁）
