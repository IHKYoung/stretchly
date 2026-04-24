# Task-ID: TID-20260423-release-014-macos-signed-rebuild

## Summary
- Title: 补做 0.1.4 macOS 开发者签名打包
- Date: 2026-04-23
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 在当前 `0.1.4` 工作树不变的前提下，复用本机 `zshrc` 中的 Apple Developer 配置重新构建 macOS 签名包，并验证 `.app` / `.dmg` 已带开发者签名且通过本地 Gatekeeper / stapler 检查。
- In-scope:
  - 检查当前 login shell 中的 Apple 签名/公证环境变量
  - 检查 keychain 中可用的 `Developer ID Application` identity
  - 重新运行 `tauri build -- --bundles app,dmg`，不再使用 `--no-sign`
  - 对新产物执行 `codesign`、`spctl`、`stapler` 验证
  - 补齐本任务 docs/logs/plans
- Out-of-scope:
  - 代码或文档功能性修改
  - 版本号调整
  - 重新提交代码或推送远端
  - GitHub release 上传
- Assumptions:
  - 当前 `zsh` login shell 会加载用户在 `~/.zshrc` 中配置的 Apple 签名/公证变量
  - `APPLE_ID / APPLE_PASSWORD / APPLE_TEAM_ID` 足够支撑 notarization
  - 当前 `0.1.4` 构建输入无需再改代码
- Risks:
  - 虽然存在 signing identity，但 notarization 可能因 Apple 凭证或网络问题失败
  - Tauri 可能只完成 codesign，没有完成 notarization / staple
  - 旧的未签名产物与新的签名产物同名，若不做验证容易误判
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 产出一份确实带 `Developer ID Application: AHAKNOW LLC (HC559NT2NP)` 签名的 `0.1.4` macOS `.app` / `.dmg`，而不是仅仅重新跑一遍 build。

## Scope
- In-scope:
- In-scope:
  - `apps/desktop/src-tauri/target/release/bundle/macos/Pauza.app`
  - `apps/desktop/src-tauri/target/release/bundle/dmg/Pauza_0.1.4_aarch64.dmg`
  - 当前 shell 的 Apple signing / notarization 环境
  - `docs/logs/2026-04-23.md`
  - `docs/plans/2026-04-23.md`
  - `docs/specs/TID-20260423-release-014-macos-signed-rebuild/**`
- Out-of-scope:
  - 仓库源码与版本真源
  - 新 commit
  - 远端 release

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src-tauri/tauri.conf.json`
  - `apps/desktop/package.json`
  - 当前 shell 环境变量输出
  - `security find-identity -v -p codesigning`
- Related docs/specs/logs reviewed:
  - `README.md`
  - `docs/RepositoryGuidelines.md`
  - `docs/specs/TID-20260423-release-014-macos-package-commit/*`
  - `docs/logs/2026-04-23.md`
  - `docs/plans/2026-04-23.md`
- Why these are sufficient:
  - 已覆盖当前打包入口、Tauri bundle 配置、上一轮无签名 release 记录，以及本机可用的 signing identity / env，足以完成签名补打包与验证。

## Acceptance Criteria (AC)
- AC1: 当前 login shell 能读取到 Apple signing / notarization 所需环境，且 keychain 中存在 `Developer ID Application: AHAKNOW LLC (HC559NT2NP)`。
- AC2: `npm --prefix apps/desktop run tauri build -- --bundles app,dmg` 成功，并重新生成 `Pauza.app` 与 `Pauza_0.1.4_aarch64.dmg`。
- AC3: `codesign -dv --verbose=4 Pauza.app` 显示签名身份为 `Developer ID Application: AHAKNOW LLC (HC559NT2NP)`。
- AC4: `spctl --assess --type exec --verbose=4 Pauza.app` 与 `xcrun stapler validate` 对 `.app` / `.dmg` 的校验通过。

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
- service_impact: 仅影响本地 `target/release/bundle/**` 签名产物与本任务 workflow docs
- touches_running_service: no
- backup_required: no
- backup_plan: `printenv | rg '^(APPLE|CSC|NOTARY|TAURI)'`、`security find-identity -v -p codesigning`、`npm --prefix apps/desktop run tauri build -- --bundles app,dmg`、`codesign -dv --verbose=4 <app>`、`spctl --assess --type exec --verbose=4 <app>`、`xcrun stapler validate <app>`、`xcrun stapler validate <dmg>`
- rollback_plan: 删除新签名产物并重新用正确环境重建；若 notarization 卡住，保留失败日志并回退到未签名产物仅作为本地参考，不当作可分发版本
- destructive_operations: none
- operator_approval_required: no
- rationale: 用户已明确要求补开发者签名，且当前 shell 已暴露相关 Apple 环境变量。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 验证 Apple 签名环境与 identity 可用
  - DoD: env 与 keychain identity 输出可追溯写入日志
- [x] Task-2: 重新构建并验证签名产物
  - DoD: 生成新的 `Pauza.app` / `Pauza_0.1.4_aarch64.dmg`，并通过 `codesign`、`spctl`、`stapler` 校验

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260423-release-014-macos-signed-rebuild/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading:
  - empty:
  - error:
  - disabled:
  - success:

## Observability / Debug Plan
- Logs:
  - 记录 env 发现、keychain identity、签名 build 输出与 `codesign/spctl/stapler` 结果
- Error codes:
  - N/A
- Trace/metrics (optional):
  - N/A
- Debug flags (optional):
  - 不使用 `--no-sign`

## Risks & Rollback
- Risks:
  - notarization 失败但 build 仍生成签名 app，容易被误判为“完整成功”
  - 当前环境变量若只够签名不够 notarization，`stapler validate` 会暴露问题
- Rollback plan:
  - 保留失败日志；删除有问题的产物后重新以正确凭证重建

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 检查当前 login shell 的 Apple signing / notarization 环境与 keychain identity。
  2. 运行带签名的 `tauri build -- --bundles app,dmg`。
  3. 对 `.app` / `.dmg` 执行 `codesign`、`spctl`、`stapler` 校验并回填日志。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: 用户已在 2026-04-23 明确要求“需要签名，我的 zshrc 里有签名可以用”
