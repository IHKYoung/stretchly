# Task-ID: TID-20260423-release-014-macos-signed-rebuild

## Meta
- Title: 补做 0.1.4 macOS 开发者签名打包
- Date: 2026-04-23
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-23.md
- Log (daily): ../../logs/2026-04-23.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 上一轮 `0.1.4` 打包显式使用了 `--no-sign`，因此本轮只补“签名版重打包”，不再重复版本 bump 或代码提交。
- 直接复用当前 `zsh` login shell 中的 Apple 签名/公证环境变量，不再额外把敏感配置写回仓库文件。
- 验证标准不只看产物是否生成，还要覆盖 `codesign`、`spctl` 与 `xcrun stapler validate`。

## Governance Notes
- Requirement Brief: 用户指出上一轮打包忘记开发者签名，并明确说明 `zshrc` 中已有可用签名配置；本任务据此只补做 `0.1.4` 的带签名 macOS 打包与本地验证，不修改代码、不追加新 commit。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: service_impact=仅影响本地 `target/release/bundle/**` 签名产物与本任务 workflow docs；touches_running_service=no；backup_required=no；backup_plan=`printenv | rg '^(APPLE|CSC|NOTARY|TAURI)'`、`security find-identity -v -p codesigning`、`npm --prefix apps/desktop run tauri build -- --bundles app,dmg`、`codesign -dv --verbose=4 <app>`、`spctl --assess --type exec --verbose=4 <app>`、`xcrun stapler validate <app>`、`xcrun stapler validate <dmg>`；rollback_plan=删除新签名产物并重新用正确环境重建；destructive_operations=none；operator_approval_required=no；rationale=用户已明确要求补签名，本轮不涉及代码改动、历史重写或线上系统。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 当前 session 未获用户显式授权调用 `spawn_agent`，且上层工具策略要求只有在用户明确要求子 agent 时才可 delegation；因此在限定范围内采用单 agent 执行签名补打包。
- Retention Decision: keep

## Notes
- 当前 shell 已读到：`APPLE_ID`、`APPLE_PASSWORD`、`APPLE_SIGNING_IDENTITY=Developer ID Application: AHAKNOW LLC (HC559NT2NP)`、`APPLE_TEAM_ID=HC559NT2NP`。
- keychain 中可用 codesign identity 包括 `Developer ID Application: AHAKNOW LLC (HC559NT2NP)`。
- 已完成签名验证：
  - `Pauza.app`：`Authority=Developer ID Application: AHAKNOW LLC (HC559NT2NP)`、`Notarization Ticket=stapled`、`spctl` accepted、`stapler validate` PASS
  - `Pauza_0.1.4_aarch64.dmg`：签名 identity 正确；经单独 `notarytool submit` + `stapler staple` 后，`spctl` accepted、`stapler validate` PASS
