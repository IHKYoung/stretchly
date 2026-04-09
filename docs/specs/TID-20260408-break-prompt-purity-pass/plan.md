# Task-ID: TID-20260408-break-prompt-purity-pass

## Summary
- Title: 收敛休息界面并清理 break prompt 文案层
- Date: 2026-04-08
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 把当前 Tauri break prompt 从“左右分栏 + cue 卡片 + 圆环”的展示型界面收回到纯净休息界面，只保留一句交互语、一个大号数字倒计时和一条细进度条，并把交互语和缺失文案全部收进 locale 资源。
- In-scope:
  - `apps/desktop/src/App.tsx` 的 break prompt 布局与倒计时呈现
  - `apps/desktop/src/i18n.ts` 的 locale 数组读取能力
  - `apps/desktop/src/lib/break-prompt.ts` 的 prompt 选择逻辑
  - `apps/desktop/src/locales/{zh-CN,en}.json` 的 break prompt 文案与缺失 key
  - 本任务 specs / daily logs / plans，以及 `docs/{UI,CodeMap,CHANGELOG}.md`
- Out-of-scope:
  - Rust host 状态机、window shell、fullscreen/window 宿主策略
  - settings schema 与 break sound / wallpaper 持久化字段
  - Electron legacy break 页面
- Assumptions:
  - 当前用户不需要在休息界面继续看到“微休息 / 休息 / cue / 倒计时”这类显式标签
  - 高级感取向是“更少组件感、更少装饰、更强留白”，而不是继续叠加功能块
  - `breakIdeasEnabled` 继续保留为“是否轮播交互语”的设置语义，只是呈现方式从卡片改为单句 prompt
- Risks:
  - 若 locale 中 prompt 数组缺失，会退回默认 prompt；需要保证双语资源齐全
  - 纯净布局如果过度简化，可能让手动完成状态缺少解释，因此需保留 manual awaiting 的兜底文案
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: break 弹出后，用户看到居中的单句交互语、数字倒计时和细条形进度条，底部保留必要 CTA。
- Fallback / secondary flow: 若关闭交互语轮播或 locale prompt 数组缺失，则回退到按 break kind 的默认 prompt；若进入 manual finish，倒计时归零并显示等待完成文案。
- User-visible boundary: 仅 Tauri `?window=break` break prompt 和偏好页中“交互语 / 开始音”相关命名。
- Key visible states / transitions:
  - running: 单句 prompt + 数字倒计时 + 条形进度
  - prompt-rotation-off: 使用默认 prompt，不显示随机轮播差异
  - manual-awaiting: 数字倒计时归零，提示转为等待完成
  - optional-clock-on: 仅在设置开启时于右上角显示系统时间

## Goal
- 让 break prompt 回到真正的“休息界面”而不是“功能演示页”，并把 break prompt 文案源头统一收回 i18n 目录。

