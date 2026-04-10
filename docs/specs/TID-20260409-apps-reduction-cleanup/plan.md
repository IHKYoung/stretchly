# Task-ID: TID-20260409-apps-reduction-cleanup

## Summary
- Title: 清理 apps 未使用迁移层并保留 app 参考目录
- Date: 2026-04-09
- Level: complex
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 继续对仓库做减法，移除不再参与当前桌面端开发链路的迁移层、旧测试、旧样例目录、本地生成目录、README 展示素材以及根级社区/发布元数据文件；保留 `apps/desktop` 作为唯一有效实现，保留 `app/` 作为原版参考代码目录，并把未来可能还要用的信息沉淀到 `docs/`。
- In-scope:
  - 删除 `apps/desktop/legacy-utils/**`
  - 删除依赖上述目录的根级旧测试
  - 删除 `coverage/`、`examples/`、`output/` 和根目录 README 展示素材
  - 抽取并归档根级 `README.md`、`CONTRIBUTING.md`、`CODE_OF_CONDUCT.md`、`LICENSE`、`net.hovancik.Pauza.desktop`、`net.hovancik.Pauza.metainfo.xml` 的有用信息，然后删除这些根文件
  - 同步更新当前 docs 与本任务记录
- Out-of-scope:
  - 删除 `app/**`
  - 重写 Tauri host 或前台运行逻辑
  - 变更当前打包/图标真源
- Assumptions:
  - `legacy-utils` 不再被当前运行时使用
  - `coverage/`、`output/` 都是可再生本地目录
  - 用户允许清理旧 README 展示素材，后续 README 会按新结构重写
- Risks:
  - 仍有脚本或 docs 残留已删除引用
  - 删除 tracked 素材后 README/贡献说明出现陈旧描述
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 收口当前仓库，只保留当前桌面端有效实现与原版参考代码，尽量剥离会干扰主体开发判断的历史过渡层和展示资产。

## Scope
- In-scope:
  - `apps/desktop/legacy-utils/**`
  - `test/**` 中依赖 legacy-utils 的旧测试
  - 根 `README.md`
  - `coverage/`、`examples/`、`output/`
  - 根目录旧 PNG 展示素材
  - `docs/{RepositoryGuidelines,CodeMap,CHANGELOG}.md`
  - 本任务 specs / 当日 plans / logs
- Out-of-scope:
  - `app/**`
  - `graphics/**`
  - `build/**`
  - `apps/desktop/src/**` 与 `apps/desktop/src-tauri/**` 的功能性逻辑

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `package.json`
  - `vitest.config.ts`
  - `apps/desktop/src/**`
  - `apps/desktop/src-tauri/**`
  - `README.md`
  - `CONTRIBUTING.md`
  - `CODE_OF_CONDUCT.md`
  - `LICENSE`
  - `net.hovancik.Pauza.desktop`
  - `net.hovancik.Pauza.metainfo.xml`
  - `graphics/generate_icon_assets.py`
- Related docs/specs/logs reviewed:
  - `docs/{RepositoryGuidelines,CodeMap,Architecture,CHANGELOG}.md`
  - `docs/specs/TID-20260409-apps-locale-single-source-refactor/*`
  - `docs/plans/2026-04-09.md`
  - `docs/logs/2026-04-09.md`
- Why these are sufficient:
  - 已覆盖当前默认运行/构建/测试链路、旧迁移层的唯一剩余消费方、README 素材引用链与图标生成链，可以判断哪些目录和素材还能影响主体开发，哪些只是噪音。

