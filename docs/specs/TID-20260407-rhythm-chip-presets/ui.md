# Task-ID: TID-20260407-rhythm-chip-presets

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 将节奏页的高频时间设置改成可一击完成的预设芯片，减少“输入数字”的认知摩擦。

## Non-Goals
- 不重做整页信息架构。
- 不把提前提醒、延后时间等低频设置也改成芯片。
- 不更改 break window 或其他标签页。

## Screens & User Flows
- Primary flow: 用户进入“节奏”标签页后，在微休息和长休息区块中直接点击 preset 芯片完成核心时间选择，并用区块右侧开关控制是否启用。
- Fallback / secondary flow: 用户在同页下半部分继续使用 stepper 调整提前提醒和延后时间。
- User-visible boundary: 仅 `apps/desktop/src/App.tsx` 的主设置页节奏分类。
- Entrypoints / handoff cues: 顶部分类 pill 切换到“节奏”；区块标题在左，开关在右；每个设置行以标签 + 芯片组 + 单位的方式呈现。

## Component Tree
- `App`
- `renderActiveCategory() -> schedule`
- `SettingsPresetCard`
- `PresetChipRow`
- `PresetChipGroup`
- `CompactNumber`（保留用于提前提醒 / 延后）

## Interaction States
- hover: 未选中芯片 hover 时文字转深、背景略提亮。
- active: 选中芯片为白底、深色文字、细阴影；点击后立即切换当前值。
- focus: 芯片需保留 ring / outline，满足键盘可见焦点。
- disabled: 开关关闭时区块保持可见，避免信息消失；开关自身维持既有禁用/开启视觉。
- loading: N/A
- empty: N/A
- error: 顶部错误条沿用现有全页错误呈现，不为芯片单独设计错误态。
- skeleton: N/A
- optimistic (if applicable): 自动保存沿用现有轻量保存中状态，不新增局部 loading。

## Responsive Rules
- 芯片行允许在主窗口较窄时换行，但标签、芯片组和单位仍需保持明显分区。
- 区块容器保持现有紧凑卡片宽度，不新增横向滚动。
- 在默认 `1040x585` 工具窗口内，微休息和长休息两组内容都应一次可见或仅少量滚动。

## Accessibility (a11y)
- keyboard navigation: 芯片保持 button 语义，可 Tab 聚焦并用 Enter/Space 触发。
- focus order: 先区块开关，再每一行芯片组，最后进入 stepper 区块。
- aria labels: 使用现有标签文案组合成可理解的 aria 信息，不引入仅靠视觉判断的状态。
- contrast: 选中态深色文字对白底，未选中态浅灰底仍保持足够对比。
- reduced motion: 不新增复杂动画，仅保留现有轻量 transition。

## Design Tokens / Tailwind Mapping
- typography: 延续当前 `text-[13px]` 设置标签和 `text-[12px]` 控件字号。
- spacing: 区块内行距控制在 `gap-2` 到 `gap-3`，芯片间距约 `gap-1.5` 或 `gap-2`。
- color usage: 选中芯片使用 `bg-white` + 现有轻阴影；未选中芯片使用 `bg-black/[0.04]` / `text-muted-foreground`。
- key classes: `rounded-lg`、`shadow-[0_1px_2px_rgba(0,0,0,0.06),0_0_0_0.5px_rgba(0,0,0,0.04)]`、`focus-visible:ring-2`、`transition-all`。

## Micro-animations (optional)
- 芯片 hover / active 仅使用 150~200ms 级别的过渡，不引入位移动画。

## Edge Cases
- long text: 中英文标签长度不同，但 preset 数字固定且短，不应撑破布局。
- slow network: N/A（本地设置页）
- empty datasets: N/A
- permission denied: N/A
- offline: N/A

## Acceptance Criteria (UI)
- 节奏页顶部微休息和长休息区块以 preset 芯片呈现核心时间设置。
- 选中态和未选中态视觉差异明确，单击即可完成选择。
- 下方提前提醒与延后区块仍然是 stepper，不与芯片交互混淆。

## Open Questions
- 无；preset 集合由用户明确指定。
