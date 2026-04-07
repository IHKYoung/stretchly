# Task-ID: TID-20260403-break-window-scale-fix

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 去掉 break 页“窗口里再套卡片”的层级。
- 让倒计时、标题和按钮直接占用 break window 本身，而不是缩在中间一小块。

## Non-Goals
- 不重新设计 break 页的动作语义。
- 不新增新的打断风格或设置项。
- 不改主设置页结构。

## Screens & User Flows
- Primary flow: break 打开后，用户直接面对一个填满当前宿主窗口的休息页面，顶部是 break kind / 时钟，中部是倒计时和说明，底部是操作按钮。
- Fallback / secondary flow: 浏览器 preview 的 `?window=break` 直接渲染模拟 break，方便视觉校验。
- User-visible boundary: 仅 break prompt 页面尺度和结构改变。
- Entrypoints / handoff cues: 顶部 badge、中央倒计时、底部 `完成 / 稍后 / 跳过`。

## Component Tree
- `BreakWindow`
- top meta row
- circular timer block
- title/detail block
- CTA row

## Interaction States
- hover: 按钮维持现有 hover。
- active: 按钮维持现有 active。
- focus: 按钮保持原有 focus ring。
- disabled: busyAction 时按钮禁用。
- loading: 非 break preview 路径不变。
- empty: `currentBreak=null` 时显示 cleared 文案。
- error: 复用命令错误处理。
- skeleton: 无
- optimistic (if applicable): preview route 的按钮继续走本地 transform。

## Responsive Rules
- 页面本身直接填满宿主窗口；不再额外限制 `max-w-[460px]`。
- 长休息比微休息使用更大的内边距和更大的倒计时环。

## Accessibility (a11y)
- keyboard navigation: CTA 顺序维持不变。
- focus order: 从主 CTA 到次级 CTA 自然流动。
- aria labels: 继续复用现有按钮文本。
- contrast: 蓝色进度环和深色文本保持可读。
- reduced motion: 去掉装饰性 orb 背景，只保留轻量进入动效。

## Design Tokens / Tailwind Mapping
- typography: 标题和倒计时放大，充分利用 break window 空间。
- spacing: 通过整页 padding 控制留白，不再依赖内层 card。
- color usage: 使用更轻的蓝白渐变背景，避免再出现大白框里套小白卡。
- key classes:
  - shell: `min-h-screen bg-[linear-gradient(...)]`
  - content width: `max-w-[760px]`
  - timer size: `size-44/52/56/64`

## Micro-animations (optional)
- 只保留 `animate-surface-in`，去掉装饰性 orb 背景动画。

## Edge Cases
- long text: 说明文案限制在合理宽度，避免超长行。
- slow network: preview route 不受网络影响。
- empty datasets: `currentBreak=null` 时仍可显示 fallback。
- permission denied: N/A
- offline: N/A

## Acceptance Criteria (UI)
- break 页不再出现“小窗口里再套一个小卡片”的结构。
- 倒计时、标题和按钮显著更大、更靠近 break window 本身。
- preview break route 可以直接拿来做 UI 验证。

## Open Questions
- 如果你还觉得微休息太小，下一轮就不是调布局，而是直接继续增大 gentle/balanced 的宿主尺寸。
