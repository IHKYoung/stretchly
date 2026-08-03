# Task-ID: TID-20260803-current-worktree-handover

## Summary
- Title: 收口当前桌面端与官网工作树
- Date: 2026-08-03
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 审阅父仓库当前全部有效修改，补齐可追溯文档和验证证据，并创建详细 commit。
- In-scope: Rust 智能等待 snapshot/tray、React 节奏输入与文字层级、三语言消息及生成 registry、相关测试/docs、官网 latest 下载父仓库快照、`.gitignore`、`scratch/README.md`、workflow handover 与 commit audit。
- Out-of-scope: 新增休息提示语、`scratch/locale-translations/**`、依赖安装、远端 push/release、历史改写、嵌套 `apps/site` 独立仓库的四个既有残留修改。
- Assumptions: 当前 tracked diff 和任务级未跟踪文档均为用户希望保留的成果；site 生产验证记录真实来自 2026-07-24 已完成任务，本轮只重跑本地可复现门禁。
- Risks: snapshot 字段与 runtime 状态漂移；tray 将等待态误当计划倒计时；自定义输入交互回弹；主语言 key/registry 漂移；多 Task-ID commit hook 归因错误；临时翻译产物误入版本库。
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: smart reminder 到点后进入等待空档，tray 显示后端最长等待倒计时；设置节奏页用完整句式编辑 preset/custom；官网按钮透明解析 latest DMG。
- Fallback / secondary flow: blocker 时不暴露等待倒计时；forced 到点直开；自定义输入空值恢复当前值；GitHub API 失败或无匹配资产时保留 pinned URL。
- User-visible boundary: macOS tray title/菜单刷新、桌面设置窗口、官网右上角下载按钮。
- Key visible states / transitions: scheduled -> due/waiting -> break；preset -> custom editing -> commit/restore；pinned href -> latest href 或 pinned fallback。

## Goal
- 让未来维护者能够从一条父仓库主提交和对应审计提交中理解当前行为、事实源、验证结果及回滚路径。

## Scope
- In-scope: 当前 `git status --short` 中除被 `.gitignore` 排除的本地翻译素材外的父仓库成果，以及完成 handover 所需的根 docs。
- Out-of-scope: 任何新产品功能、提示语内容扩充、网络部署和嵌套仓库历史收口。

## Source Basis (Read Before Code)
- Related code/files reviewed: `apps/desktop/src-tauri/src/{state.rs,state/tests.rs,shell.rs}`、`apps/desktop/src/{App.tsx,styles.css}`、三份主语言消息及 `registry.generated.json`、设置/翻译测试、`apps/site/{script.js,download/targets.js}`、`.gitignore` 与 `scratch/README.md`。
- Related docs/specs/logs reviewed: `AGENTS.md`、`docs/{RepositoryGuidelines,Architecture,UI,CHANGELOG}.md`、`workflow/{task-lifecycle,gates}.md`、`TID-20260727-settings-simplification-015`、`apps/site/docs/specs/TID-20260724-site-latest-download-route/**`、handover-commit skill、Git hooks 与 commit audit scripts。
- Why these are sufficient: 覆盖等待状态事实源与派生接口、tray 观察者、设置展示者、多语言生成链、官网下载契约、测试面、暂存归因和 post-commit 审计闭环。

