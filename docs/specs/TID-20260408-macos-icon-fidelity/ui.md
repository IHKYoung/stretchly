# Task-ID: TID-20260408-macos-icon-fidelity

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 让 macOS 菜单栏中的 Pauza tray icon 更接近系统应用的占位和清晰度预期。
- 让 Dock icon 使用更原生的 app icon 资源表现，不再显得异常放大。

## Non-Goals
- 不重做品牌视觉语言。
- 不修改任何设置页、break prompt 或 tray 菜单的布局与文案。

## Screens & User Flows
- Primary flow: macOS 菜单栏中的 Pauza tray icon 静态显示
- Fallback / secondary flow: Dock 中的 Pauza app icon 静态显示
- User-visible boundary: 仅限原生系统壳层图标，不涉及应用内页面
- Entrypoints / handoff cues: 应用启动后菜单栏与 Dock 的默认图标

## Component Tree
- macOS status bar button image
- macOS Dock application icon image

## Interaction States
- hover: N/A
- active: N/A
- focus: N/A
- disabled: N/A
- loading: 应用初始化时使用同一套图标资源
- empty: N/A
- error: 若 patch 失败，至少保留 Tauri builder 提供的 tray icon，不出现空图标
- skeleton: N/A
- optimistic (if applicable): N/A

## Responsive Rules
- 图标几何按小尺寸画布设计；menu bar 以更保守的 glyph padding 控制视觉占位。

## Accessibility (a11y)
- keyboard navigation: N/A
- focus order: N/A
- aria labels: N/A
- contrast: template image 交由 macOS 自身根据浅色/深色菜单栏着色
- reduced motion: N/A

## Design Tokens / Tailwind Mapping
- typography: N/A
- spacing: tray glyph 外边距收紧为更接近系统图标的安全范围
- color usage: tray icon 保持单色模板图；Dock icon 使用现有品牌黑白配色
- key classes: N/A

## Micro-animations (optional)
- N/A

## Edge Cases
- long text: N/A
- slow network: N/A
- empty datasets: N/A
- permission denied: 未授予 Screen Recording 时，本轮无法采集系统级视觉证据
- offline: N/A

## Acceptance Criteria (UI)
- tray icon 的视觉占位比此前更克制，不再贴满菜单栏图标槽位
- tray icon 走 template image 着色路径，浅/深色菜单栏下都由系统处理着色
- Dock icon 使用 bundled app icon 资源，不再基于原始 `icon.png` 直接覆盖

## Open Questions
- 待用户本机视觉复核：当前 tray glyph 是否还需要继续缩半档
