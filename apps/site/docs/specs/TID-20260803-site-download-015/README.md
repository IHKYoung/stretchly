# Task-ID: TID-20260803-site-download-015

## Meta
- Title: 更新官网下载目标到 Pauza 0.1.5
- Date: 2026-08-03
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: IN_PROGRESS

## Links
- Plan (daily): ../../plans/2026-08-03.md
- Log (daily): ../../logs/2026-08-03.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md
- Evidence: ./evidence/README.md

## Decision Log
- 2026-08-03：保留 GitHub latest release 解析为主路径，只把 pinned fallback 更新到 `v0.1.5`。
- 2026-08-03：GitHub Release 创建并通过远端下载回验后才推送生产站点 fallback 提交。
- 2026-08-03：`copy.js`、`docs/ReleasePlaybook.md` 与两份模板中的既有本地修改不属于本任务，不暂存、不回滚。

## Governance Notes
- Requirement Brief: 让官网 primary/fallback 下载都稳定到达已验证的 `Pauza_0.1.5_aarch64.dmg`。
- Interaction Impact: indirect
- Interaction Freeze: 下载按钮交互不变；latest API 成功走 latest URL，失败走同版本 pinned URL，再失败显示既有错误。
- Execution Safety Block: 触及生产站点；仅快进 push，不覆盖远端历史，Release 不存在或未验证时停止部署。
- Approval Owner: orchestrator
- Delegation Policy: 本任务受会话规则限制，不启动 sub-agent。
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 会话规则明确禁止未获请求时 delegation；orchestrator 只在列明文件范围内串行闭环。
- Retention Decision: 保留下载目标、任务文档、变更日志、commit audit 与生产 HTTP/Release evidence。

## Notes
- 用户已明确授权发布安装包并让官网可下载。
- 删除部署、Release 或 tag 不在本任务授权范围内。
