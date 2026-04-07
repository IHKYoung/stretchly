# Task-ID: TID-20260403-settings-fixed-split

## Summary
- Title: 设置页固定 1:3 分栏与 16:9 最小窗口
- Date: 2026-04-03
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 修正当前 Tauri 设置页“侧边栏 + 主体 + save rail”三栏带来的别扭感，让左侧导航稳定占据主窗口约四分之一，右侧主体占据剩余约四分之三，同时把主窗口默认尺寸与最小尺寸拉回适合桌面偏好页的 16:9 区间；并额外收紧过度圆角和冗长介绍文案。
- In-scope:
  - `apps/desktop/src/App.tsx` 的主设置页外层布局重构
  - 将保存/状态区并回主体列，不再作为全局第三列
  - 压缩 hero / section / sidebar 中的冗长介绍文案
  - 收紧 desktop 前台 primitives 与设置页局部组件的大圆角
  - `apps/desktop/src-tauri/tauri.conf.json` 的主窗口默认尺寸与最小尺寸调整
  - 本任务 specs / evidence / 当天 logs / plans / `docs/UI.md` / `docs/CHANGELOG.md`
- Out-of-scope:
  - break prompt 结构与样式
  - Rust host 命令、settings schema、tray 行为或调度逻辑
  - 新增设置项、文案大改或新的信息分类
- Assumptions:
  - 用户认可现有侧边栏分类模型，只要求把全局空间分配修正为稳定的桌面 split view
  - 主窗口最小尺寸提高后，不再需要为桌面交付态保留“窄屏堆叠主路径”
  - 浏览器 preview 仍可作为开发旁证，但最终交付以 Tauri 主窗口尺寸约束为准
- Risks:
  - save/status dock 并回主体后，概览页的状态摘要会与 overview 卡片产生一定信息重复
  - 提高最小窗口尺寸会减少小屏设备的可压缩范围，但这是本次布局稳定性的必要代价
  - 统一收紧圆角后，break prompt 等前台局部也会一起变得更克制
- Interaction impact: direct
- Primary visible flow: 打开设置页后，左侧看到稳定的分类导航；右侧主体顶部看到保存/状态 dock；切换分类时，右侧持续展示当前分类详情与保存动作。
- Fallback / secondary flow: 浏览器 preview 在低于 `1280px` 时仍可能进入堆叠态，但 Tauri 主窗口通过最小尺寸限制避免桌面交付态落入该分支。
- User-visible boundary: `apps/desktop/src/App.tsx` 主设置页与 `apps/desktop/src-tauri/tauri.conf.json` 主窗口尺寸约束
- Key visible states / transitions: sidebar-fixed-split、category-active、save-dirty、save-synced、desktop-min-window-enforced

## Goal
- 让 Pauza 的设置页重新成为稳定、克制的桌面双栏偏好页，而不是一个被第三列压缩、又带大量圆角和介绍文案的面板。

