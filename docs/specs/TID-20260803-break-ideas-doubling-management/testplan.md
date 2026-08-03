# Task-ID: TID-20260803-break-ideas-doubling-management

## Test Strategy
- Unit: Vitest 检查三语 count/key/shape、manifest 完整性、类别交错、重复与长度预算；保留 runtime selection/rotation tests。
- Integration: 运行 sync generator 两次确认校验和幂等，再运行全量 Vitest、desktop typecheck 与 production build。
- E2E (if applicable): 使用现有 break preview 尝试检查三语最长完整休息换行；若 browser backend 不可用，保留自动化与构建证据并记录缺口。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> content governance test + generator official key parity/count/ID sequence。
- AC2 -> batch manifest schema/entry/target tests；断言 manifest 不含正文。
- AC3 -> generator validation + invalid fixture/纯函数 tests（如实现可导入）+ targeted process invocation。
- AC4 -> duplicate/length/category stats；中英繁人工抽样与禁用模板短语扫描。
- AC5 -> generator idempotence、full Vitest、typecheck、build、diff/workflow checks 与 evidence。

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: `desktopBreakIdeas.js` 覆盖 micro 稳定与 long 完整轮换；content test 覆盖新增池。
- Fallback / secondary flow -> tests/evidence: 保留 default prompt 与 locale fallback 现有测试；本任务不改实现。
- Visible states / transitions -> tests/evidence: 断言完整正文打完后才进入 60 秒 hold；最长内容构建/预览证据。

## Governance Gates
- Agent Config Validation: `python3 scripts/validate_agent_configs.py`
- Workflow Docs Validation: `python3 scripts/validate_workflow_docs.py --mode manual`
- Approval Escalation Owner: orchestrator

## False-pass Cases
- 已结案任务对应的 spec 仍保留未填写占位。
- `orchestrator` 未显式使用 `sandbox_mode = "danger-full-access"` 与 `approval_policy = "never"`，却仍宣称当前仓库运行在 aggressive 基线。
- 代码变更前未记录 `Source Basis`，导致实现依据不可追溯。
- 子 agent 未显式 `approval_policy = "never"`。
- `moderate/complex` 任务通过缩小 `Required Roles` 伪装为 trivial fallback。
- 已使用 `single-agent-fallback`，但 logs/plans 没有单独记录 `Execution Mode` / `Fallback Scope` / `Fallback Reason Code`。
- 需要受角色边界约束的文件系统写命令没有经过 `run_role_guard.py`，只在结案时补跑范围校验。
- `git add -- <explicit paths...>` 仍被包进 wrapper / helper script，导致运行时看不到裸命令前缀。
- `interaction_impact != none`，但 plan/testplan/ui spec 没有定义 primary flow / fallback flow / visible states / evidence coverage。

## Evidence Capture (UI / E2E)
- Required: partial   <!-- yes | no | partial -->
- Owner: orchestrator
- Artifacts path: docs/specs/TID-20260803-break-ideas-doubling-management/evidence/
- What to capture:
  - Screenshots: 三语最长完整休息或明确的 browser backend blocker。
  - Video/trace (optional): N/A。
  - HAR/console logs (optional): preview console；静态内容无网络 HAR 要求。

## Quality Gates (Non-functional)
- a11y: 无新增控件；现有 keyboard/focus 相关测试不得回归。
- perf budget: 记录 registry 与 build 体积变化；不设虚假硬阈值，但异常倍增必须解释。
- error handling / observability: invalid locale/kind/id/manifest 以 generator 非零退出并给出定位。
- security / privacy: 内容与构建过程不接触用户数据或外部服务。

## Boundary / Invalid Input Cases
- official locale 缺一个 ID、额外 ID、空 `text`、micro 多 `title`、long 少 `title`、归一化重复。
- manifest 重 ID、断号、错误首尾/数量、未知类别、缺 locale、包含正文、类别连续超过上限。
- 字段短于/长于 locale budget；legacy locale 不受 official parity 误伤。

## Concurrency / Race Cases (if applicable)
- N/A：生成是单进程静态转换；验证全部通过后才写 registry。

## Mocks & Test Data
- 直接读取 repo source/manifest；无网络 mock。必要的 invalid case 使用临时目录，不污染真实 bundle。

## Commands to Run
- `python3 scripts/sync_desktop_break_ideas.py`
- `npm test -- --run test/desktopBreakIdeasContent.js test/desktopBreakIdeas.js test/desktopBreakCopySource.js`
- `npm test`
- `npm run typecheck`
- `npm --prefix apps/desktop run build`
- `git diff --check`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria: 所有自动门禁通过，统计符合 AC，registry 第二次生成无 diff；任何无法采集的 UI 证据单独标明而不伪称已验证。
- Outputs to keep (10~20 lines snippet): generator summary、content distribution/length report、tests/typecheck/build 摘要、workflow validator 和 git diff check。

## Actual Results

- PASS：generator 报告 `50 languages / 50 bundles / 1 managed batch`；连续两次生成 SHA-256 均为 `474f7ef9c8dd7aa01b32e16a4062182e26b94bbbc85b66a5d1313388542cb8ec`。
- PASS：代表性 negative-path 检查确认 official 缺 ID、归一化重复、manifest 夹带正文、禁用短语都会显性失败。
- PASS：全量 Vitest `8 files / 132 tests`；新增 content governance suite 为 `5 tests`。
- PASS：TypeScript typecheck；Vite production build `1827 modules`，JS `4,561.25 kB / gzip 1,427.56 kB`，保留既有大 chunk warning。
- PASS：changed-file Standard、Python compile、`git diff --check`、workflow docs validator。
- SKIP：agent config validator 检测到仓库无 multi-agent config。
- BLOCKED（partial evidence）：Vite preview ready，但 Browser runtime 列表为空，无法采集最长文案截图；没有宣称视觉检查通过。
