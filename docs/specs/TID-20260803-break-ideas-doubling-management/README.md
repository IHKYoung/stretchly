# Task-ID: TID-20260803-break-ideas-doubling-management

## Meta
- Title: 扩充并治理休息提示语
- Date: 2026-08-03
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE

## Links
- Plan (daily): ../../plans/2026-08-03.md
- Log (daily): ../../logs/2026-08-03.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 旧文案全部保留；三个 official locale 从 `364/244` 精确扩到 `728/488`，新增微休息 `364`、完整休息 `244`。
- `break-ideas/messages/*.json` 继续是唯一正文真源；批次 manifest 只记录 ID、类别、目标和 QA 预算，不复制文本。
- 新文案延续现有调侃、健康知识与情绪关照；微休息保持短促，完整休息允许标题加较长正文。
- 类别用于审计和相邻条目交错，不作为运行时权重或 UI 标签。
- 只维护 `zh-CN / zh-TW / en` 三个 official locale；legacy bundles 和运行时选择算法不变。

## Governance Notes
- Requirement Brief: 用户要求正式开始扩充，并要求本轮内容得到可持续管理；同时确认旧文案风格保留、避免模板化 AI 味、完整休息可以更长。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: 微休息每轮仍只显示一条；完整休息仍按稳定起点顺序轮换并在完整呈现后停留 60 秒；只改变可抽取内容池。
- Execution Safety Block: service_impact=none；touches_running_service=no；backup_required=no；backup_plan=依赖 git diff、批次 manifest、生成器和 tests；rollback_plan=回退新增 ID、manifest、校验、registry 与 docs；destructive_operations=none；operator_approval_required=no；rationale=本地静态内容与验证链路，不含发布/迁移/外部副作用。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Scope: 当前 break ideas 扩充、三语本地化、内容治理、测试与文档。
- Fallback Reason Code: other-blocked
- Warmup Required Roles: orchestrator,architect,coder,tester,scribe,reality_checker
- Warmup Ready Roles: single-agent-fallback（主 agent 覆盖限定角色）
- Warmup Agent IDs: none（会话规则禁止用户未明确要求时启动子 agent）
- Warmup Verification: BLOCKED：用户未请求 delegation；按会话规则采用范围明确的 single-agent fallback。
- Escalation Summary: 不触发外部发布、依赖安装或破坏性操作；若目标数量/语言范围变化再回到用户决策。
- Retention Decision: keep（正式文本、批次 manifest、生成门禁、测试和写作规范）；drop（一次性 authoring 中间文件）

## Notes
- 健康知识只使用稳定、轻量的屏幕休息事实，不写精确疾病风险或诊断；参考 AOA 20-20-20 与 HSE 短而频繁、离屏、变换姿势/活动的官方建议。
- 本任务不创建 commit，除非用户后续明确要求。
- 最终交付：official locale 各 `728/488`；batch schema v1、README、generator/content tests、generated registry、docs/evidence 均已落地。
- 验证：generator 幂等；Vitest `8 files / 132 tests`、typecheck、frontend build、changed-file Standard、Python compile、diff/workflow checks 通过；agent config 因仓库无 multi-agent config 正常 SKIP。
- 视觉证据缺口：Vite preview ready，但 Browser runtime 无可用实例，未采集截图；全量 runtime line-split round-trip 与 production build 已通过，但不替代截图。
