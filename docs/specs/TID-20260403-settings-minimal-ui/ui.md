# Task-ID: TID-20260403-settings-minimal-ui

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 将设置页改回极简桌面工具式布局，只显示用户真正会调的核心设置。
- 去掉 dashboard、overview、快捷动作与大段解释性文案。
- 让页面在桌面宽度下稳定呈现 `1/4 : 3/4` 的侧栏/主体关系。

## Non-Goals
- 不为低频设置设计新的高级页。
- 不重做 break prompt 视觉语义。
- 不新增统计面板、运行时概览或快捷键录入区。

## Screens & User Flows
- Primary flow: 用户打开设置页，左侧选择 `节奏 / 提醒与打断 / 智能暂停 / 通用`，右侧编辑当前分类设置并保存。
- Fallback / secondary flow: 浏览器 preview 使用模拟状态渲染同一套页面；`?window=break` 继续进入 break prompt。
- User-visible boundary: 仅 `apps/desktop` 设置页主窗口结构与样式发生变化。
- Entrypoints / handoff cues: 左侧导航按钮、右侧分类标题、顶部 `保存 / 撤销`、底部状态摘要。

## Component Tree
- `App`
- `SidebarNavButton[]`
- `Card`（仅右侧主面板）
- `SettingsGroup`
- `SettingRow`
- `Switch`
- `NumberControl`
- `SegmentedControl`
- `Select`
- `Textarea`

## Interaction States
- hover: 仅导航按钮与表单控件提供轻量 hover 反馈。
- active: 当前分类导航使用深色实底高亮。
- focus: 复用现有控件 focus ring，不新增额外动效。
- disabled: 无改动时 `保存 / 撤销` 保持 disabled；preview snapshot 可见保存按钮默认禁用。
- loading: 初次 `snapshot` 未返回时显示 `正在加载 Pauza…`。
- empty: N/A，设置页不是空列表界面。
- error: 顶部红色错误条承载 command/load 失败信息。
- skeleton: 无。
- optimistic (if applicable): preview 模式下保存和 autostart toggle 会更新本地 snapshot 作为即时反馈。

## Responsive Rules
- `xl` 及以上：左侧导航约 `1/4`，右侧主面板约 `3/4`。
- `xl` 以下：退化为单列堆叠，侧栏在上、内容在下。
- Tauri 主窗口默认 `1440x810`，最小 `1280x720`，保证 split view 不塌陷。

## Accessibility (a11y)
- keyboard navigation: 左侧导航按钮、switch、segmented control、select、textarea 和保存按钮都可键盘聚焦。
- focus order: 从左侧导航到右侧内容再到顶部操作，保持线性阅读顺序。
- aria labels: switch 与 segmented control 继续使用现有 `aria-label`。
- contrast: 深色导航激活态与白色主面板保持明确对比。
- reduced motion: 设置页移除装饰性背景动画，不再依赖漂浮光斑。

## Design Tokens / Tailwind Mapping
- typography: 标题保持大号但简短，辅助信息只保留状态和必要说明。
- spacing: 行级设置统一使用 `px-6 py-4`，保留工具型秩序感。
- color usage: 背景改为暖米色，主面板纯白，边框浅灰，激活态使用深蓝黑。
- key classes:
  - split view: `xl:grid-cols-[minmax(260px,1fr)_minmax(0,3fr)]`
  - sidebar: `border-r border-slate-200`
  - main panel: `rounded-[8px] border border-slate-200 bg-white`
  - inline controls: `rounded-[8px|10px] border border-slate-200 bg-slate-50`

## Micro-animations (optional)
- 设置页不再主动使用装饰性动效；只保留控件级过渡。

## Edge Cases
- long text: locale 文案仍需保持短句，否则会挤压单行设置布局。
- slow network: `get_snapshot` 慢时先显示加载态。
- empty datasets: app exclusions 为空时，textarea 允许空字符串。
- permission denied: autostart 命令失败时，错误会落到顶部 error 条。
- offline: N/A，设置页不依赖远程网络。

## Acceptance Criteria (UI)
- 页面不再出现 overview hero、快捷动作区、额外第三列保存栏。
- 左侧只有 4 个分类入口，右侧只有一个主内容面板。
- 截图中能清晰看到“工具面板”而不是“展示卡片墙”。
- 视觉语言较之前更克制：背景更干净、卡片半径更小、装饰性背景消失。

## Open Questions
- 低频高级设置未来是进入二级页面，还是通过“更多设置”独立入口承接，后续再定。
