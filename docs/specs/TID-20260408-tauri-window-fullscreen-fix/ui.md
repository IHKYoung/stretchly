# Task-ID: TID-20260408-tauri-window-fullscreen-fix

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 让主设置窗口保持最低可用尺寸
- 让 windowed break 不再缩成右下角小浮窗，而是居中的大窗口
- 让 fullscreen break 在 macOS 上真正无标题栏铺满，不再出现顶部空白

## Non-Goals
- 不改 break 页面视觉布局、文案、CTA 排列
- 不改 settings 分类和表单内容

## Screens & User Flows
- Primary flow: break 到点后在 window 模式下弹出居中的大窗口，约占当前工作区 `80% x 80%`
- Fallback / secondary flow: break 到点后进入 fullscreen，屏幕顶部不再残留标题栏式空白；用户拖拽主设置窗口缩小时，在 `800x600` 停止
- User-visible boundary: 仅宿主窗口壳层，不改页面内容
- Entrypoints / handoff cues: windowed/fullscreen break 自动弹出；主设置窗口人工 resize

## Component Tree
- React 组件树不变；变更仅发生在窗口宿主层（Tauri shell）

## Interaction States
- hover: N/A
- active: windowed break centered / fullscreen break visible
- focus: strict/fullscreen break 仍保持 focusable
- disabled: N/A
- loading: N/A
- empty: N/A
- error: 若 fullscreen helper 失败，仍按现有错误模型返回 invoke error
- skeleton: N/A
- optimistic (if applicable): N/A

## Responsive Rules
- 主设置窗口不再允许缩到 `800x600` 以下
- break 页继续直接填满宿主窗口；window mode 下宿主窗口统一居中，并约占当前工作区 `80% x 80%`

## Accessibility (a11y)
- keyboard navigation:
- focus order: 不变
- aria labels: 不变
- contrast: 不变
- reduced motion: 不变

## Design Tokens / Tailwind Mapping
- typography:
- spacing: 不变
- color usage: 不变
- key classes: `App.tsx` break layout不改，仍沿用 `min-h-screen` 填充宿主窗口

## Micro-animations (optional)
- 不新增

## Edge Cases
- long text:
- slow network: N/A
- empty datasets: N/A
- permission denied: 若无法做系统级截图，本轮仍可通过代码路径和构建验证收口
- offline: N/A

## Acceptance Criteria (UI)
- windowed break 不再缩在右下角，而是稳定居中
- windowed break 宿主窗口约占工作区 `80% x 80%`
- fullscreen break 顶部无额外空白区域
- 主设置窗口不能再缩小到 `800x600` 以下
- fullscreen 关闭时不留下残余黑壳或错误标题栏状态

## Open Questions
- 是否需要后续补一份 macOS 本机截图到 evidence 目录，作为最终视觉旁证
