# Task-ID: TID-20260724-site-latest-download-route

## Meta
- Title: 官网 latest 下载路径修复与部署
- Date: 2026-07-24
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: fast    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-07-24.md
- Log (daily): ../../logs/2026-07-24.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- GitHub Latest Release API 是最新资产名的事实源；页面不再要求 latest 资产名与 pinned fallback 完全一致。
- 运行时使用 API 返回的 `_aarch64.dmg` 资产名拼接 `https://github.com/IHKYoung/Pauza/releases/latest/download/<asset-name>`。
- `v0.1.4` 固定 URL 仅用于 API 请求失败、响应异常或缺少匹配资产时的可用性回退。
- `IHKYoung/Pauza` 的 `baseline` 继续只承载官网；父仓库源码不推入该分支。

## Governance Notes
- Requirement Brief: 更新并部署官网，使下载按钮始终跟随 GitHub latest release，并以固定 `v0.1.4` 地址作为失败回退；不改变页面视觉和其它交互。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: 下载按钮外观、文案、焦点语义和页面其它交互保持不变，只调整最终 href 的解析契约。
- Execution Safety Block: `service_impact=yes; touches_running_service=yes; backup_required=yes; backup_plan=保留远端 bad94ee 与 Vercel 上一生产部署; rollback_plan=git revert 本任务提交并推送 baseline，或在 Vercel 回滚上一部署; destructive_operations=none; operator_approval_required=yes; approval=用户于 2026-07-24 明确要求更新官网`
- Approval Owner: orchestrator
- Delegation Policy: `orchestrator` 在用户已授权其负责日常编排后，可自主决定是否 `spawn_agent`
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 会话级多 Agent 规则禁止在用户未明确要求 delegation 时 spawn；由主 agent 限定承担 scribe/coder/tester/evidence_collector，并由用户明确批准生产推送。
- Retention Decision: keep（latest 路径契约、fallback 和发布证据均长期保留）

## Notes
- Source Basis: `index.html`、`download/targets.js`、`script.js`、`README.md`、`scripts/publish_site_release.py`、远端 `baseline=bad94ee`、GitHub latest API 与 Vercel Production deployment 记录。
- Pre-deploy verification: latest 路径 302 到 `v0.1.4`；三条下载解析 smoke PASS；`node --check`、`git diff --check`、113 项 Vitest PASS。
- Production verification: Vercel deployment `5585280999` 在 SHA `9bf7456` 上成功；线上最终 DOM href 为 `releases/latest/download/Pauza_0.1.4_aarch64.dmg`，且 GitHub 302 到 v0.1.4 tagged asset；详见 `./evidence/README.md`。
