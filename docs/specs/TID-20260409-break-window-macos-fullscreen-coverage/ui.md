# Task-ID: TID-20260409-break-window-macos-fullscreen-coverage

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 保持现有 break 页面 UI 不变，只修正它在 macOS 全屏工作区中的宿主展示层行为。

## Non-Goals
- 不修改 break 页面布局、CTA、字体、背景或动画。

## Screens & User Flows
- Primary flow: 用户正在 macOS 全屏应用里工作，break 到点后，提醒窗口仍能出现在当前工作屏幕/Space 之上。
- Fallback / secondary flow: 非 macOS 或非全屏工作区场景继续沿用当前 break window 行为。
- User-visible boundary: 只影响 break 宿主窗口如何出现，不改变 break 页面内部 UI。
- Entrypoints / handoff cues: 现有 break 到点弹出流程保持不变。

## Component Tree
- N/A（无前台组件树变化）

## Interaction States
- hover: unchanged
- active: unchanged
- focus: 仍由现有 strict / focusable 逻辑决定
- disabled: N/A
- loading: unchanged
- empty: N/A
- error: 若 native patch 失败，退回现有 Tauri window 显示路径
- skeleton: N/A
- optimistic (if applicable): N/A

## Responsive Rules
- break 页面现有的 fullscreen / 16:9 windowed 响应规则保持不变。

## Accessibility (a11y)
- keyboard navigation: unchanged
- focus order: unchanged
- aria labels: unchanged
- contrast: unchanged
- reduced motion: unchanged

## Design Tokens / Tailwind Mapping
- typography: unchanged
- spacing: unchanged
- color usage: unchanged
- key classes: unchanged

## Micro-animations (optional)
- unchanged

## Edge Cases
- long text: unchanged
- slow network: N/A
- empty datasets: N/A
- permission denied: 若系统拒绝窗口前置，则退回当前 break 窗口行为
- offline: unchanged

## Acceptance Criteria (UI)
- macOS 全屏工作区中，break 宿主窗口不会再只停留在后台 Space。
- 现有 break 页面视觉和交互不产生额外回归。

## Open Questions
- N/A
