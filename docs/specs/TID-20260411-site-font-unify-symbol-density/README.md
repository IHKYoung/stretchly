# Task-ID: TID-20260411-site-font-unify-symbol-density

## Meta
- Title: 官网统一 LXGW 字体并增强符号粒子密度
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
- 官网首页所有可见文字统一回到 `LXGW WenKai Screen`。
- `Pauza>` 保持同一字体，不再单独走另一套 monospace。
- 粒子字符池收敛到 `0 / 1 / # / @ / ！ / ¥ / $`，并抬高数量和字号。

## Governance Notes
- Requirement Brief: 用户要求让 `Pauza>` 也保持使用 `LXGW` 字体，并让整个网站保持一个统一字体；同时要求粒子里的 `0 / 1 / # / @ / ！ / ¥ / $` 更多、更大、更密。
- Interaction Impact: direct
- Interaction Freeze: 已冻结为 `全站统一 LXGW 字体 + Pauza> 同字体 + 80vw 输出区保持不变 + 粒子以 0/1/#/@/！/¥/$ 为主并增加密度`；本轮不改下载路由、文案池和整体布局。
- Execution Safety Block: service_impact=仅限 `apps/site` 首页字体、粒子密度与相关 docs/evidence；touches_running_service=no；backup_required=no；backup_plan=依赖本地静态服务器、DOM 度量、浏览器快照、console 摘要与 workflow docs validator；rollback_plan=回退 `apps/site/{styles.css,script.js}` 与相关 docs；destructive_operations=none；operator_approval_required=no；rationale=纯静态站点前端样式/脚本微调，不涉及线上服务、权限、数据或外部副作用。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无；本轮只收敛首页字体和粒子密度。
- Retention Decision: keep

## Notes
- 证据见 `./evidence/README.md`；其中包含统一字体、输出宽度与粒子符号采样结果。
