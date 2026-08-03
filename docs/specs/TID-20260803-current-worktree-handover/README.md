# Task-ID: TID-20260803-current-worktree-handover

## Meta
- Title: 收口当前桌面端与官网工作树
- Date: 2026-08-03
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-08-03.md
- Log (daily): ../../logs/2026-08-03.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 父仓库本次收口当前所有可见、可维护的源码/文档修改；`scratch/locale-translations/` 继续作为本地翻译实验目录忽略，只提交其边界说明 `scratch/README.md`。
- 智能投递等待剩余时间由 Rust runtime 计算，通过 `DesktopSnapshot.next_break_wait_remaining_ms` 暴露；tray 只展示，不从计划倒计时或前台时间自行合成。
- 智能等待倒计时属于到点后的临时操作反馈，因此即使用户关闭常规 tray 倒计时也继续显示；pause/focus/DND/app exclusion 等 blocker 仍按既有优先级覆盖或冻结等待。
- 设置页保持既有信息架构，只补齐节奏字段的完整句式、自定义输入入口和统一文字层级；不改设置 schema、autosave 或 profile 数值。
- 官网 latest 下载任务已在 `apps/site` 独立仓库完成并部署；父仓库本次只纳入对应代码、说明、spec/evidence 与审计快照，不执行 push/release。

## Governance Notes
- Requirement Brief: 用户要求先把已有修改形成一条详细、可审计的 commit；本任务负责审阅、补文档、验证并提交父仓库当前有效工作树。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: tray 在智能等待阶段显示后端倒计时；设置节奏页展示“每隔/持续/提醒周期/自定义”完整语义；官网按钮视觉不变，latest API 失败仍回退 pinned URL。
- Execution Safety Block: service_impact=none；touches_running_service=no；backup_required=no；backup_plan=提交前保留完整 diff/status 与验证记录；rollback_plan=普通 `git revert`；destructive_operations=none；operator_approval_required=no；rationale=用户已明确授权本地 commit，未授权也不执行外部发布或历史改写。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Scope: 当前父仓库 worktree 审阅、验证、文档、commit 与 audit closure。
- Fallback Reason Code: other-blocked
- Warmup Required Roles: orchestrator,architect,coder,tester,scribe,reality_checker
- Warmup Ready Roles: single-agent-fallback（主 agent 覆盖限定角色）
- Warmup Agent IDs: none（会话规则禁止用户未明确要求时启动子 agent）
- Warmup Verification: BLOCKED：用户未请求 delegation；按会话规则采用范围明确的 single-agent fallback。
- Escalation Summary: 会话规则禁止在用户未明确要求 delegation 时启动子 agent；主 agent 仅在当前父仓库范围内完成审阅、验证、文档和提交。
- Retention Decision: keep（运行态契约、设置语义、官网 latest 快照、测试与交接记录）；drop（本地翻译生成中间物）

## Notes
- `apps/site` 是嵌套独立 Git 仓库；其 `copy.js`、`docs/ReleasePlaybook.md` 与两份 spec template 仍有独立仓库既有未提交修改，但这些内容已与父仓库 `HEAD` 一致，不属于本次父仓库 diff，不在本次提交中改写。
- 本任务不包含前序讨论中的微休息/完整休息提示语扩充。
- 当前周期通过 127 项前端测试、31 项 Rust 测试、typecheck、production build、三条 site download smoke、两套 locale generator、targeted lint、diff 与 workflow checks；`cargo fmt --check` 因本机缺少 rustfmt 未运行，根 lint 因扫描 Tauri target 和既有 site/test 尾逗号风格失败，浏览器截图因无可用 browser backend 缺失。
