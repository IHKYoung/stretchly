# Task-ID: TID-20260423-release-014-macos-package-commit

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
  - `rg -n '0.1.4' README.md package.json package-lock.json apps/desktop/package.json apps/desktop/package-lock.json apps/desktop/src-tauri/Cargo.toml apps/desktop/src-tauri/Cargo.lock apps/desktop/src-tauri/tauri.conf.json docs/CHANGELOG.md`
- E2E (if applicable):
  - `npm --prefix apps/desktop run tauri build -- --bundles app,dmg --no-sign`
  - `git status --short`
  - `git log --oneline -n 3`

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `rg -n '0.1.4' README.md package.json package-lock.json apps/desktop/package.json apps/desktop/package-lock.json apps/desktop/src-tauri/Cargo.toml apps/desktop/src-tauri/Cargo.lock apps/desktop/src-tauri/tauri.conf.json docs/CHANGELOG.md`
- AC2 -> `python3 scripts/sync_desktop_locales.py` + `npm test` + `npm run typecheck` + `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` + `npm --prefix apps/desktop run build` + `python3 scripts/validate_workflow_docs.py --mode manual`
- AC3 -> `npm --prefix apps/desktop run tauri build -- --bundles app,dmg --no-sign` + `ls apps/desktop/src-tauri/target/release/bundle/{macos,dmg}`
- AC4 -> `git status --short` + `git log --oneline -n 3` + `docs/commits/2026-04-23.md`

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
- Artifacts path: docs/specs/TID-20260423-release-014-macos-package-commit/evidence/
- What to capture:
  - Screenshots:
  - Video/trace (optional):
  - HAR/console logs (optional): 保留打包命令与产物路径摘录即可

## Quality Gates (Non-functional)
- a11y:
  - N/A（本轮不新增 UI 交互）
- perf budget:
  - 本地 build / tauri build 不新增已知阻断
- error handling / observability:
  - build、validator 与 git 输出进入日志
- security / privacy:
  - 不新增依赖、密钥或外部网络发布动作

## Boundary / Invalid Input Cases
- 若某个版本真源仍停留在 `0.1.3`，本轮不能进入 DONE
- 若 `tauri build` 未生成 `Pauza_0.1.4_aarch64.dmg`，视为打包失败
- `post-commit` 自动生成的 `docs/commits/2026-04-23.md` 若未通过 audit-only follow-up commit 落盘，工作区不算完成收尾

## Concurrency / Race Cases (if applicable)
- N/A；本轮不新增运行时并发逻辑

## Mocks & Test Data
- 使用真实当前工作树、真实 arm64 macOS 构建环境与真实本地产物 `Pauza_0.1.4_aarch64.dmg`

## Commands to Run
- `python3 scripts/sync_desktop_locales.py`
- `npm test`
- `npm run typecheck`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm --prefix apps/desktop run build`
- `npm --prefix apps/desktop run tauri build -- --bundles app,dmg --no-sign`
- `python3 scripts/validate_agent_configs.py`
- `python3 scripts/validate_workflow_docs.py --mode manual`
- `git status --short`
- `git log --oneline -n 3`

## Expected Results
- PASS criteria:
  - 版本真源统一为 `0.1.4`
  - locale registry 重建成功
  - 单元 / 构建 / validator 通过
  - 产出 `Pauza.app` 与 `Pauza_0.1.4_aarch64.dmg`
  - 主提交与 audit-only follow-up commit 完成
- Outputs to keep (10~20 lines snippet):
  - locale sync PASS
  - 测试与构建 PASS 摘要
  - `tauri build` 产物路径摘要
  - `git log --oneline -n 3`
