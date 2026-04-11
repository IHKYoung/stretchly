# Task-ID: TID-20260411-site-symbol-particles

## Summary
- Title: 官网强化终端提示头并改为符号粒子
- Date: 2026-04-11
- Level: trivial
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 强化首页左上角 `Pauza>` 的终端存在感，把粒子反馈改成更像程序员/码字工作者常见的符号字元，并让主输出文案宽度按浏览器页面宽度 `80%` 收敛。
- In-scope:
  - 强化 `Pauza>` 的字号、字重和分隔线
  - 把粒子从光点改成 `0 / 1 / #` 为主的符号字元
  - 让主输出区宽度按浏览器宽度 `80%` 收敛
  - 同步本任务 docs / evidence / CHANGELOG / UI / CodeMap / site README
- Out-of-scope:
  - 改动下载路由或文案池内容
  - 改动打字机节奏与提醒短句逻辑
  - 重新设计首页整体布局
- Assumptions:
  - 用户要的是更强的终端感，而不是更复杂的视觉装饰
  - 粒子仍应保持轻量，不能变成满屏特效
  - 输出区 80% 宽度应体现在主文案本身，而不只是舞台容器
- Risks:
  - prompt 过重会抢主文案视觉
  - 符号粒子若符号池过杂，会削弱“程序员感”
  - 输出区过宽会让长句失去节奏
- Interaction impact: direct
- Primary visible flow: 用户打开首页后，左上角先看到更醒目的 `Pauza>` prompt；中央主文案以浏览器宽度 `80%` 展示；点击或移动时冒出以 `0 / 1 / #` 为主的符号粒子。
- Fallback / secondary flow: reduced-motion 下仍保留更醒目的 prompt 和 `80%` 输出区，只弱化粒子动画。
- User-visible boundary: 仅限 `apps/site` 首页首屏的 prompt、输出区和粒子反馈。
- Key visible states / transitions: prompt-emphasized、output-width-80vw、symbol-particles-active

## Goal
- 让首页从“有终端味道”进一步收敛到“明显是写作者/开发者语境下的终端输出”。

## Scope
- In-scope:
  - `apps/site/styles.css`
  - `apps/site/script.js`
  - `apps/site/README.md`
  - `docs/specs/TID-20260411-site-symbol-particles/*`
  - `docs/{plans,logs}/2026-04-11.md`
  - `docs/{CHANGELOG,CodeMap,UI}.md`
- Out-of-scope:
  - `apps/site/index.html`
  - `apps/site/copy.js`
  - `apps/site/download/**`

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/site/index.html`
  - `apps/site/styles.css`
  - `apps/site/script.js`
- Related docs/specs/logs reviewed:
  - `apps/site/README.md`
  - `docs/UI.md`
  - `docs/CodeMap.md`
  - `docs/specs/TID-20260411-site-terminal-shell-refine/*`
  - `docs/specs/TID-20260411-site-interaction-refresh/*`
  - `docs/{logs,plans}/2026-04-11.md`
- Why these are sufficient:
  - 已覆盖首页当前结构、样式和交互脚本真源，以及前两轮官网任务的布局/交互边界，足以在不动信息架构的前提下强化终端感。

## Acceptance Criteria (AC)
- AC1: `Pauza>` 在首页左上角更醒目，字号、字重和终端分隔感明显增强。
- AC2: 首页主输出文案宽度等于当前浏览器页面宽度的 `80%`。
- AC3: 鼠标/点击产生的粒子以 `0 / 1 / #` 等符号字元为主，不再是圆点光粒。
- AC4: 既有打字机、点击提醒和下载按钮不受影响。
- AC5: DOM 度量、浏览器 console 摘要与 workflow docs validator 通过。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen-for-implementation
- Primary flow: 首页加载 -> 左上角 prompt 更醒目 -> 中央文案按 80vw 宽度展示 -> 点击/移动时出现符号粒子
- Fallback / secondary flow: reduced-motion 下保留 prompt 强度和输出宽度，弱化粒子动态
- Interaction authority / ownership boundary: 仅首页 prompt 样式、输出区宽度和粒子字元；不改下载按钮行为和文案池
- Visible entrypoints / handoff cues: 左上角 `Pauza>`、中央主文案、点击/移动反馈
- In-scope interactions:
  - prompt emphasis
  - output width 80vw
  - symbol particles
- Out-of-scope interactions:
  - 下载页重做
  - 文案池改写
  - 新增页面结构
- Interaction acceptance criteria:
  - prompt 明显增强但不压过主文案
  - 粒子以代码/写作语境里的符号为主
  - 输出区宽度变化不破坏整体居中秩序

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,ui_designer,coder,tester,scribe,evidence_collector
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限 `apps/site` 首页 prompt 样式、输出区宽度、粒子字元与相关 docs/evidence
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖本地静态服务器、DOM 度量、浏览器快照、console 摘要与 workflow docs validator
- rollback_plan: 回退 `apps/site/{styles.css,script.js}` 与相关 docs
- destructive_operations: none
- operator_approval_required: no
- rationale: 纯静态站点前端样式/脚本微调，不涉及线上服务、权限、数据或外部副作用

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 强化终端提示头
  - DoD: `Pauza>` 的字体权重、字号和右侧分隔线都明显增强
- [x] Task-2: 符号粒子化与输出宽度收敛
  - DoD: 粒子改为以 `0 / 1 / #` 为主的符号字元，主输出区宽度收敛到 `80vw`
- [x] Task-3: 现实检查与文档收口
  - DoD: DOM 度量、console 摘要、evidence 和文档同步完成

## Evidence Plan (UI / E2E)
- Evidence required: partial
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260411-site-symbol-particles/evidence/
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: 首页无 console error
  - disabled: reduced-motion 路径结构与宽度仍成立
  - success: prompt-emphasized、output-width-80vw、symbol-particles-active

## Observability / Debug Plan
- Logs: 浏览器 console、prompt 样式、输出区宽度比例、粒子字元采样
- Error codes: 不新增
- Trace/metrics (optional): N/A
- Debug flags (optional): `prefers-reduced-motion`

## Risks & Rollback
- Risks:
  - prompt 权重过大导致头重脚轻
  - 符号粒子过杂，削弱代码感
  - 输出宽度过宽导致长句节奏被冲淡
- Rollback plan:
  - 若终端感过重，优先回退 `styles.css` 中的 prompt 强化和输出宽度，再回退 `script.js` 的符号池

## Sequential Phases
- phase_execution: N/A
- phase_confirmation_policy: N/A
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 强化 prompt 样式并让输出区宽度收敛到 80vw
  2. 把粒子字元改成更偏 `0 / 1 / #` 的池
  3. 通过 DOM 度量、console 和 docs gate 验证后收口

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: no
- Approved: no-op（trivial 默认直行）
