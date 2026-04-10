# Task-ID: TID-20260409-break-window-layout-and-controls-refresh

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 让 break 页面更安静、更聚焦于休息本身，而不是继续保持展示型双栏外观。
- 将可见动作收口到真正允许的 CTA，不在主界面继续暴露 `Skip`。

## Non-Goals
- 不重做 break 页面 copy、宿主窗口尺寸或调度时机。
- 不新增额外设置项或二级工具栏。

## Screens & User Flows
- Primary flow: break 到点后页面以单列方式显示 eyebrow、提示语、数字倒计时和细进度条；主界面只展示当前状态允许的 CTA。
- Fallback / secondary flow: `manualAwaiting` 时主按钮显示 `Resume work`；`canPostpone` 为真时显示 `Later`。
- User-visible boundary: 仅影响 `?window=break`。
- Entrypoints / handoff cues: 现有 break 到点弹出流程。

## Component Tree
- `main > section`
- `eyebrow chip`
- `title`
- `body`
- `countdown`
- `progress meter`
- `manualAwaiting hint`
- `CTA stack`

## Interaction States
- hover: CTA hover 保持现有轻微玻璃态反馈
- active: CTA 只会在允许状态出现
- focus: 仅落在当前可操作按钮上
- disabled: `busyAction` 时按钮继续 disabled
- loading: N/A
- empty: N/A
- error: 若 command 出错，沿用现有前台错误处理
- skeleton: N/A
- optimistic (if applicable): preview transform 继续只模拟允许的动作

## Responsive Rules
- 内容区在桌面 break 窗口内保持居中，文本与 CTA 最大宽度分别受 `max-w-[min(86vw,600px)]` / `max-w-[min(86vw,480px)]` 约束。

## Accessibility (a11y)
- keyboard navigation: 仅需在当前允许的 CTA 间移动
- focus order: 倒计时之后进入 CTA 栈
- aria labels: 沿用现有按钮语义
- contrast: 维持当前主题下的文字与玻璃按钮对比
- reduced motion: 保留既有轻量 countdown 呼吸动画，不新增更重 motion

## Design Tokens / Tailwind Mapping
- typography: `type-break` 作为主标题、正文与倒计时字族
- spacing: 单列布局以 `gap-4` / `gap-5` 控制文本与 CTA 节奏
- color usage: `break-prompt.ts` scene/palette 继续负责 text/meter/action 颜色
- key classes:
  - `max-w-[min(86vw,600px)]`
  - `text-[clamp(1.55rem,3.6vw,2.7rem)]`
  - `text-[clamp(4rem,11vw,7.5rem)]`
  - `h-[3px] w-full rounded-full`
  - `rounded-full`

## Micro-animations (optional)
- 倒计时沿用 `animate-break-breathe`，不再额外堆叠展示型动画。

## Edge Cases
- long text: 正文最大宽度限制在 `40ch` 左右，避免单列段落过宽
- slow network: N/A
- empty datasets: N/A
- permission denied: N/A
- offline: unchanged

## Acceptance Criteria (UI)
- break 页面为单列主视觉，不再保留旧双栏结构。
- 数字倒计时和细进度条成为主视觉中心。
- 主界面不再显示 `Skip`，CTA 只随 `manualAwaiting` / `canPostpone` 变化。

## Open Questions
- 是否需要在未来把 `Skip` 迁到更隐蔽的次级入口，可留待后续任务讨论。
