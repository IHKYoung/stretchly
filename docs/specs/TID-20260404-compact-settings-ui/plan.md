# Task-ID: TID-20260404-compact-settings-ui

## Summary
- Title: 压缩设置窗口与重做紧凑型设置界面
- Date: 2026-04-04
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 将当前设置页从偏展示型的大面板重做为更小巧、更平、更密的桌面工具窗口，让用户在更小的主窗口里也能看清并完成核心设置。
- In-scope: 重构 `apps/desktop/src/App.tsx` 的设置页布局；压平 `button/select/segmented-control/switch/textarea` 的基础控件样式；引入自动保存并去掉显式保存栏；收紧 `styles.css` 背景；继续削减侧栏/顶部状态杂讯；下调 Tauri 主窗口默认与最小尺寸；补齐本任务 docs/evidence。
- Out-of-scope: 不新增设置项；不调整 `PauzaSettings` 数据契约；不修改 break window 视觉语义和调度逻辑；不恢复 overview、快捷键编辑或运行时动作页。
- Assumptions: 用户确认当前核心设置集合已经收敛；更小主窗口仍允许通过分类切换与必要滚动完成设置；现有国际化 key 足够承载本轮重排。
- Risks: 过度压缩可能导致部分文案或控件在低宽度下拥挤；主窗口尺寸下调后若控件密度不够，仍会显得空和笨重。
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: 用户打开主窗口后，在左侧窄导航切换 `节奏 / 提醒与打断 / 智能暂停 / 通用`，在右侧单一主面板内直接编辑当前分类设置并保存。
- Fallback / secondary flow: 当窗口降到最小尺寸时，内容区保持滚动，不引入二级弹窗或隐藏面板。
- User-visible boundary: `apps/desktop` 主设置窗口整体视觉、间距、控件密度与默认尺寸。
- Key visible states / transitions: 已同步/有未保存变更；分类切换；禁用态按钮；错误提示条。

## Goal
- 让 Tauri 设置页看起来更像“偏好设置工具窗口”，而不是大而散的展示页。

## Scope
- In-scope:
  - 将设置页收为窄侧栏 + 单一主面板的紧凑结构。
  - 移除主面板外层大卡片观感，压缩标题、留白和按钮尺寸。
  - 将按钮、选择器、分段控件、开关和多行输入改为更平直的小圆角样式。
  - 将设置保存改为自动保存，去掉显式 `保存 / 撤销` 操作。
  - 将主窗口默认尺寸调整为 `1040x585`，最小尺寸调整为 `960x540`。
  - 落盘快照证据与当日文档。
