# Task-ID: TID-20260803-break-ideas-doubling-management

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 扩大用户在微休息与完整休息时看到的自然口语提示池，让重复感下降，同时保持现有交互节奏。
- 微休息快速促成一个小动作；完整休息允许更完整的场景、调侃和情绪照顾。

## Non-Goals
- 不调整 break 页面布局、按钮、倒计时、字体、动画速度或停留时长。
- 不在 UI 暴露内部类别、批次号或健康来源。

## Screens & User Flows
- Primary flow: break 开始 -> 当前语言池选出 prompt -> 逐字呈现；完整休息在正文完整出现后停留 60 秒，再进入下一条。
- Fallback / secondary flow: 关闭 ideas 或 source 为空时继续显示现有 default prompt；legacy locale 按现有 fallback 工作。
- User-visible boundary: break 窗口中央标题与正文，不包含新增管理信息。
- Entrypoints / handoff cues: `Pauza>` prompt 与既有“继续工作/完成/稍后/跳过”动作。

## Component Tree
- 既有 `App.tsx` break surface -> prompt title/body -> typewriter；无组件树变化。

## Interaction States
- hover: 不变。
- active: 不变。
- focus: 不变。
- disabled: `breakIdeasEnabled=false` 时走现有默认文案。
- loading: 不变，无新增加载态。
- empty: source 为空时走现有默认文案。
- error: 构建期由 generator 拒绝无效内容；runtime 不新增错误态。
- skeleton: N/A。
- optimistic (if applicable): N/A。

## Responsive Rules
- 沿用现有断行与 viewport 适配；长度门槛限制极端长文，最长样本需要预览/构建证据。

## Accessibility (a11y)
- keyboard navigation: 不变。
- focus order: 不变。
- aria labels: 不变，无新增控件。
- contrast: 不变。
- reduced motion: 沿用现有行为，本任务不改动画契约。

## Design Tokens / Tailwind Mapping
- typography: 沿用现有 prompt title/body 层级。
- spacing: 不变。
- color usage: 不变。
- key classes: 不变。

## Micro-animations (optional)
- 沿用打字机与完整休息切换间隔，不新增动画。

## Edge Cases
- long text: 完整休息允许更长，但受 locale 字符预算与最长样本检查保护。
- slow network: N/A，资源随应用打包。
- empty datasets: 现有 default prompt fallback。
- permission denied: N/A。
- offline: 全功能可用。

## Acceptance Criteria (UI)
- 微休息仍为一条稳定短提示；完整休息仍按顺序轮换且相邻类别不扎堆。
- 三种 official locale 的新增标题/正文可正常解析、打字和换行，既有动作区不变。
- 新文案不显示内部 ID、类别、批次或来源信息。

## Open Questions
- 视觉结构无待决问题；截图若受本地 browser backend 阻塞，将在 evidence 显性记录。
