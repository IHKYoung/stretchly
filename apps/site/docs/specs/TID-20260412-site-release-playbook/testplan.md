# Task-ID: TID-20260412-site-release-playbook

## Test Strategy
- Unit: 不单独拆单元测试；脚本体量较小，优先验证命令行主路径与幂等路径。
- Integration: 使用当前真实 `Pauza_0.1.2_aarch64.dmg` 运行脚本，验证站点文件替换、release 命令生成与幂等行为。
- E2E (if applicable): 不涉及浏览器 E2E。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `python3 scripts/publish_site_release.py --asset <current dmg> --skip-release`
- AC2 -> `python3 scripts/publish_site_release.py --asset <current dmg> --dry-run`
- AC3 -> 人工检查 `README.md`、`docs/RepositoryGuidelines.md`、`docs/ReleasePlaybook.md`
- AC4 -> `git status --short` 不再出现 `scripts/__pycache__` / `.playwright-mcp`

## Interaction Contract Coverage
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: N/A
- Fallback / secondary flow -> tests/evidence: N/A
- Visible states / transitions -> tests/evidence: N/A
- Validator expectation: 当 `interaction_impact != none` 时，本节三项与 Evidence Capture 的 `Required` 不得继续保留 `N/A/no/TBD`

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
- 高风险任务在 `Execution Safety Block` 已标记风险的情况下，仍把 `Approval needed` 写成 `no`。
- 已使用 `single-agent-fallback`，但 logs/plans 没有单独记录 `Execution Mode` / `Fallback Scope` / `Fallback Reason Code`。
- 需要受角色边界约束的文件系统写命令没有经过 `run_role_guard.py`，只在结案时补跑范围校验。
- `git add -- <explicit paths...>` 仍被包进 wrapper / helper script，导致运行时看不到裸命令前缀。
- `interaction_impact != none`，但 plan/testplan/ui spec 没有定义 primary flow / fallback flow / visible states / evidence coverage。

## Evidence Capture (UI / E2E)
- Required: no   <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260412-site-release-playbook/evidence/
- What to capture:
  - Screenshots: N/A
  - Video/trace (optional): N/A
  - HAR/console logs (optional): 保留脚本 dry-run 输出与 validator 输出摘要

## Quality Gates (Non-functional)
- a11y: 不涉及
- perf budget: 单次命令只做少量文本替换与 1~2 个 `gh` 调用
- error handling / observability: 失败时给出明确 stderr / 退出码；dry-run 可复盘
- security / privacy: 不在脚本内硬编码 token；依赖 `gh auth login`

## Boundary / Invalid Input Cases
- asset 文件不存在
- asset 文件名不匹配 `Pauza_<version>_aarch64.dmg`
- `download/targets.js` 或 `index.html` 未命中预期正则
- `gh auth status` 失败

## Concurrency / Race Cases (if applicable)
- 不涉及并发；脚本按固定顺序串行执行
- 已有 release 的重复执行通过 `upload --clobber` 保证幂等覆盖

## Mocks & Test Data
- 使用真实本地 asset：`/Users/changkunyang/CKProjects/Pauza/apps/desktop/src-tauri/target/release/bundle/dmg/Pauza_0.1.2_aarch64.dmg`

## Commands to Run
- `python3 scripts/publish_site_release.py --asset /Users/changkunyang/CKProjects/Pauza/apps/desktop/src-tauri/target/release/bundle/dmg/Pauza_0.1.2_aarch64.dmg --skip-release`
- `python3 scripts/publish_site_release.py --asset /Users/changkunyang/CKProjects/Pauza/apps/desktop/src-tauri/target/release/bundle/dmg/Pauza_0.1.2_aarch64.dmg --dry-run`
- `python3 scripts/validate_agent_configs.py`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria: 脚本能正确推导 `v0.1.2` 与 release URL；`--skip-release` 路径成功；`--dry-run` 打印出 `gh release view` 与 `gh release upload/create` 命令；workflow validator 通过
- Outputs to keep (10~20 lines snippet): 脚本输出中的 asset/version/tag/url 摘要、`[OK] Site pinned release URLs are already up to date.`、validator PASS 摘要
