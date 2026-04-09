# Task-ID: TID-20260407-microbreak-break-labels

## Test Strategy
- Unit: 不新增独立单测；现有 `npm test` 作为回归网，确保 locale 与既有逻辑改动未引入基础回归。
- Integration: 通过 `npm --prefix apps/desktop run typecheck` 与 `npm --prefix apps/desktop run build` 验证桌面前端仍可完整编译。
- E2E (if applicable): 通过浏览器 preview 的可访问性快照和控制台摘要，证明主设置页术语已切换为“微休息 / 休息”。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 检查 `apps/desktop/src/locales/{zh-CN,en}.json` diff，并用浏览器 preview 快照验证设置页上确实显示“微休息 / 休息”；tray/runtime 通过 locale 文件变更自证。
- AC2 -> 检查 `app/locales/{zh-CN,en}.json` diff，确认 legacy app 里不再保留 `小憩` / `Mini break` / `Long break` 的当前可见术语。
- AC3 -> 检查 `README.md`、`docs/CHANGELOG.md`、`docs/UI.md`、`docs/SettingsInventory.md`、`docs/CodeMap.md`、`net.hovancik.Pauza.metainfo.xml` diff，确认当前说明已对齐并注明兼容内部标识。
- AC4 -> `npm test`、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`、`python3 scripts/validate_agent_configs.py`、`python3 scripts/validate_workflow_docs.py --mode manual` 与 evidence README。

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: `settings-terminology-snapshot.md` 展示主设置页中“微休息 / 休息”两组标题与下方“提前提醒 / 延后”标签。
- Fallback / secondary flow -> tests/evidence: `apps/desktop/src/locales/{zh-CN,en}.json` 中 tray `skipLongBreak`、runtime `long.title` / `kind.long` 已同步改成 `休息 / Break`。
- Visible states / transitions -> tests/evidence: 快照覆盖开启态的微休息 / 休息开关、预设分组标签和后续分区标签；控制台摘要证明预览无新增显著报错。

## Governance Gates
- Agent Config Validation: `python3 scripts/validate_agent_configs.py`
- Workflow Docs Validation: `python3 scripts/validate_workflow_docs.py --mode manual`
- Approval Escalation Owner: orchestrator

## False-pass Cases
- `DONE` 任务对应的 spec 仍保留 `TBD/INIT` 占位。
- `orchestrator` 未显式使用 `sandbox_mode = "danger-full-access"` 与 `approval_policy = "never"`，却仍宣称当前仓库运行在 aggressive 基线。
- 代码变更前未记录 `Source Basis`，导致实现依据不可追溯。
- 子 agent 未显式 `approval_policy = "never"`。
- `moderate/complex` 任务通过缩小 `Required Roles` 伪装为 trivial fallback。
- 已使用 `single-agent-fallback`，但 logs/plans 没有单独记录 `Execution Mode` / `Fallback Scope` / `Fallback Reason Code`。
- 需要受角色边界约束的文件系统写命令没有经过 `run_role_guard.py`，只在结案时补跑范围校验。
- `git add -- <explicit paths...>` 仍被包进 wrapper / helper script，导致运行时看不到裸命令前缀。
- `interaction_impact != none`，但 plan/testplan/ui spec 没有定义 primary flow / fallback flow / visible states / evidence coverage。

## Evidence Capture (UI / E2E)
- Required: yes   <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260407-microbreak-break-labels/evidence/
- What to capture:
  - Screenshots: 以可访问性快照形式保留设置页主内容，显示“微休息 / 休息”。
  - Video/trace (optional): N/A
  - HAR/console logs (optional): 浏览器控制台摘要一份，确认仅有 React DevTools 提示。

## Quality Gates (Non-functional)
- a11y: 标签改名后，屏幕阅读器依然能从开关与 group label 中读出清晰含义。
- perf budget: 仅改文案与文档，不新增依赖或运行时成本。
- error handling / observability: 控制台无新增显著错误；构建与测试通过。
- security / privacy: 无新网络请求、无新权限、无敏感数据处理。

## Boundary / Invalid Input Cases
- 内部 `miniBreak*` / `longBreak*` 标识必须保留，不能因为 UI 文案统一而误改存储 key。
- 历史 spec / logs / evidence 属于审计切片，不作为本轮批量替换对象。
- 中文“微休息”比旧“长休息 / 小憩”字长不同，设置页布局不能因此错位。

## Concurrency / Race Cases (if applicable)
- 不适用；本轮仅替换 copy 与文档。

## Mocks & Test Data
- 使用浏览器 preview 的默认设置快照，无需额外 mock。

## Commands to Run
- `npm test`
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_agent_configs.py`
- `python3 scripts/validate_workflow_docs.py --mode manual`
- 浏览器 preview snapshot / console capture

## Expected Results
- PASS criteria:
  - 测试、typecheck、build 和两项 workflow gate 通过。
  - 浏览器 preview 快照中能直接看到“微休息 / 休息”。
  - 控制台无新增显著错误。
- Outputs to keep (10~20 lines snippet):
  - `npm test` 的 `18 passed / 319 passed` 摘要。
  - `vite build` 的 `1820 modules transformed` 摘要。
  - 控制台单条 React DevTools info。
