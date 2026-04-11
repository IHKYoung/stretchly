# Task-ID: TID-20260411-site-font-unify-symbol-density

## Summary
- Title: 官网统一 LXGW 字体并增强符号粒子密度
- Date: 2026-04-11
- Level: trivial
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 让 `Pauza>` 和整个官网都统一使用 `LXGW WenKai Screen`，并把粒子收敛到更大、更密、更多 `0 / 1 / # / @ / ！ / ¥ / $` 这类符号。
- In-scope:
  - 统一首页所有可见文字到 `LXGW WenKai Screen`
  - 让 `Pauza>` 也使用同一字体
  - 增加粒子的字号、数量和符号密度
  - 保持主输出区 `80vw` 宽度不变
  - 同步本任务 docs / evidence / CHANGELOG / UI / CodeMap / site README
- Out-of-scope:
  - 改动首页文案池内容
  - 改动下载路由与站点结构
  - 改动打字机节奏
- Assumptions:
  - 用户要的是“字体完全统一”，不是保留终端 monospace 特例
  - 粒子应更有代码感，但不该演变成压过主文案的满屏特效
  - `80vw` 输出区已经是正确边界，本轮只修字体和粒子
- Risks:
  - 全站单字体后，终端 prompt 的“命令行味道”会更依赖字重和排版
  - 粒子过多可能让页面显得发脏
- Interaction impact: direct
- Primary visible flow: 用户打开首页后，`Pauza>`、主文案、下载按钮和互动提示都以同一套 `LXGW` 字体展示；点击或移动时会看到更大更密、以 `0 / 1 / # / @ / ！ / ¥ / $` 为主的符号粒子。
- Fallback / secondary flow: reduced-motion 下仍保留统一字体和同一套字符粒子语义，只弱化动态数量感。
- User-visible boundary: 仅限 `apps/site` 首页的文字样式和粒子反馈。
- Key visible states / transitions: single-font-lxgw、prompt-lxgw、symbol-particles-dense、output-width-80vw-retained

## Goal
- 让首页从“终端感”进一步收敛到“同一种书写气质统一覆盖整页，并且粒子语义也更统一”。

## Scope
- In-scope:
  - `apps/site/styles.css`
  - `apps/site/script.js`
  - `apps/site/README.md`
  - `docs/specs/TID-20260411-site-font-unify-symbol-density/*`
  - `docs/{plans,logs}/2026-04-11.md`
  - `docs/{CHANGELOG,CodeMap,UI}.md`
- Out-of-scope:
  - `apps/site/index.html`
  - `apps/site/copy.js`
  - `apps/site/download/**`

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/site/styles.css`
  - `apps/site/script.js`
  - `apps/site/index.html`
- Related docs/specs/logs reviewed:
  - `apps/site/README.md`
  - `docs/UI.md`
  - `docs/CodeMap.md`
  - `docs/specs/TID-20260411-site-symbol-particles/*`
  - `docs/specs/TID-20260411-site-terminal-shell-refine/*`
  - `docs/{logs,plans}/2026-04-11.md`
- Why these are sufficient:
  - 已覆盖首页当前字体分配、粒子字元池和前两轮官网终端风格边界，足以在不改结构的前提下完成字体统一与粒子密度增强。

## Acceptance Criteria (AC)
- AC1: 首页所有可见文字都统一使用 `LXGW WenKai Screen`。
- AC2: `Pauza>` 继续保持终端 prompt 感，但字体与整站一致。
- AC3: 粒子数量和字号明显提升，并且点击时稳定出现 `0 / 1 / # / @ / ！ / ¥ / $` 这一组符号。
- AC4: 主输出区宽度仍保持浏览器页面宽度的 `80%`。
- AC5: 浏览器 console、DOM 度量与 workflow docs validator 通过。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen-for-implementation
- Primary flow: 首页加载 -> 全站文字统一 LXGW -> 点击或移动时出现更大更密的指定符号粒子
- Fallback / secondary flow: reduced-motion 下仍保留统一字体与同一组字符语义，只弱化动态反馈
- Interaction authority / ownership boundary: 仅首页字体分配、粒子密度和粒子字元池；不改布局和文案池
- Visible entrypoints / handoff cues: `Pauza>`、下载按钮、互动提示、粒子反馈
- In-scope interactions:
  - single-font site
  - prompt font unify
  - dense symbol particles
- Out-of-scope interactions:
  - 下载页改造
  - 文案池改写
  - 新增特效层
- Interaction acceptance criteria:
  - 全站不再混用第二套字体
  - `Pauza>` 仍清晰可辨为 prompt
  - 粒子密度提高但不破坏整体克制感

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
- service_impact: 仅限 `apps/site` 首页字体、粒子密度与相关 docs/evidence
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖本地静态服务器、DOM 度量、浏览器快照、console 摘要与 workflow docs validator
- rollback_plan: 回退 `apps/site/{styles.css,script.js}` 与相关 docs
- destructive_operations: none
- operator_approval_required: no
- rationale: 纯静态站点前端样式/脚本微调，不涉及线上服务、权限、数据或外部副作用

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 统一首页字体
  - DoD: `Pauza>`、下载按钮、提示文案、粒子都使用 `LXGW WenKai Screen`
- [x] Task-2: 增强粒子密度与符号覆盖
  - DoD: 粒子更大更密，点击时稳定出现 `0 / 1 / # / @ / ！ / ¥ / $`
- [x] Task-3: 完成现实检查与文档收口
  - DoD: DOM 度量、console 摘要、evidence 和 docs 同步完成

## Evidence Plan (UI / E2E)
- Evidence required: partial
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260411-site-font-unify-symbol-density/evidence/
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: 首页无 console error
  - disabled: reduced-motion 路径仍保留统一字体与结构
  - success: single-font-lxgw、symbol-particles-dense、output-width-80vw-retained

## Observability / Debug Plan
- Logs: 浏览器 console、font-family、输出区宽度比例、粒子字符计数
- Error codes: 不新增
- Trace/metrics (optional): N/A
- Debug flags (optional): `prefers-reduced-motion`

## Risks & Rollback
- Risks:
  - 单字体后终端感削弱
  - 粒子变多后页面发脏
- Rollback plan:
  - 若结果不佳，优先回退 `styles.css` 的字体统一和 prompt 权重，再回退 `script.js` 的字符池与数量

## Sequential Phases
- phase_execution: N/A
- phase_confirmation_policy: N/A
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 将全站文字统一到 `LXGW`
  2. 扩大粒子字符池中目标符号的占比，并提高数量/字号
  3. 用 DOM 度量和 docs gate 验证后收口

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: no
- Approved: no-op（trivial 默认直行）
