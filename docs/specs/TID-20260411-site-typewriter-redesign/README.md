# Task-ID: TID-20260411-site-typewriter-redesign

## Meta
- Title: 官网重设计为极简打字机单页
- Date: 2026-04-11
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
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
- 官网首页从“多信息面板 landing page”进一步收敛为极简单舞台结构：纸面背景 + 打字机文案 + 单下载按钮。
- 展示文案不新编一套 marketing slogan，而是直接精选自桌面端 App 已内嵌的提醒文案，保留 Pauza 自己的语气。
- 为了稳定使用“落霞孤鹜 / 霞鹜”风格，不走外部字体 CDN，而是把仓库中已存在的 `LXGW WenKai Screen` 字体复制到 `apps/site/fonts/`。
- 视觉再次收刀：首页只保留 `pauza>` 提示、动态文案和下载按钮，移除了额外说明字，让文案成为唯一主角。

## Governance Notes
- Requirement Brief: 用户要求把刚做好的官网改成极简版本：纯白纸面背景，中央只保留一个类似命令行打字机的动态文案区，循环展示 App 内嵌的调侃式提醒文案，并在右上角保留一个简单下载按钮。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: frozen；本轮只改官网首页视觉与文字展示方式，不改下载稳定路由结构，不接入真实 GitHub 下载地址。
- Execution Safety Block: service_impact=仅限 `apps/site` 官网首页重构、站点文案源与字体资源新增、docs 同步与本地浏览器验证；touches_running_service=no；backup_required=no；backup_plan=通过本地静态服务器、浏览器现实检查、截图证据与 workflow docs validator 验证；rollback_plan=回退 `apps/site` 首页相关文件与本任务 docs 后恢复到上一版站点；destructive_operations=none；operator_approval_required=no；rationale=不涉及线上服务、数据迁移、权限提升或外部付费动作。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: BLOCKED: 当前 session 未获用户显式 delegation 授权，遵循上层工具策略不调用 `spawn_agent`；本轮由单 agent 在限定范围内完成官网重设计、验证与文档闭环。
- Retention Decision: keep

## Notes
- 官网首页的打字机文案当前精选自 `apps/desktop/src/locales/messages/zh-CN.json` 中的 `miniBreakIdeas`，后续若要扩展更多站点文案，应继续优先从 App 真源中抽取。
