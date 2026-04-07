# Task-ID: TID-20260404-compact-settings-ui

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 将设置窗口收成更像桌面工具的紧凑偏好页，而不是展示型大面板。
- 在更小窗口里保持四个核心分类的可读性与可操作性。

## Non-Goals
- 不重做 break prompt。
- 不新增设置项或恢复 overview / 快捷键编辑 / 第三列 save rail。

## Screens & User Flows
- Primary flow: 用户打开设置窗口后，先在左侧窄导航选择分类，再在右侧单一主面板内直接调整当前分类设置；设置自动保存，无脏数据时顶部几乎无状态噪音。
- Fallback / secondary flow: 当窗口缩到最小尺寸时，内容区通过滚动保持可达，不再拆出额外栏位。
- User-visible boundary: `apps/desktop` 主设置窗口整体框架、按钮/选择器/数字输入/分段控件样式、默认窗口尺寸。
- Entrypoints / handoff cues: 左侧高亮分类、顶部当前分类标题、保存中/未保存的轻量状态、出错时的提示条。

## Component Tree
- `App`
- `main` 工具窗口容器
- `aside` 窄侧栏
- `SidebarNavButton` × 4
- `section` 主面板
- `SettingsGroup`
- `SettingRow`
- `InlineField`
- `Switch` / `NumberControl` / `Select` / `SegmentedControl`
- 顶部轻量状态区

## Interaction States
- hover: 侧栏项、按钮和分段控件仅做轻量背景变化，不使用夸张发光或浮起效果。
- active: 当前分类通过左侧实线和浅底色标识；当前分段项使用白底和细边线。
- focus: 保留 ring，但缩到工具型控件可接受范围。
- disabled: 自动保存后不再依赖显式保存按钮；开关与输入自身保留原生 disabled 支持。
- loading: N/A（本轮不新增 skeleton）
- empty: N/A
- error: 顶部错误条保留为单行浅红提示。
- skeleton: N/A
- optimistic (if applicable): 沿用现有 dirty/synced 状态切换。

## Responsive Rules
- 默认围绕 `1040x585` 主窗口设计。
- 最小窗口 `960x540` 下保留侧栏与主面板，不切换为移动端布局。
- 设置行在较窄宽度下允许换行，保证控件不溢出。

## Accessibility (a11y)
- keyboard navigation: 侧栏按钮、表单控件与保存操作保持原生键盘可达。
- focus order: 侧栏 -> 主面板标题区 -> 当前分类表单项。
- aria labels: 延续现有 `Switch` / `SegmentedControl` / `Select` 上的 aria label。
- contrast: 主文本使用深蓝黑，辅助文本与边框保持高于此前的淡色卡片风格。
- reduced motion: 本轮不新增新的动效依赖。

## Design Tokens / Tailwind Mapping
- typography: 标题降到 `15px` 左右，正文与控件文案主要使用 `11px~13px`。
- spacing: 外层 `p-2.5`，设置行 `px-4 py-2.5`，导航按钮 `px-3 py-2`。
- color usage: 暖灰背景 `#eee9df` + 白色面板 + 浅灰边框 + 少量深色强调。
- key classes: `max-w-[920px]`、`w-[168px]`、`rounded-[6px]`、`rounded-[7px]`、`border-slate-200`、`bg-[#f4f3ef]`。

## Micro-animations (optional)
- 保持最少；只保留基础的颜色过渡，不做展示型浮动或渐变演出。

## Edge Cases
- long text: 设置项说明允许换行，避免在小窗口里被截断。
- slow network: N/A（本页主要读取本地 Tauri snapshot）
- empty datasets: `appExclusionCommands` 允许为空。
- permission denied: 开机自启动切换仍由后端命令决定，前端保留错误提示条。
- offline: N/A

## Acceptance Criteria (UI)
- 设置页从视觉上收敛为紧凑工具窗口，不再有大卡片、大标题、额外状态栏和装饰背景。
- 左侧窄导航 + 右侧单面板在 `1040x585` 和 `960x540` 约束下仍成立。
- 控件圆角、阴影和背景明显压平，整体观感不再松散。
- 设置改为自动保存，顶部不再需要显式 `保存 / 撤销` 按钮。

## Open Questions
- 下一轮是否继续把 break prompt 一并收成更克制的系统工具风格。