## Acceptance Criteria (AC)
- AC1: `apps/desktop/legacy-utils/**` 与依赖它的根级旧测试全部移除，默认测试链路只剩当前 desktop 真源相关测试。
- AC2: `coverage/`、`examples/`、`output/` 与根目录旧 README 展示素材全部移除。
- AC3: 根级 `README.md`、`CONTRIBUTING.md`、`CODE_OF_CONDUCT.md`、`LICENSE`、`net.hovancik.Pauza.desktop`、`net.hovancik.Pauza.metainfo.xml` 已移除，且其有用信息已归档到 `docs/RootMetadataArchive.md`。
- AC4: `npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`、`npm test`、`python3 scripts/validate_workflow_docs.py --mode manual` 通过。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: N/A
- Primary flow: N/A
- Fallback / secondary flow: N/A
- Interaction authority / ownership boundary: N/A
- Visible entrypoints / handoff cues: N/A
- In-scope interactions: N/A
- Out-of-scope interactions: N/A
- Interaction acceptance criteria: N/A
- Validator expectation: 当 `interaction_impact != none` 时，本节与 Requirement Brief 中的交互字段不得继续保留 `N/A/TBD`

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,architect,coder,tester,scribe,reality_checker
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限仓库结构、根 README、测试入口与开发索引文档
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 Git diff、`rg` 审计、typecheck/build/test 与 workflow docs gate
- rollback_plan: 从 VCS 恢复被删除目录、素材、README 与 docs，然后重跑验证命令
- destructive_operations: 删除非运行时目录、旧测试和 tracked 素材
- operator_approval_required: no
- rationale: 用户已明确要求“做减法”，并指定 `app/` 仅保留为参考目录；本轮不涉及线上服务、历史重写、提权或外部副作用。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 删除无运行价值的迁移层和旧测试
  - DoD: `apps/desktop/legacy-utils/**` 与根级旧测试已从仓库中移除，当前运行时源码未被误删。
- [x] Task-2: 删除非主体开发目录与 README 展示素材
  - DoD: `coverage/`、`examples/`、`output/` 与根目录旧展示 PNG 已移除，不再有当前默认链路依赖这些素材。
- [x] Task-3: 归档并清理根级社区/发布元数据
  - DoD: `docs/RootMetadataArchive.md` 已提炼保留有用信息，根级 `README.md`、`CONTRIBUTING.md`、`CODE_OF_CONDUCT.md`、`LICENSE`、`net.hovancik.Pauza.desktop`、`net.hovancik.Pauza.metainfo.xml` 已移除。
- [x] Task-4: 同步当前说明文档与任务记录
  - DoD: `docs/{RepositoryGuidelines,CodeMap,CHANGELOG,RootMetadataArchive}.md` 与本任务 specs/plans/logs 已无占位信息且与当前目录一致。
- [x] Task-5: 执行最终验证并结案
  - DoD: typecheck/build/test/docs gate 全部通过，spec 与当日 plans/logs 进入 DONE。

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260409-apps-reduction-cleanup/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: N/A
  - disabled: N/A
  - success: N/A

## Observability / Debug Plan
- Logs:
  - `git status --short`
  - `rg` 审计 legacy-utils / examples / deleted screenshot 引用以及根级元数据文件是否清零
  - `npm test`、typecheck/build、docs validator 输出
- Error codes: N/A
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 误删当前仍被文档或脚本引用的 tracked 文件
  - `legacy-utils` 删除后仍残留测试或脚本引用，导致 `npm test` 失败
  - 删除根级法律/发布元数据后，后续如需恢复发布会缺少快速参考
- Rollback plan:
  - 从 VCS 恢复对应目录、素材与 README/docs
  - 重新执行 `npm test`、`npm --prefix apps/desktop run build` 与 docs validator 确认回滚完成

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 审计 `legacy-utils`、旧测试、`coverage/`、`examples/`、`output/` 与 README 素材引用链。
  2. 删除确认无运行价值的目录、旧测试和展示素材。
  3. 抽取根级社区治理、法律与 Linux 发布元数据到 `docs/RootMetadataArchive.md`，删除对应根文件。
  4. 同步当前开发文档与 cleanup task 记录。
  5. 运行 typecheck/build/test/docs gate，按结果收口结案。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: 用户已在 2026-04-09 明确要求“现在做减法，用不到的都可以移除”，并进一步点名清理 `CODE_OF_CONDUCT.md`、`CONTRIBUTING.md`、`LICENSE`、`net.hovancik.Pauza.desktop`、`net.hovancik.Pauza.metainfo.xml` 与 `README.md`。
