# Task-ID: TID-20260411-site-symbol-particles

## Meta
- Title: 官网强化终端提示头并改为符号粒子
- Date: 2026-04-11
- Level: trivial  <!-- trivial | moderate | complex -->
- Lane: fast    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-11.md
- Log (daily): ../../logs/2026-04-11.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md
- Evidence Report: ./evidence/README.md

## Decision Log
- 左上角 `Pauza>` 进一步加重，强化终端 prompt 感。
- 粒子从模糊光点改成以 `0 / 1 / #` 为主的符号字元。
- 主输出文案宽度按当前浏览器页面宽度 `80%` 收敛。

## Governance Notes
- Requirement Brief: 用户要求让 `Pauza>` 更明显，并把粒子改成更像程序员/码字工作者常见的 `0 / 1 / #` 等符号；随后又补充要求让网站输出文案宽度按浏览器页面宽度 `80%` 收敛。
- Interaction Impact: direct
- Interaction Freeze: 已冻结为 `更醒目的左上终端 prompt + 80% 页面宽度输出区 + 以符号字元为主的粒子反馈`；本轮不改下载路由、文案池内容和打字机节奏。
- Execution Safety Block: service_impact=仅限 `apps/site` 首页 prompt 样式、输出区宽度、粒子字元与相关 docs/evidence；touches_running_service=no；backup_required=no；backup_plan=依赖本地静态服务器、DOM 度量、浏览器快照、console 摘要与 workflow docs validator；rollback_plan=回退 `apps/site/{styles.css,script.js}` 与相关 docs；destructive_operations=none；operator_approval_required=no；rationale=纯静态站点前端样式/脚本微调，不涉及线上服务、权限、数据或外部副作用。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无；本轮只在首页现有结构上增强终端感和粒子语义。
- Retention Decision: keep

## Notes
- 证据以 DOM 度量、console 摘要和浏览器快照为主，足以覆盖本轮“样式权重 + 输出宽度 + 粒子字元”变化。
