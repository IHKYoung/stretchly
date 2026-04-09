# Task-ID: TID-20260408-break-prompt-purity-pass

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 把 break prompt 文案来源统一收回 locale 资源层，避免继续在 TS helper 中硬编码。
- 在不修改 host 合约和 settings schema 的前提下，完成一次纯前台 break prompt 收敛。

## Non-Goals
- 不改 `DesktopSnapshot` / `CurrentBreakSnapshot` 的 Rust host 输出结构。
- 不新增 locale 加载依赖或远程配置能力。

## Constraints & Assumptions
- 不能新增依赖。
- 现有 `breakIdeasEnabled` 设置语义要保留，只改变 break prompt 的呈现方式。
- locale 读取层当前只支持字符串；本轮需在不破坏既有 `t()` 用法的前提下增加数组读取能力。

## System Boundaries
- Modules:
  - `apps/desktop/src/App.tsx`：break prompt 视图与 CTA
  - `apps/desktop/src/i18n.ts`：locale lookup / 插值 / 数组读取
  - `apps/desktop/src/lib/break-prompt.ts`：prompt 选择、背景和音频 helper
  - `apps/desktop/src/locales/{zh-CN,en}.json`：break prompt copy 真源
- Ownership:
  - UI 布局在 `App.tsx`
  - copy 真源在 locale JSON
  - prompt 选择策略在 `break-prompt.ts`
- Dependency direction:
  - `App.tsx` -> `i18n.ts` / `break-prompt.ts`
  - `break-prompt.ts` -> `i18n.ts`
  - `i18n.ts` -> locale JSON
  - 不反向依赖 Rust host

## API / Contract
- Signatures / Endpoints:
  - `t(language, key, vars?): string`
  - `tList(language, key): string[]`
  - `pickBreakPrompt(language, kind, startedAtMs): string`
- Request/Response schema (typed):
  - 无新增 IPC；沿用现有 `CurrentBreakSnapshot.kind` 和 `PauzaSettings.breakIdeasEnabled`
- Error model (codes, retryability):
  - 无新增 error code；locale 缺失时前台回退到默认 prompt

## Data Model / Storage
- 在 `apps/desktop/src/locales/{zh-CN,en}.json` 中新增：
  - `ui.break.defaultPrompt.microbreak|longBreak`
  - `ui.break.prompts.microbreak|longBreak`
- 不新增 settings 持久化字段。

## Invariants
- break prompt 主文案不得再从 TS 常量数组读取。
- locale prompt 数组为空时，必须稳定回退到默认 prompt。
- break prompt 布局保持单列，不因 break kind 或宽屏进入双栏分叉。

## Concurrency / Lifecycle / Memory Model
- break prompt 仍按现有 1s snapshot polling 更新。
- 倒计时和进度条基于同一个 `remaining` 计算，避免出现两套计时源漂移。

## Observability Plan (Debug-Driven)
- Logs: 无新增 runtime log；问题排查优先看 locale lookup、`pickBreakPrompt()` fallback 和前端 build/typecheck
- Metrics: N/A
- Traces: N/A
- Debug flags: N/A

## Security & Privacy Considerations
- 本轮不新增网络请求、权限或数据持久化；自定义壁纸路径与既有逻辑保持不变。

## Risks & Rollback
- Failure modes:
  - locale prompt key 缺失导致 break prompt 暴露 key string
  - 纯净布局在特殊状态下信息不足
- Rollback steps:
  - 回退 `App.tsx`、`i18n.ts`、`break-prompt.ts` 与 locale JSON；无需迁移数据

## Acceptance Criteria (System)
- locale 数组读取不影响现有 `t()` 字符串读取。
- `break-prompt.ts` 不再保存 break prompt copy 常量。
- 构建通过，且 break prompt fallback 路径可由代码审查证明。

## Open Questions / Decision Requests
- 暂无额外决策请求。本轮已明确采用条形进度而不是圆环。
