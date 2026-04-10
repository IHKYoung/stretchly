# Task-ID: TID-20260409-smart-reminder-decay-thresholds

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 用更准确的文案表达“先等更自然空档，再逐步放宽”的 smart reminder 语义。

## Non-Goals
- 不新增任何设置项。
- 不重做设置页布局或 break prompt 视觉结构。

## Screens & User Flows
- Primary flow: 用户看到的不是一组数字参数，而是 reminder mode 与状态文案；当 due 但仍在忙时，状态会表达“先等更自然的空档”，而不是像卡住。
- Fallback / secondary flow: smart reminder 一直等不到明显空档时，系统会自动放宽阈值并最终开始 break。
- User-visible boundary: 影响 reminder mode 提示文案、waiting 状态详情和 adaptive heads-up 文案。
- Entrypoints / handoff cues: reminder mode 说明、runtime waiting status、pre-break adaptive notification。

## Component Tree
- `messages/{en,zh-CN}.json`
  - `ui.smartPauseHint`
  - `ui.reminderModeHint`
  - `runtime.actions.waitingForOpportunity`
  - `runtime.break.status.waitingOpportunityDetail`
  - `runtime.notifications.*AdaptiveSoon`

## Interaction States
- hover:
  - 无变化
- active:
  - 无变化
- focus:
  - 无变化
- disabled:
  - 无新增 disabled 态
- loading:
  - 无
- empty:
  - 无
- error:
  - 无新增 error 态
- skeleton:
  - 无
- optimistic (if applicable):
  - 无

## Responsive Rules
- 仅文案变化，无新增布局要求。

## Accessibility (a11y)
- keyboard navigation:
  - 无变化
- focus order:
  - 无变化
- aria labels:
  - 无变化
- contrast:
  - 无变化
- reduced motion:
  - 无变化

## Design Tokens / Tailwind Mapping
- typography:
  - 无变化
- spacing:
  - 无变化
- color usage:
  - 无变化
- key classes:
  - 无变化

## Micro-animations (optional)
- 无

## Edge Cases
- long text:
  - 新文案需要能在 status/detail 区自然换行
- slow network:
  - N/A
- empty datasets:
  - N/A
- permission denied:
  - N/A
- offline:
  - N/A

## Acceptance Criteria (UI)
- 文案不再暗示“单一短空档”，而是准确表达“先等更自然空档，再逐步放宽”。
- 不新增任何用户可调参数控件。

## Open Questions
- 无
