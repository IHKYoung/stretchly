# Task-ID: TID-20260403-macos-tray-menu-crash

## Goals
- 修复 macOS 菜单栏 tray icon 右键后的原生菜单闪退。
- 保持用户看到的菜单分组、动作命名和左键行为不变。

## Non-Goals
- 不重设计 tray 菜单视觉。
- 不修改 Tauri 主设置页或 break prompt。

## Screens & User Flows
- Primary flow: 用户在 macOS 顶部菜单栏对 Pauza tray icon 执行右键，弹出原生菜单并可看到状态、focus/pause/skip 子菜单与基础动作项。
- Fallback / secondary flow: 用户左键 tray icon 时仍只显示主设置窗口，不弹菜单。
- User-visible boundary: 仅限 `apps/desktop` 的 macOS tray 原生菜单。
- Entrypoints / handoff cues: 菜单栏图标是唯一入口；弹出菜单中的 `Open`、`Resume`、`Reset`、`Quit` 等项沿用现有文案和分组。

## Component Tree
- Root context menu（macOS 使用原生 `Submenu` 作为根）
- `status` / `status-detail` 禁用信息项
- `skip` / `focus` / `pause` 子菜单
- `resume` / `reset` / `hide` / `toggle-autostart` / `quit` 动作项

## Interaction States
- hover: 由 AppKit 原生菜单高亮负责
- active: 右键展开菜单，点击项后触发对应 tray action
- focus: 原生系统菜单处理，无自定义焦点样式
- disabled: `status` 与 `status-detail` 仅展示信息，不可点击
- loading: 不适用，tray menu 为同步原生菜单
- empty: 不适用，菜单至少保留状态项与 `Quit`
- error: 右键时不应出现闪退或进程退出
- skeleton: 不适用
- optimistic (if applicable): 不适用

## Responsive Rules
- 不适用；macOS 原生菜单由系统负责布局和尺寸约束

## Accessibility (a11y)
- keyboard navigation: 依赖 macOS 原生菜单键盘导航
- focus order: 依赖系统菜单顺序，与当前信息架构一致
- aria labels: 不适用；原生 AppKit menu item 使用系统语义
- contrast: 使用系统菜单主题和对比度
- reduced motion: 无自定义动画

## Design Tokens / Tailwind Mapping
- typography: 不适用；原生菜单
- spacing: 不适用；原生菜单
- color usage: 不适用；原生菜单
- key classes: 无

## Micro-animations (optional)
- 无；使用系统原生菜单展开动画

## Edge Cases
- long text: 状态文本较长时由系统菜单自行截断
- slow network: 不适用
- empty datasets: 不适用
- permission denied: 若自动化脚本缺少 assistive access，只影响测试采集，不影响产品运行
- offline: 不适用

## Acceptance Criteria (UI)
- 右键 tray icon 可稳定展开原生菜单，不出现闪退
- 展开的菜单仍包含状态信息、focus/pause/skip 子菜单和基础动作项
- 左键 tray icon 仍只显示主窗口

## Open Questions
- 无；等待用户手工验证 tray 右键