## Scope
- In-scope:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/components/ui/{button,card,input,select,segmented-control}.tsx`
  - `apps/desktop/src-tauri/tauri.conf.json`
  - `docs/specs/TID-20260403-settings-fixed-split/*`
  - `docs/plans/2026-04-03.md`
  - `docs/logs/2026-04-03.md`
  - `docs/UI.md`
  - `docs/CHANGELOG.md`
- Out-of-scope:
  - `apps/desktop/src/locales/*`
  - `apps/desktop/src-tauri/src/*.rs`
  - `?window=break` break prompt copy/flow
  - 旧 Electron 设置页

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/styles.css`
  - `apps/desktop/src/components/ui/{card,button,input,select,segmented-control}.tsx`
  - `apps/desktop/src-tauri/tauri.conf.json`
- Related docs/specs/logs reviewed:
  - `AGENTS.md`
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `docs/UI.md`
  - `docs/plans/2026-04-03.md`
  - `docs/logs/2026-04-03.md`
  - `docs/specs/TID-20260403-settings-sidebar-layout/{plan,ui}.md`
- Why these are sufficient:
  - 已覆盖当前设置页的真实 DOM 结构、Tauri 主窗口尺寸入口、前台 primitives、现有 UI 文档和今天刚完成的 sidebar-layout 任务边界，足以定位“别扭感”来源是三栏分配、过小窗口约束、过度圆角和冗长介绍，而不是 host/schema 问题。

## Acceptance Criteria (AC)
- AC1: 在桌面主窗口默认/最小尺寸下，设置页形成稳定的两栏布局，左侧导航约占 `1/4`，右侧主体约占 `3/4`。
- AC2: 保存/状态区不再以全局第三列占据窗口，而是并回主体列且保持可见、可用。
- AC3: Tauri 主窗口默认尺寸与最小尺寸调整到接近 `16:9`，避免桌面交付态跌回窄屏堆叠布局。
- AC4: 桌面前台的大圆角和冗长介绍文案明显收敛，界面回到更工具化的表达。
- AC5: `npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build` 通过，并产出一份 `1440x810` 视口下的 UI 证据截图。

## Interaction Freeze
- Freeze status: frozen
- Primary flow: `打开设置页 -> 左侧选分类 -> 右侧主体查看当前分类详情 -> 在主体顶部 dock 保存 / 撤销 / 恢复默认值`
- Fallback / secondary flow: `浏览器 preview 小视口仅作开发旁证；桌面主窗口通过 min size 避免进入该路径`
- Interaction authority / ownership boundary: 仅前台布局层与主窗口尺寸约束；设置读写命令与 runtime 状态继续由既有 host 命令提供
- Visible entrypoints / handoff cues:
  - 左侧固定 sidebar category buttons
  - 主体顶部 save/status dock
  - 当前分类 hero 与 section cards
- In-scope interactions:
  - category 切换
  - 保存 / 撤销 / 默认值
  - 固定 split layout 下的内容浏览
- Out-of-scope interactions:
  - break prompt
  - tray / shortcut / notification runtime
  - settings schema 或字段增删
- Interaction acceptance criteria:
  - 用户打开窗口后，不再看到全局第三列压缩主体
  - 当前分类与保存状态在同一主体区域内保持清晰 handoff
  - 提高最小窗口后，桌面主窗口默认保持 split view

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
- service_impact: 仅调整主设置页前台布局、desktop primitives 圆角和 Tauri 主窗口尺寸约束，不改设置存储、调度逻辑或外部副作用
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 VCS 回退，并用 `typecheck/build` 与 screenshot 作为改动旁证
- rollback_plan: 回退 `apps/desktop/src/App.tsx`、`apps/desktop/src/components/ui/*`、`apps/desktop/src-tauri/tauri.conf.json` 及本任务 docs/evidence
- destructive_operations: 替换当前全局三栏布局、旧的较小最小窗口尺寸，以及 desktop 前台的一批高圆角 primitives，但不删除任何设置字段或宿主命令
- operator_approval_required: no
- rationale: 本轮只处理桌面前台布局比例、窗口基础尺寸和视觉噪音，不涉及数据、权限、网络或运行中服务风险

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 将设置页外层从全局三栏改为固定 split view
  - DoD: sidebar 占比稳定，主体不再被单独 save rail 压缩
- [x] Task-2: 提高主窗口默认尺寸与最小尺寸
  - DoD: `tauri.conf.json` 的主窗口改为接近 `16:9` 的默认/最小尺寸
- [x] Task-3: 收紧桌面前台的过度圆角与冗长介绍
  - DoD: settings page 的 hero / section / sidebar 文案明显减少，Card/Button/Input/Select/SegmentedControl 圆角整体下降一档
- [x] Task-4: 完成构建验证与截图证据
  - DoD: typecheck/build 通过，并产出 `1440x810` 视口截图、snapshot 与 console log

## Evidence Plan (UI / E2E)
- Evidence required: partial
- Owner: orchestrator（single-agent-fallback）
- Artifact path: docs/specs/TID-20260403-settings-fixed-split/evidence/
- Required states to capture:
  - loading: 不单独造假，沿用现有简单 loading 文案实现
  - empty: N/A
  - error: 不主动注入错误，保留代码审查
  - disabled: 同步态下 save 按钮保持 disabled，可通过 screenshot + DOM snapshot 旁证
  - success: `1440x810` 视口下的 split layout screenshot 与 DOM snapshot

## Observability / Debug Plan
- Logs:
  - Vite build 输出
  - Playwright browser console log
- Error codes:
  - 无新增错误码；布局错误主要通过浏览器 console、typecheck 和 build 暴露
- Trace/metrics (optional):
  - N/A
- Debug flags (optional):
  - 浏览器 preview 继续作为主设置页视觉检查入口

## Risks & Rollback
- Risks:
  - save/status dock 并回主体后，概览页会存在轻微信息重复
  - 更大的最小窗口会牺牲一部分小尺寸压缩自由度
- Rollback plan:
  - 恢复旧的 `App.tsx` 三栏布局、恢复旧的 desktop primitives 样式和 `tauri.conf.json` 的窗口尺寸，并删除本任务 evidence/docs

## Sequential Phases
- phase_execution: N/A
- phase_confirmation_policy: N/A
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 建立本任务 docs 并记录 Requirement Brief / Safety Block / Source Basis
  2. 将设置页改为 `1fr : 3fr` 的 split view，并移除全局第三列
  3. 调整主窗口默认与最小尺寸到 `16:9`
  4. 收紧前台 primitives 的圆角并裁掉冗长说明文案
  5. 执行 typecheck/build
  6. 用浏览器 preview 采集 `1440x810` 证据并回填 docs

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- split layout、窗口尺寸、圆角/文案收紧和验证证据都可从 logs/plans/specs 追溯。

## Approval
- Approval needed: yes
- Approved: yes（用户已明确要求把设置页改回稳定的 `1/4 + 3/4` 分栏，并提高最小窗口尺寸；后续又明确要求减少圆角和废话式介绍）