## Acceptance Criteria (AC)
- AC1: `next_break_wait_remaining_ms` 只由 Rust runtime 在 due + smart waiting + no blocker 时派生，tray 在等待期间显示该倒计时且不受常规倒计时开关影响。
- AC2: 节奏详情能区分 preset 与“自定义”入口，完整休息按提醒周期和折算分钟解释；主语言文案、生成 registry 与排版层级一致。
- AC3: 官网成功路径使用 `releases/latest/download/<encoded asset>`，失败路径保持 pinned；父仓库纳入已完成 site task 的 spec/evidence/audit 快照。
- AC4: 临时翻译素材继续被忽略；全部相关测试、生成器、typecheck、Rust tests、frontend build、语法检查与 workflow gates 通过，或缺口显式记录。
- AC5: 主提交使用两个实际 staged Task-ID 的精确多任务元数据；post-commit 审计通过独立 audit-only commit 闭环。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: locked
- Primary flow: backend wait budget -> snapshot -> tray countdown；preset/custom control -> autosave；GitHub API asset name -> latest download href。
- Fallback / secondary flow: blocker/forced 不伪造等待；空 custom draft 回退原值；site API reject/empty 保持 pinned。
- Interaction authority / ownership boundary: Rust runtime 持有调度事实；Tauri shell 观察并展示；React 只消费 snapshot 与设置值；GitHub Release API 持有 latest asset 事实。
- Visible entrypoints / handoff cues: tray title、设置页节奏卡的 preset/custom 输入、官网 `.download-button`。
- In-scope interactions: 等待倒计时可见性、节奏字段编辑与说明、文字层级、下载 href 解析。
- Out-of-scope interactions: reminder 算法阈值、autosave 协议、break CTA、官网样式、发布流程。
- Interaction acceptance criteria: 不合成后端等待进度；不承诺 smart skip 立即开始；设置可读且可恢复；下载按钮始终有可点击 fallback。

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: 当前会话使用受管 `workspace-write` 权限；只执行仓库内非破坏性检查、文档和 commit，不宣称模板中的 unrestricted baseline。
- Required Roles: orchestrator,architect,coder,tester,scribe,reality_checker
- Execution Mode Policy: 会话规则仅在用户明确要求 delegation 时允许子 agent；本任务采用范围明确的 single-agent fallback。
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`；本任务未启动子 agent。
- Escalation Route: subagent -> Orchestrator -> direct local execution | user（仅破坏性、外部成本、发布或历史改写）。
- Safe-local Command Route: 主 agent 直接执行裸 repo-local 命令，并使用显式路径暂存。
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: none（本轮只修改本地父仓库并 commit）。
- touches_running_service: no。
- backup_required: no。
- backup_plan: 提交前保留 `git status`、完整 diff、验证输出与 staged stat。
- rollback_plan: 后续使用普通 `git revert <commit>`；无 schema/data migration。
- destructive_operations: none。
- operator_approval_required: no（用户已明确要求详细 commit；push/release 不在范围）。
- rationale: 本地非破坏性 handover/commit 是用户直接授权的标准收口操作。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 审查并建立系统/交互边界
  - DoD: 当前 diff、相邻 runtime/locale/site 契约、用户修改与临时素材边界已核对。
- [x] Task-2: 补齐文档并完成验证
  - DoD: spec/log/plan/changelog/evidence 完整，测试与门禁有当前周期结果。
- [x] Task-3: 精确暂存、主提交与审计闭环
  - DoD: 多 Task-ID message 通过 hook，audit-only commit 完成，父仓库无残留任务改动。

## Evidence Plan (UI / E2E)
- Evidence required: partial  <!-- yes | no | partial -->
- Owner: orchestrator
- Artifact path: docs/specs/TID-20260803-current-worktree-handover/evidence/
- Interaction validation note: 主要交互由源码回归、Rust/Vitest 和 production build 覆盖；本地 Vite ready，但 browser backend 列表为空，未取得截图，此缺口已在 evidence 显式记录。
- Required states to capture:
  - loading: site 请求期间 pinned href 保持可用；桌面 snapshot 沿既有 loading path。
  - empty: custom draft 为空恢复当前值；site 无匹配 asset 回退 pinned。
  - error: site fetch/response 错误显性 warning；桌面 load/save error 保持既有展示。
  - disabled: 常规 tray 倒计时关闭时，智能等待临时反馈仍可见；blocked/forced path 不伪造等待。
  - success: waiting countdown、preset/custom、三语语义和 latest href 合约通过自动化验证。

## Observability / Debug Plan
- Logs: Vitest、Cargo、TypeScript/Vite、locale generator、site syntax、workflow validator 与 Git hook 输出。
- Error codes: 保留命令退出码；site API failure 走 console warning + pinned fallback，不合成业务错误。
- Trace/metrics (optional): 前端测试数、Rust 测试数、50 locale bundles、Vite build summary、staged Task-ID 集合。
- Debug flags (optional): 浏览器 preview `?window=settings` / `?window=break`；本轮不改变 debug flags。

## Risks & Rollback
- Risks: 既有 UI 修改缺少真实机截图；嵌套 site repo 有独立残留；commit hook 会在主提交后生成未暂存审计文件。
- Rollback plan: 主提交和 audit-only 提交分别普通 revert；site 独立仓库残留保持不动并在交接中列出。

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 审查当前 diff、系统状态边界、嵌套仓库与 workflow/hook 规则。
  2. 补齐根 handover task package、CHANGELOG 和 evidence。
  3. 运行生成器、tests、typecheck/build、site syntax、diff/workflow checks。
  4. 使用显式路径暂存，校验 staged Task-ID 后创建详细主提交。
  5. 检查 post-commit audit，按 hook 规则创建 audit-only follow-up 并报告残留。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: 用户于 2026-08-03 明确要求“先把已有的修改，做一个详细的commit”。
