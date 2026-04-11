# Task-ID: TID-20260411-site-interaction-refresh

## Summary
- Title: 官网打字机停留与交互增强
- Date: 2026-04-11
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 在保持当前极简官网方向不变的前提下，增强首页打字机节奏、舞台居中布局和用户可感知的互动反馈，让页面从“好看”提升到“有手感”。
- In-scope:
  - 打字机停留时长从 5 秒改为 10 秒
  - 首页主舞台改为更明确的水平居中，桌面端宽度约占视口 80%
  - 增加鼠标移动/滚动/点击的粒子或光晕反馈
  - 增加点击触发的调侃提醒短句气泡
  - 扩充官网前端使用的调侃提醒文案池
  - 同步本任务 docs / evidence / CHANGELOG / CodeMap / UI
- Out-of-scope:
  - 改动下载路由结构
  - 引入新的构建栈或第三方前端依赖
  - 改成多页官网或加入大段 marketing 文案
- Assumptions:
  - 用户要的是“极简但不无聊”，不是更复杂的信息架构
  - 交互应保持轻量、克制，不应把纸面风格做成炫技 landing page
  - 静态站点应继续保持无依赖
- Risks:
  - 交互加太多会破坏目前的纯净感
  - 粒子效果若不控制数量，容易让页面发脏或影响性能
  - 点击提醒气泡若过长，会让舞台重新显得偏乱
- Interaction impact: direct
- Primary visible flow: 用户打开首页，看到居中的 80% 舞台，打字机文案逐字出现并在打完后停留 10 秒；移动鼠标时页面给出轻微跟随反馈；点击时页面给出粒子爆发和一条调侃提醒。
- Fallback / secondary flow: 当系统偏好 `prefers-reduced-motion` 时，首页保留居中舞台和首条文案，但弱化/关闭持续粒子与过强动效，只保留最小可理解的点击反馈。
- User-visible boundary: 仅限 `apps/site` 首页首屏及其交互反馈。
- Key visible states / transitions: home-idle、typewriter-typing、typewriter-hold-10s、pointer-aura、pointer-trail、click-burst、nudge-bubble、reduced-motion-fallback

## Goal
- 让首页在不增加信息负担的情况下，获得更明确的居中秩序和“点一下就会回嘴”的产品个性。

## Scope
- In-scope:
  - `apps/site/index.html`
  - `apps/site/styles.css`
  - `apps/site/script.js`
  - `apps/site/copy.js`
  - `docs/specs/TID-20260411-site-interaction-refresh/*`
  - `docs/{plans,logs}/2026-04-11.md`
  - `docs/{CHANGELOG,CodeMap,UI}.md`
- Out-of-scope:
  - `apps/site/download/**`
  - `apps/desktop/**`
  - 真实下载 URL 配置

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/site/index.html`
  - `apps/site/styles.css`
  - `apps/site/script.js`
  - `apps/site/copy.js`
  - `apps/desktop/src/locales/messages/zh-CN.json`
- Related docs/specs/logs reviewed:
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `docs/logs/2026-04-11.md`
  - `docs/plans/2026-04-11.md`
  - `docs/specs/TID-20260411-site-landing-page/*`
  - `docs/specs/TID-20260411-site-typewriter-redesign/*`
- Why these are sufficient:
  - 已覆盖当前官网的真实 HTML/CSS/JS 真源、既有文案池以及前两轮官网任务的边界，足以在不改站点结构的前提下增强节奏、交互和布局。

## Acceptance Criteria (AC)
- AC1: 首页打字机文案打完后会停留约 10 秒，再退格切换到下一条。
- AC2: 桌面端主舞台保持水平居中，整体宽度接近视口的 80%，文本视觉重心不再偏左。
- AC3: 鼠标移动、滚动或点击时，页面会出现轻量且不过度喧宾夺主的互动反馈。
- AC4: 点击首页空白处或主舞台时，会弹出一条调侃式“别久坐”短句，文案池数量明显多于当前版本。
- AC5: 移动端仍保持单屏稳定展示，不因新增交互而出现内容溢出或滚动异常。
- AC6: `npm test`、浏览器现实检查与 workflow docs validator 通过。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen-for-implementation
- Primary flow: 打开首页 -> 阅读居中打字机文案 -> 鼠标移动获得轻微跟随感 -> 点击后出现粒子和调侃提醒
- Fallback / secondary flow: reduced-motion 下保留静态中心舞台和最小点击反馈，不保留高频持续动画
- Interaction authority / ownership boundary: 仅 `apps/site` 首页首屏负责；下载按钮继续只是入口，不参与本轮互动反馈
- Visible entrypoints / handoff cues: 首页中央打字机舞台、舞台下方轻提示文案、右上角下载按钮
- In-scope interactions:
  - typed hold timing
  - centered stage layout
  - pointer aura / trail
  - click burst particle
  - click-to-nudge bubble
- Out-of-scope interactions:
  - 下载页互动重做
  - 二级页面滚动故事线
  - 音频或震动反馈
- Interaction acceptance criteria:
  - 交互存在感明显增强，但首页仍然保持纯白、克制和单页极简
  - 粒子效果不会遮挡主文案，点击气泡不会长期滞留
  - reduced-motion 路径可读且可用

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,ui_designer,architect,coder,tester,scribe,evidence_collector
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限 `apps/site` 首页布局、交互脚本、文案池与相关 docs/evidence
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖本地静态服务器、浏览器现实检查、截图证据、`npm test` 与 workflow docs validator
- rollback_plan: 回退 `apps/site/{index.html,styles.css,script.js,copy.js}` 与相关 docs
- destructive_operations: none
- operator_approval_required: no
- rationale: 本轮只改纯静态站点前端，不涉及线上服务、权限、数据或外部副作用

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 重构首页舞台布局与节奏
  - DoD: 主舞台视觉上明确居中，桌面端宽度接近 80%，打字机 hold 时间变为 10 秒
- [x] Task-2: 增加交互反馈与点击调侃提醒
  - DoD: 鼠标/滚动/点击有轻量反馈，点击能弹出短句气泡且不破坏极简感
- [x] Task-3: 浏览器验证与文档收口
  - DoD: 桌面/移动端现实检查通过，evidence、CHANGELOG、CodeMap、UI 与当日 docs 更新完成

## Evidence Plan (UI / E2E)
- Evidence required: yes
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260411-site-interaction-refresh/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: 下载按钮与首页无控制台报错
  - disabled: reduced-motion fallback（如适用）
  - success: centered-stage、typewriter-hold、pointer-feedback、click-nudge

## Observability / Debug Plan
- Logs: 浏览器 console、首页交互行为、页面尺寸与滚动状态
- Error codes: 不新增
- Trace/metrics (optional): N/A
- Debug flags (optional): `prefers-reduced-motion`

## Risks & Rollback
- Risks:
  - 粒子数量过多导致视觉噪音
  - pointer-follow 过强导致页面发飘
  - 新短句若长度控制不好，会在小屏上遮挡主内容
- Rollback plan:
  - 若交互过重，优先回退粒子/气泡逻辑，只保留 10 秒 hold 和居中布局

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 冻结首页交互契约、验收标准和现实检查范围
  2. 调整首页 HTML/CSS，收敛为更强居中的 80% 舞台
  3. 重写前端脚本，加入 10 秒 hold、粒子反馈和点击调侃提醒
  4. 扩充文案池并做浏览器现实检查
  5. 更新 evidence / docs / changelog

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes
- Approved: yes（用户已明确要求直接继续优化官网首页交互）
