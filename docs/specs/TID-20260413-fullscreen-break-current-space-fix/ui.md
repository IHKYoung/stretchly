# Task-ID: TID-20260413-fullscreen-break-current-space-fix

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。

## Goals
- 让 break 到点后能重新覆盖用户当前正在看的 fullscreen Space。
- 保持 break 页面本身的视觉内容、CTA 和窗口样式不变，只修复“看不见”的宿主浮出问题。

## Non-Goals
- 不重做 break prompt 的视觉排版、文案或按钮层级。
- 不改设置页、tray 菜单或调度规则。

## Screens & User Flows
- Primary flow: 用户正在浏览器、编辑器等全屏应用里工作时，Pauza 到点后会直接浮到当前 fullscreen Space，用户无需去别的屏幕找休息提示。
- Fallback / secondary flow: 在非全屏场景或普通桌面场景下，break 仍沿用当前 windowed/fullscreen 设置与已有样式展示。
- User-visible boundary: 仅 break window 的宿主出现位置与当前 Space 可见性。
- Entrypoints / handoff cues: 自动到点 break；当前工作屏被 Pauza 宿主覆盖或浮出。

## Component Tree
- React 组件树不变；改动仅发生在 Tauri shell 的宿主窗口层。

## Interaction States
- hover: unchanged
- active: break 到点后，当前 fullscreen Space 可直接看到 break prompt
- focus: strict / focusable break 继续沿用现有 focus 语义；non-focusable break 仍由宿主激活链路兜底
- disabled: N/A
- loading: N/A
- empty: N/A
- error: 若 native patch 失败，仍沿用现有错误模型；本轮不新增 UI 错误态
- skeleton: N/A
- optimistic (if applicable): N/A

## Responsive Rules
- 不新增断点或宿主尺寸规则；只修复当前 Space 的可见性，不改 break 页布局比例。

## Accessibility (a11y)
- keyboard navigation: 保持现有 break CTA 键盘操作与焦点语义
- focus order: 不改
- aria labels: 不改
- contrast: 不改
- reduced motion: 不改

## Design Tokens / Tailwind Mapping
- typography: unchanged
- spacing: unchanged
- color usage: unchanged
- key classes: `App.tsx` break prompt 容器类不变，仍由宿主窗口负责可见性

## Micro-animations (optional)
- unchanged

## Edge Cases
- long text: unchanged
- slow network: N/A（本地桌面端）
- empty datasets: N/A
- permission denied: 若系统级 fullscreen Space 行为无法在当前终端环境自动取证，本轮仍以代码路径和本机手动 spot-check 收口
- offline: unchanged

## Acceptance Criteria (UI)
- 当前正在使用的 fullscreen Space 不再“无提示”。
- break 不再只在其它屏幕或后台 Space 自己开始。
- 修复不改变 break prompt 本身的视觉内容与交互语义。

## Open Questions
- 无。