## Scope
- In-scope:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/i18n.ts`
  - `apps/desktop/src/lib/break-prompt.ts`
  - `apps/desktop/src/locales/{zh-CN,en}.json`
  - `docs/specs/TID-20260408-break-prompt-purity-pass/*`
  - `docs/{plans,logs}/2026-04-08.md`
  - `docs/{UI,CodeMap,CHANGELOG}.md`
- Out-of-scope:
  - `apps/desktop/src-tauri/src/*`
  - 旧 Electron break 页面与 legacy locale
  - 新增依赖、音频资源或 settings schema 字段

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/i18n.ts`
  - `apps/desktop/src/lib/break-prompt.ts`
  - `apps/desktop/src/locales/{zh-CN,en}.json`
  - `app/{break.html,microbreak.html,break-renderer.js,microbreak-renderer.js}`
- Related docs/specs/logs reviewed:
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `docs/UI.md`
  - `docs/logs/2026-04-08.md`
  - `docs/plans/2026-04-08.md`
  - `docs/specs/TID-20260408-break-prompt-parity-upgrade/{plan,ui}.md`
- Why these are sufficient:
  - 已覆盖当前 Tauri break prompt 的布局、prompt helper、locale 结构和最近一轮 parity 改造来源，足以判断哪些内容是“功能补齐”残留，哪些需要重新做减法。

## Acceptance Criteria (AC)
- AC1: break prompt 不再呈现左右分栏、圆形倒计时、cue 标签卡片和 break kind 标签，主视觉收敛为单列居中布局。
- AC2: break prompt 使用“数字倒计时 + 细条形进度条”的单一倒计时体系，不再保留圆环方案。
- AC3: break prompt 主文案显示为一句交互语；开启轮播时从 locale prompt 数组中取值，关闭时回退到 locale 默认 prompt。
- AC4: `apps/desktop/src/lib/break-prompt.ts` 不再硬编码中文/英文交互语文案，文案统一由 `apps/desktop/src/locales/*.json` 管理。
- AC5: `ui.microbreakStartSound` / `ui.longBreakStartSound` 等缺失 key 补齐后，设置页不再泄漏原始字段名。
- AC6: `npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build` 与 `python3 scripts/validate_workflow_docs.py --mode manual` 通过。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen before code
- Primary flow: break 自动弹窗后，用户直接面对一条交互语和倒计时；CTA 仅作为底部动作区存在。
- Fallback / secondary flow: prompt 轮播关闭或资源缺失时，回退到默认 prompt；manual finish 时倒计时归零并显示等待完成提示。
- Interaction authority / ownership boundary: 仅限 React 前台 break prompt 与 locale 资源层；不改 host 调度、break window 宿主和 CTA 语义。
- Visible entrypoints / handoff cues: 运行时 `?window=break`；偏好页中 `交互语` 开关和开始音下拉。
- In-scope interactions:
  - 阅读单句 prompt
  - 观看数字倒计时与条形进度
  - 点击完成 / 稍后 / 跳过
  - 在设置页切换交互语轮播与开始音选项
- Out-of-scope interactions:
  - 修改提醒模式
  - 编辑自定义 prompt 列表
  - 调整宿主窗口尺寸 / fullscreen 策略

### Interaction acceptance criteria
- 宽屏与窄屏均保持单列阅读顺序
- break 界面默认不再出现“微休息 / 休息 / cue / countdown”标签化块
- 交互语文案只能来自 locale 资源而非 TS 常量

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
- service_impact: 仅限本地 Tauri 前台 break prompt 呈现与 locale 资源层
- touches_running_service: no
- backup_required: no
- backup_plan: 以 Git diff、前端 build/typecheck 和 workflow docs validation 作为回滚边界
- rollback_plan: 回退 `apps/desktop/src/{App.tsx,i18n.ts,lib/break-prompt.ts,locales/*}` 与本任务 docs
- destructive_operations: 替换现有 break prompt 视觉结构并删除 prompt 硬编码
- operator_approval_required: no
- rationale: 纯前台 UI / i18n 收敛，无数据迁移、提权、外部副作用或运行中服务风险

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 收敛 break prompt 交互方案
  - DoD: 明确采用“单列 + 数字倒计时 + 条形进度”并写入本任务 plan / ui spec
- [x] Task-2: 改造 break prompt 结构
  - DoD: `App.tsx` 不再保留双栏、圆环和 cue card，改为纯净单列布局
- [x] Task-3: 抽离 prompt 文案到 locale
  - DoD: `break-prompt.ts` 不再硬编码 prompt copy，locale 提供 prompt 数组与默认 prompt
- [x] Task-4: 完成构建验证与文档同步
  - DoD: build / typecheck / docs validation 通过，daily logs/plans 与长期文档同步

## Evidence Plan (UI / E2E)
- Evidence required: partial  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260408-break-prompt-purity-pass/evidence/
- Interaction validation note: 以浏览器/Tauri preview 证明单列 break prompt 已去掉双栏与圆环；manual awaiting 与 prompt-off fallback 以代码路径审查为辅。
- Required states to capture:
  - loading: N/A（沿用现有前台 loading，不是本轮重点）
  - empty: prompt 轮播关闭时回退默认 prompt
  - error: locale 缺失时不应直接露出 key
  - disabled: CTA 在 busyAction 期间遵循既有 disabled 规则
  - success: 单列 prompt + 数字倒计时 + 条形进度

## Observability / Debug Plan
- Logs: 本轮不新增 runtime logger；优先通过 build/typecheck、React preview 和 locale lookup 检查 break prompt 文案来源
- Error codes: 无新增 error code
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 数字倒计时与条形进度若不同步，会导致 break prompt 可信度下降
  - locale prompt 数组若后续被误删，界面可能退回默认 prompt
- Rollback plan:
  - 直接回退 `App.tsx`、`i18n.ts`、`break-prompt.ts`、locale 与本任务 docs，恢复上一版 parity 布局

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 把 break prompt 视觉决策固定为单列纯净布局，并在 plan/ui spec 中写明取舍
  2. 改写 `App.tsx` 的 break prompt，移除双栏、圆环、cue 卡片与显式 break kind 标签
  3. 在 `i18n.ts` 增加 locale 数组读取，迁走 `break-prompt.ts` 中的 prompt 文案硬编码
  4. 补齐缺失翻译 key，并同步 UI/CodeMap/CHANGELOG 与 daily docs
  5. 运行前端构建和 workflow docs 校验

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: 用户在当前线程明确要求把休息界面改回“纯净的休息界面”，并直接要求在条形/圆形倒计时中做产品设计取舍，同时要求 break prompt 文案迁到 i18n 文件夹统一管理
