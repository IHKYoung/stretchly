# Task-ID: TID-20260723-release-014-github-publish

## Meta
- Title: 发布 0.1.4 macOS 版本到 GitHub Release
- Date: 2026-07-23
- Level: complex
- Lane: deep
- Execution Profile: merge-gate
- Status: DONE

## Links
- Plan (daily): ../../plans/2026-07-23.md
- Log (daily): ../../logs/2026-07-23.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md
- Release Notes: ./release-notes.md
- Evidence: ./evidence/README.md

## Decision Log
- 公开版本锁定为 `v0.1.4`；四个 manifest 已是 `0.1.4`，不预升到 `0.1.5`。
- 将 0.1.4 本地打包后仍列在“未发布”的 reminder、break ideas 与结构整理纳入首次公开的 0.1.4 快照。
- GitHub Release 资产继续发布到 `IHKYoung/Pauza`；代码提交沿当前 `baseline` 分支 fast-forward 到 `origin`，同一 `v0.1.4` tag 推到 `origin` 与 `pauza`。由于 `pauza/baseline` 与当前历史不相交，本轮不改写该分支。
- macOS 安装包必须重新从当前快照构建；不得复用 2026-05-06 的旧 bundle 冒充当前产物。

## Governance Notes
- Requirement Brief: 用户要求把当前版本打包发布到 GitHub Release，并同时更新本地版本；本任务据此冻结当前工作树为 `v0.1.4`，同步本地站点与版本说明，验证、提交、推送并发布 macOS arm64 DMG。
- Interaction Impact: indirect
- Interaction Freeze: 用户可见下载入口只从 `v0.1.3/Pauza_0.1.3_aarch64.dmg` 切换到 `v0.1.4/Pauza_0.1.4_aarch64.dmg`；GitHub API 失败时仍保留该固定链接。
- Execution Safety Block: service_impact=更新 GitHub 代码分支、tag、release 与官网下载目标；touches_running_service=yes（GitHub Release/下载入口）；backup_required=no；backup_plan=发布前完成测试、构建、哈希和签名检查，并保留旧 v0.1.3 release；rollback_plan=代码用后续 revert，站点链接恢复到 v0.1.3，错误 release/tag 仅在用户再次授权后删除或替换；destructive_operations=none；operator_approval_required=yes；rationale=用户已明确授权发布当前版本和更新本地版本，但凭据恢复与签名降级仍必须显性处理。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 上层策略禁止在用户未明确要求时启动子 agent，本任务由单 agent 在限定发布边界内执行；GitHub token 失效与签名 identity 缺失作为显性门禁保留。
- Retention Decision: keep（发布记录、固定下载契约与验证证据用于后续版本复用）

## Notes
- 当前 GitHub CLI 已通过官方设备授权恢复为 `IHKYoung`，token scope 包含本次 release 所需的 `repo`。
- `security find-identity` 虽未列出 identity，但 Tauri 实际成功调用 `Developer ID Application: AHAKNOW LLC (HC559NT2NP)` 完成 `.app` / `.dmg` 签名；最终以 `codesign`、`spctl` 与 `stapler` 的产物核验为准。
- 当前 DMG 已由 Apple notarization 接受并 stapled；commit、tag、GitHub Release、latest 状态与真实下载 hash 均已核验通过。
