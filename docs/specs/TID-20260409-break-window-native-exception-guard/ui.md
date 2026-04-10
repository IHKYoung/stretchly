# Task-ID: TID-20260409-break-window-native-exception-guard

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 保持现有 break 页面视觉与 CTA 不变，只修复进入 break 前的宿主窗口崩溃。

## Non-Goals
- 不修改 break 页面布局、字体、背景、文案或按钮行为。

## Screens & User Flows
- Primary flow: 用户正常工作时进入 break，宿主窗口先显示，再尝试 macOS native patch；即便 patch 抛异常，应用也不应崩溃。
- Fallback / secondary flow: native patch 失败时，break 退回默认 Tauri 宿主显示路径，应用继续存活。
- User-visible boundary: 仅影响 break 页面出现前后的宿主行为，不影响 break 页面内部 UI。
- Entrypoints / handoff cues: 现有 break 到点弹出流程。

## Component Tree
- N/A（无前台组件树变化）

## Interaction States
- hover: unchanged
- active: unchanged
- focus: 仍由现有 break 宿主窗口与 CTA 逻辑控制
- disabled: unchanged
- loading: unchanged
- empty: N/A
- error: macOS native patch 失败时应降级为 no-op，不再表现为应用直接 abort
- skeleton: N/A
- optimistic (if applicable): N/A

## Responsive Rules
- break 页面现有 fullscreen / windowed 响应规则保持不变。

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
- permission denied: 若系统拒绝或 native patch 抛异常，break 应退回默认宿主路径而不是终止进程
- offline: unchanged

## Acceptance Criteria (UI)
- 进入 break 前不再因为 macOS 原生异常直接退出应用。
- break 页面内部视觉和交互不因本轮修复产生额外回归。

## Open Questions
- 仍需用户在本机分别验证一次 windowed / fullscreen break 进入流程。