- Out-of-scope:
  - break prompt 的后续视觉重做。
  - 后端状态机、tray、shortcut、notification 逻辑。
  - 新增设置分类或高级设置入口。

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/styles.css`
  - `apps/desktop/src/components/ui/{button,card,select,segmented-control}.tsx`
  - `apps/desktop/src-tauri/tauri.conf.json`
- Related docs/specs/logs reviewed:
  - `docs/UI.md`
  - `docs/CHANGELOG.md`
  - `docs/plans/2026-04-04.md`
  - `docs/logs/2026-04-04.md`
- Why these are sufficient: 本轮只调整前台布局、基础控件与主窗口尺寸；上述文件已覆盖视觉结构、窗口约束和历史收口点，没有触及新的后端契约。

## Acceptance Criteria (AC)
- AC1: 设置页主结构收敛为窄侧栏 + 单一主面板，去掉大卡片壳、展示型背景和大面积留白，分类切换与设置变更仍可用。
- AC2: 基础控件样式明显收平，按钮、选择器、分段控件、开关与数字输入在视觉上更接近桌面工具窗口，而不是软塌塌的圆角卡片。
- AC3: Tauri 主窗口默认与最小尺寸收紧到更小的工具窗口级别，且前端仍能通过 `typecheck/build`。
- AC4: 设置页改为自动保存后，顶部不再需要显式保存按钮，只保留轻量同步状态。
- AC5: 留下可追溯证据，说明当前紧凑设置页在浏览器预览下能够稳定渲染。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen for implementation
- Primary flow: 左侧窄导航切分类，右侧单面板查看和修改当前分类设置，顶部保持保存/撤销。
- Fallback / secondary flow: 缩小到最小尺寸时以滚动维持可达性，不引入额外栏位或弹层。
- Interaction authority / ownership boundary: 仅重排主设置窗口布局与控件密度；设置项语义、命令名和 break 运行时交互不在本轮变更范围。
- Visible entrypoints / handoff cues: 主窗口打开即进入设置页；侧栏分类高亮；顶部 `保存 / 撤销`；错误提示条。
- In-scope interactions: 分类切换、开关切换、数字输入、分段控件选择、下拉选择、保存/撤销。
- Out-of-scope interactions: break 窗口 CTA、tray 菜单、快捷键编辑、运行时动作。
- Interaction acceptance criteria: 用户在更小主窗口内仍可完成四个分类设置；页面不再呈现“展示页 / 多卡片 / 大英雄区”观感，侧栏和顶部不再充满运行时状态噪音；设置变更自动保存，不再打断用户。
- Validator expectation: 当 `interaction_impact != none` 时，本节与 Requirement Brief 中的交互字段不得继续保留 `N/A/TBD`

### Interaction acceptance criteria
- 用户在 `1120x630` 默认窗口内仍能看清四个分类并完成保存。
- 页面不再出现第三列 save rail、大卡片外壳和展示型背景。

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,ui_designer,architect,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅影响 Tauri 主设置窗口视觉结构和默认尺寸。
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 VCS、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build` 与浏览器快照作为回归旁证。
- rollback_plan: 回退 `App.tsx`、基础 UI 组件、`styles.css`、`tauri.conf.json` 与本任务 docs。
- destructive_operations: 替换现有设置页布局与主窗口尺寸配置。
- operator_approval_required: no
- rationale: 用户已经明确要求直接推翻当前设置页的前端表达；本轮不涉及数据、权限、外部副作用或运行中服务写操作。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 重做设置页容器、导航和设置行密度
  - DoD: `App.tsx` 改为更窄的工具窗口结构，设置页不再有外层大卡片观感。
- [x] Task-2: 压平基础控件与背景
  - DoD: `button/select/segmented-control/switch/textarea/styles.css` 收紧圆角、阴影、背景和尺寸。
- [x] Task-3: 引入自动保存并继续削减顶部/侧栏杂讯
  - DoD: 设置变更自动持久化，设置页顶部不再依赖显式保存按钮。
- [x] Task-4: 收紧主窗口尺寸并补验证证据
  - DoD: `tauri.conf.json` 更新到更小尺寸，typecheck/build 与 evidence 文档完成。

## Evidence Plan (UI / E2E)
- Evidence required: yes  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260404-compact-settings-ui/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A（当前预览直接进入已加载状态）
  - empty: N/A
  - error: 错误提示条保留在实现中，本轮不主动制造错误态
  - disabled: `保存 / 撤销` 在无脏数据时为 disabled
  - success: 默认预览页快照与控制台日志

## Observability / Debug Plan
- Logs: 不新增运行时日志；沿用现有前端错误提示条和构建输出。
- Error codes: N/A
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 仍可能有用户认为个别分类页在 `960x540` 下不够紧凑。
  - 由于字体加载超时，PNG 截图未能稳定产出，只能以结构快照留证。
- Rollback plan:
  - 回退 `App.tsx`、`styles.css`、基础 UI 组件和 `tauri.conf.json`。
  - 重新运行 typecheck/build 确认回退没有引入额外问题。

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 重构设置页为窄侧栏 + 单主面板结构，压缩标题、留白和设置行。
  2. 调整基础 UI 控件为更平直的小圆角工具样式。
  3. 下调 Tauri 主窗口尺寸并执行 typecheck/build。
  4. 采集浏览器快照与控制台日志，更新 docs/CHANGELOG/UI/plans/logs。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: user direct request in-thread
