# Task-ID: TID-20260423-release-014-macos-signed-rebuild

## Test Strategy
- Unit:
  - N/A（本轮不改代码）
- Integration:
  - `printenv | rg '^(APPLE|CSC|NOTARY|TAURI)'`
  - `security find-identity -v -p codesigning`
  - `python3 scripts/validate_workflow_docs.py --mode manual`
- E2E (if applicable):
  - `npm --prefix apps/desktop run tauri build -- --bundles app,dmg`
  - `codesign -dv --verbose=4 apps/desktop/src-tauri/target/release/bundle/macos/Pauza.app`
  - `spctl --assess --type exec --verbose=4 apps/desktop/src-tauri/target/release/bundle/macos/Pauza.app`
  - `xcrun stapler validate apps/desktop/src-tauri/target/release/bundle/macos/Pauza.app`
  - `xcrun stapler validate apps/desktop/src-tauri/target/release/bundle/dmg/Pauza_0.1.4_aarch64.dmg`

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `printenv | rg '^(APPLE|CSC|NOTARY|TAURI)'` + `security find-identity -v -p codesigning`
- AC2 -> `npm --prefix apps/desktop run tauri build -- --bundles app,dmg`
- AC3 -> `codesign -dv --verbose=4 apps/desktop/src-tauri/target/release/bundle/macos/Pauza.app`
- AC4 -> `spctl --assess --type exec --verbose=4 apps/desktop/src-tauri/target/release/bundle/macos/Pauza.app` + `xcrun stapler validate ...`

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
- Artifacts path: docs/specs/TID-20260423-release-014-macos-signed-rebuild/evidence/
- What to capture:
  - Screenshots:
  - Video/trace (optional):
  - HAR/console logs (optional): 保留签名 identity、`codesign`、`spctl`、`stapler` 输出摘录即可

## Quality Gates (Non-functional)
- a11y:
  - N/A（本轮不涉及 UI）
- perf budget:
  - build 时间可接受，无需额外优化
- error handling / observability:
  - 必须保留签名与校验命令输出
- security / privacy:
  - 不把 Apple 凭证写入仓库文件；只使用当前 shell 已加载的环境变量

## Boundary / Invalid Input Cases
- 若 `codesign` 只显示 ad-hoc 或没有 `Developer ID Application: AHAKNOW LLC (HC559NT2NP)`，则视为失败
- 若 `spctl` 或 `stapler validate` 失败，则不接受该产物为“已签名可分发包”

## Concurrency / Race Cases (if applicable)
- N/A

## Mocks & Test Data
- 使用真实当前 shell Apple 环境、真实 keychain identity 与真实 `0.1.4` 本地产物

## Commands to Run
- `printenv | rg '^(APPLE|CSC|NOTARY|TAURI)'`
- `security find-identity -v -p codesigning`
- `npm --prefix apps/desktop run tauri build -- --bundles app,dmg`
- `codesign -dv --verbose=4 apps/desktop/src-tauri/target/release/bundle/macos/Pauza.app`
- `spctl --assess --type exec --verbose=4 apps/desktop/src-tauri/target/release/bundle/macos/Pauza.app`
- `xcrun stapler validate apps/desktop/src-tauri/target/release/bundle/macos/Pauza.app`
- `xcrun stapler validate apps/desktop/src-tauri/target/release/bundle/dmg/Pauza_0.1.4_aarch64.dmg`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - Apple signing / notarization 环境可见
  - keychain 中存在目标 `Developer ID Application` identity
  - 产出 `Pauza.app` 与 `Pauza_0.1.4_aarch64.dmg`
  - `.app` 显示正确签名身份，并通过本地 `spctl` 与 `stapler` 校验
- Outputs to keep (10~20 lines snippet):
  - env / identity 摘录
  - build 成功摘要
  - `codesign` / `spctl` / `stapler` 关键输出
