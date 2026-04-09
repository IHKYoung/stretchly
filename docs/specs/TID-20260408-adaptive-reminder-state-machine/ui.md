# Task-ID: TID-20260408-adaptive-reminder-state-machine

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 从交互语义上消除“我正在打字，你却突然插进来”的体验断裂。
- 让用户能从现有状态文本里理解：提醒不是失效，而是在等一个更合适的空档。

## Non-Goals
- 不改设置页布局、break window 样式或按钮结构。
- 不新增新的设置页表单控件。

## Screens & User Flows
- Primary flow:
  - 倒计时到点时，若用户仍在持续输入，break 不立即弹出。
  - tray/detail/设置页状态摘要切换为“等待空档”语义。
  - 检测到短暂停顿后，break window 再出现。
- Fallback / secondary flow:
  - 若到点时本就空闲，则 break 仍像现在一样立即开始。
  - 若等待期间进入 focus / DND / app exclusion / natural break，则状态切回阻塞语义。
- User-visible boundary:
  - 设置页顶部状态文本
  - tray 状态详情
  - OS notification 文案
  - break window 出现时机
- Entrypoints / handoff cues:
  - 无新增入口；所有变化都通过既有状态栏与 break window 生命周期体现。

## Component Tree
- `App.tsx` 设置页状态摘要：消费 `snapshot.status / statusDetail / lastAction`
- tray 菜单顶部两行状态：消费 `snapshot.status / status_detail`
- break prompt：继续消费 `currentBreak`，视觉结构不变

## Interaction States
- hover: N/A（无新增控件）
- active: `scheduled`、`due-but-protected`、`soft-nudged`、`break-running`
- focus: 沿用现有控件焦点管理，无新增焦点流
- disabled: blocker 生效时 break 不投递
- loading: N/A
- empty: N/A
- error: N/A（不新增前台错误态）
- skeleton: N/A
- optimistic (if applicable): N/A

## Responsive Rules
- 无新增布局变更；桌面窄窗口与 tray 菜单继续复用当前容器。

## Accessibility (a11y)
- keyboard navigation:
- focus order: 不新增焦点项，保持现有顺序。
- aria labels: 无新增控件，无新增 aria 面。
- contrast: 仅新增文本语义，不改颜色 token。
- reduced motion: 不新增动画。

## Design Tokens / Tailwind Mapping
- typography: 沿用现有状态摘要文本样式。
- spacing: 沿用现有容器节奏。
- color usage: 不新增颜色语义。
- key classes: N/A（本轮不改前台样式）

## Micro-animations (optional)
- N/A

## Edge Cases
- long text:
- slow network: N/A（host 本地状态）
- empty datasets: N/A
- permission denied: 不新增权限请求
- offline: N/A

## Acceptance Criteria (UI)
- 用户在持续输入时不会突然看到 break window。
- 用户能从状态文本看出提醒正在等待空档，而不是单纯“卡住”。
- 当等待过久时，只会出现一次温和提醒，不会形成通知轰炸。

## Open Questions
- 如果后续用户仍希望更强控制，可以把 hidden adaptive settings 暴露为“提醒策略”高级项，但不在本轮范围内。
