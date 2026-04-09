# Task-ID: TID-20260409-break-surface-i18n-reduction

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- break prompt 保持“单句交互语 + 大号倒计时 + 细进度条”的纯净形态，只增强材质质感。
- 设置页相关分组改成用户可直接理解的中文表达，不露专业术语或语言混杂。
- 自定义壁纸预览在设置页完整展示图片主体，而不是拉伸或裁切。

## Non-Goals
- 不重新引入左右分栏、圆环倒计时、标签化提示或 cue 卡片。
- 不新增新的 break 操作按钮或额外页面。

## Screens & User Flows
- Primary flow: 用户在设置页调整休息窗口背景、声音与语言，保存后进入 break window，看到更轻、更通透的卡片与倒计时。
- Fallback / secondary flow: 浏览器 preview 下也能看到相同 break 表面；未上传壁纸时预览区域显示空状态提示而非拉伸占位图。
- User-visible boundary: 设置页“休息窗口”“休息音效”“语言”分组，以及 break window 本体。
- Entrypoints / handoff cues: 设置页的分组标题、上传按钮、音效下拉、语言下拉；break window 的倒计时与按钮区。

## Component Tree
- BreakWindow
  - full-screen background / wallpaper layer
  - translucent glass card
  - prompt headline
  - numeric countdown
  - linear progress bar
  - optional finish-waiting helper text
  - action buttons
- Settings / Break surface section
  - backdrop selector
  - custom wallpaper upload row
  - wallpaper preview card
  - cue toggle
  - current time toggle
- Settings / Break sounds section
  - microbreak start sound
  - microbreak end sound
  - long break start sound
  - long break end sound
  - shared volume slider
- Settings / General section
  - language selector driven by locale config

## Interaction States
- hover: 上传按钮、下拉框与主操作按钮保持现有轻量 hover，不增加戏剧性动画。
- active: break 按钮点击态维持当前按钮系统反馈。
- focus: 所有上传、下拉、按钮、滑杆继续使用现有 focus ring。
- disabled: 保存中或 busy 时，上传/移除壁纸、动作按钮与部分选择控件按当前禁用态降透明。
- loading: settings bootstrap 与 preview 仍沿用现有 loading 文案。
- empty: 未上传壁纸时显示明确中文说明；无当前休息时 break preview 显示已结束/空态。
- error: 加载失败继续走现有错误提示，但不露 key 或原字段名。
- skeleton: N/A
- optimistic (if applicable):

## Responsive Rules
- break card 在移动宽度与窄窗下保持单列居中，最大宽度不超过当前设计上限。
- 设置页壁纸预览采用固定高宽比容器，内部图片 `object-contain`，保证横图和竖图都能完整展示。
- window 模式宿主最小显示比例按 16:9 收敛，避免再次退化成 4:3 感的矮窗。

## Accessibility (a11y)
- keyboard navigation: 上传按钮、语言/声音下拉、滑杆、break 操作按钮均可键盘访问。
- focus order: 保持 DOM 顺序，从分组标题后的主控件开始顺序流动。
- aria labels: 音效、语言、壁纸相关控件继续带明确 aria-label。
- contrast: 玻璃卡片透明度提高后，正文与数字仍需保持深色文本对比度。
- reduced motion:

## Design Tokens / Tailwind Mapping
- typography: break headline 与 countdown 继续使用高对比大号字体，不追加副标题噪音。
- spacing: 卡片内边距略大于当前设置卡片，维持纯净留白。
- color usage: 提高玻璃层通透感，但保留深色文本与柔和阴影，避免“半透明到发灰”。
- key classes: `rounded-[36px]` 级别大圆角、较轻 `bg-white/*`、更高 `backdrop-blur-*`、预览图片 `object-contain`。

## Micro-animations (optional)
- 仅保留现有倒计时条宽度过渡与按钮系统过渡，不新增新的表演性动画。

## Edge Cases
- long text: 多语言文案变长时仍以单列布局自然换行，不挤爆卡片。
- slow network: N/A（本轮为本地桌面端）
- empty datasets: 缺少自定义壁纸时给出清晰空状态说明。
- permission denied: 文件选择取消或本地图片处理失败时，不污染当前已保存壁纸。
- offline: N/A

## Acceptance Criteria (UI)
- break 卡片相较当前更通透，但仍保持纯净、克制的单列构图。
- 设置页中文文案不出现 `break window`、`prompt`、字段 key 或中英文拼盘。
- 自定义壁纸预览完整显示图片，不裁切主体。
- 音效设置包含开始/结束两个时点的选择项。
- 语言选择来源于语言配置而非手写两个 option。

## Open Questions
- 若未来要把所有 legacy 语言完整暴露到新桌面端，需要补齐桌面端新增文案的本地化覆盖；本轮先完成结构抽离与暴露边界定义。
