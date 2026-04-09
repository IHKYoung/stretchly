# Task-ID: TID-20260409-break-surface-i18n-reduction

## Meta
- Title: 收敛休息界面细节、接回完整多语言并审查 legacy app 资产
- Date: 2026-04-09
- Level: complex
- Lane: deep
- Execution Profile: sequential-phases
- Status: DONE

## Links
- Plan (daily): ../../plans/2026-04-09.md
- Log (daily): ../../logs/2026-04-09.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md
- Evidence: ./evidence/README.md

## Decision Log
- 先收敛 break / settings 当前体验，再做 i18n 架构抽离，最后审查 legacy `app/` 是否还能继续作为运行时依赖。
- 多语言目标不是继续堆 if/else，而是改成“每种语言一份消息文件 + 一份配置文件 + 自动注册”的目录结构。
- 为避免设置页再次出现多语言混杂，本轮只会把“桌面端文案已完整覆盖”的语言暴露为可选项；legacy 语言资产先纳入注册体系与审查范围。
- locale 同步脚本会把 legacy `app/locales/*.json` 复制到 `apps/desktop/src/locales/messages/*.json`，再为每种语言生成对应 `config/*.json` 与共享 `registry.generated.json`。
- legacy `app/` 当前仍不能整体删除：根入口 `package.json.main`、legacy npm scripts、测试文件与仓库文档仍显式引用该目录。

## Governance Notes
- Requirement Brief: 用户要求继续收敛 break prompt 和设置页体验，重点修复玻璃感、16:9 窗口约束、壁纸完整预览、结束音效与设置文案；随后把当前仅 `zh-CN/en` 的前后端 i18n 改成按语言文件/配置驱动的结构，并审查 legacy `app/` 是否仍承担桌面端运行职责。
- Interaction Impact: direct
- Interaction Freeze: break window 继续保持“单句交互语 + 大号数字倒计时 + 细条形进度”的纯净结构，不恢复左右分栏、标签化说明或组件堆叠；设置页继续采用中文用户向文案，不出现 `break window`、`prompt`、`manual finish` 这类专业术语直出。
- Execution Safety Block: 仅涉及本地桌面端前台、Tauri host settings schema、locale 加载链路与文档；不触及线上服务、外部副作用或历史重写。legacy `app/` 先审查后决定是否删除。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 当前 session 的上层运行策略禁止在未获用户显式授权时使用 `spawn_agent`；因此由单 agent 在限定范围内完成设计、实现、验证与文档闭环。
- Retention Decision: keep

## Notes
- 本任务分三阶段串行推进：
  - 阶段 1：break / settings 体验收敛。
  - 阶段 2：i18n 目录化、配置化与前后端自动注册。
  - 阶段 3：legacy `app/` 依赖审查与减法结论。
