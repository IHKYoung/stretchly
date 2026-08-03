# Task-ID: TID-20260803-release-015-github-publish

## Meta
- Title: 发布 0.1.5 macOS 版本到 GitHub Release
- Date: 2026-08-03
- Level: complex
- Lane: deep
- Execution Profile: merge-gate
- Status: IN_PROGRESS

## Links
- Plan (daily): ../../plans/2026-08-03.md
- Log (daily): ../../logs/2026-08-03.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md
- Release Notes: ./release-notes.md
- Evidence: ./evidence/README.md

## Decision Log
- 2026-08-03：公开最新版仍为 `v0.1.4`，所有版本真源已是 `0.1.5`，本次发布目标确定为 `v0.1.5`，不额外提升到 `0.1.6`。
- 2026-08-03：`IHKYoung/Pauza` 的 GitHub Release 是公开安装包事实源；官网优先解析 latest release，固定下载回退同步到 `v0.1.5`。
- 2026-08-03：根仓库 `origin/baseline` 可快进更新；历史分叉的 `pauza/baseline` 不更新，只向两个远端推送同一个 `v0.1.5` tag。
- 2026-08-03：只发布签名、公证、装载和下载回验全部通过的 Apple Silicon DMG。

## Governance Notes
- Requirement Brief: 将已完成的 `0.1.5` 桌面端与提示语扩充收口为可公开下载的 macOS Apple Silicon 版本，并让官网稳定指向该 Release。
- Interaction Impact: indirect
- Interaction Freeze: 用户点击官网“下载”后，由 latest release 解析到 `Pauza_0.1.5_aarch64.dmg`；API 失败时回退到同一版本的固定资产 URL；失败必须留在页面并显式提示。
- Execution Safety Block: 触及 GitHub branch/tag/release、Apple 签名公证和官网部署；只允许快进 push，不覆盖远端资产，不改写历史。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 当前运行规则明确禁止启动 sub-agent；由 orchestrator 在同一工作树串行完成角色检查、构建、发布与现实回验。
- Retention Decision: 保留发布任务包、release notes、DMG SHA-256、GitHub/Vercel 现实状态与回验命令；本地构建产物仍按既有忽略规则管理。

## Notes
- 用户已在 2026-08-03 明确要求“打包一版发布到官网进行下载，GitHub release 管理”，构成此次外部发布授权。
- 删除远端 tag、Release 或资产、强推、改写历史均不在授权范围内。
