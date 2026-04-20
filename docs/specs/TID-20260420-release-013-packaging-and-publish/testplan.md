# Task-ID: TID-20260420-release-013-packaging-and-publish

## Test Strategy
- Unit:
  - `npm test`
  - `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- Integration:
  - `python3 scripts/sync_desktop_locales.py`
  - `npm run typecheck`
  - `npm --prefix apps/desktop run build`
  - `python3 scripts/validate_workflow_docs.py --mode manual`
  - `python3 scripts/validate_agent_configs.py`
- E2E (if applicable):
  - `npm --prefix apps/desktop run tauri build -- --bundles dmg --no-sign`
  - `python3 apps/site/scripts/publish_site_release.py --asset <dmg>`
  - `gh release view v0.1.3 --repo IHKYoung/Pauza`

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `rg -n '0.1.3' README.md package.json package-lock.json apps/desktop/package.json apps/desktop/package-lock.json apps/desktop/src-tauri/Cargo.toml apps/desktop/src-tauri/tauri.conf.json apps/site/README.md apps/site/docs/ReleasePlaybook.md docs/CHANGELOG.md`
- AC2 -> `python3 scripts/sync_desktop_locales.py` + `npm test`
- AC3 -> `npm --prefix apps/desktop run tauri build -- --bundles dmg --no-sign`
- AC4 -> `python3 apps/site/scripts/publish_site_release.py --asset <dmg>` + `gh release view v0.1.3 --repo IHKYoung/Pauza`
- AC5 -> `git status --short` + `git log --oneline -n 3` + `git push <remote> <branch>`

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
- 已使用 `single-agent-fallback`，但 logs/plans 没有单独记录 `Execution Mode` / `Fallback Scope` / `Fallback Reason Code`。
- 需要受角色边界约束的文件系统写命令没有经过 `run_role_guard.py`，只在结案时补跑范围校验。
- `git add -- <explicit paths...>` 仍被包进 wrapper / helper script，导致运行时看不到裸命令前缀。
- `interaction_impact != none`，但 plan/testplan/ui spec 没有定义 primary flow / fallback flow / visible states / evidence coverage。

## Evidence Capture (UI / E2E)
- Required: no   <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260420-release-013-packaging-and-publish/evidence/
- What to capture:
  - Screenshots:
  - Video/trace (optional):
  - HAR/console logs (optional):

## Quality Gates (Non-functional)
- a11y:
  - N/A（本轮不新增 UI 交互）
- perf budget:
  - build / tauri build 不新增已知阻断
- error handling / observability:
  - build、release、validator 与 git 输出进入日志
- security / privacy:
  - 不新增依赖或密钥；不做历史重写

## Boundary / Invalid Input Cases
- 根目录翻译 scratch 文件仍存在时，不应被纳入正式提交
- release 资产命名若不是 `Pauza_<version>_aarch64.dmg`，发布脚本必须失败
- 若 GitHub latest 与 pinned 资产不一致，首页 pinned URL 仍应指向本次 release 版本

## Concurrency / Race Cases (if applicable)
- N/A；本轮只做发布链验证，不新增并发运行时代码

## Mocks & Test Data
- 使用真实本地产出的 `Pauza_0.1.3_aarch64.dmg`
- 使用真实 GitHub 仓库 `IHKYoung/Pauza`

## Commands to Run
- `npm test`
- `npm run typecheck`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm --prefix apps/desktop run build`
- `npm --prefix apps/desktop run tauri build -- --bundles dmg --no-sign`
- `python3 scripts/sync_desktop_locales.py`
- `python3 scripts/validate_agent_configs.py`
- `python3 scripts/validate_workflow_docs.py --mode manual`
- `python3 apps/site/scripts/publish_site_release.py --asset <dmg>`
- `gh release view v0.1.3 --repo IHKYoung/Pauza`

## Expected Results
- PASS criteria:
  - 版本真源统一为 `0.1.3`
  - locale registry 重建成功
  - 单元/构建/validator 通过
  - 产出 `Pauza_0.1.3_aarch64.dmg`
  - GitHub release 与代码 commit/push 完成
- Outputs to keep (10~20 lines snippet):
  - locale sync PASS
  - 测试与构建 PASS 摘要
  - `gh release view v0.1.3 --repo IHKYoung/Pauza`
  - `git status --short` 结案摘要
