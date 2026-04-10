# Task-ID: TID-20260409-tray-live-countdown

## Goals
- 让用户在顶部 tray 直接看到真实流动的剩余时间，而不是近似静止的分钟桶。

## Non-Goals
- 不改变 tray 菜单结构。
- 不调整 break 页面 UI。

## Screens & User Flows
- Primary flow:
  - 用户在正常运行、当前 break、focus 或定时 pause 状态下查看 menubar/tray，顶部文本应持续变化。
- Fallback / secondary flow:
  - 在 DND、app exclusion、natural break 等阻塞态，不显示误导性 countdown。
- User-visible boundary:
  - 仅影响 tray 顶部文本和 tooltip。
- Entrypoints / handoff cues:
  - 系统 menubar / tray icon。

## Component Tree
- 宿主 tray icon
- tray 标题文本
- tray tooltip
- 既有原生菜单

## Interaction States
- hover:
  - tooltip 显示当前状态标题和详情。
- active:
  - 点击图标仍只负责打开设置窗口，不改行为。
- focus:
  - 焦点不涉及网页组件。
- disabled:
  - 阻塞态隐藏 schedule countdown。
- loading:
  - N/A
- empty:
  - 无有效 countdown 时标题清空，仅保留图标。
- error:
  - N/A
- skeleton:
  - N/A
- optimistic (if applicable):
  - N/A

## Responsive Rules
- N/A（宿主 tray 不走前端响应式布局）

## Accessibility (a11y)
- keyboard navigation:
  - 不改现有菜单键盘行为。
- focus order:
  - 不涉及网页焦点顺序。
- aria labels:
  - N/A
- contrast:
  - 沿用系统 tray 文本渲染。
- reduced motion:
  - 倒计时为文本刷新，不引入动画。

## Design Tokens / Tailwind Mapping
- N/A（宿主层原生 UI）

## Micro-animations (optional)
- N/A

## Edge Cases
- long text:
  - 只显示 `m:ss / h:mm:ss` 数字文本，避免占满 menubar。
- slow network:
  - N/A
- empty datasets:
  - 无倒计时时标题为空。
- permission denied:
  - N/A
- offline:
  - N/A

## Acceptance Criteria (UI)
- 顶部 tray 文本为秒级倒计时，不再呈现“卡住”的体验。
- 阻塞态不会显示错误倒计时。
- tooltip 能补足状态语义。

## Open Questions
- N/A
