# Task-ID: TID-20260407-microbreak-break-labels

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 让当前界面里两类休息的叫法更符合用户直觉，并保持成对、对称。

## Non-Goals
- 不改变设置页布局和控件结构。
- 不改 tray 菜单层级、break prompt CTA 或视觉风格。
- 不把内部字段同步重命名到界面之外的实现层。

## Screens & User Flows
- Primary flow: 用户进入主设置页“节奏”分类，直接看到“微休息”和“休息”两张分区卡，以及下方“提前提醒 / 延后”里的对应标签。
- Fallback / secondary flow: 用户通过 tray 的“跳到”子菜单或 break runtime 标题识别同样的术语，不需要重新理解两类休息是什么。
- User-visible boundary: `apps/desktop` 主设置页、tray skip label、runtime break title/kind、legacy app 文案、README 与软件中心截图 caption。
- Entrypoints / handoff cues: 设置页导航、分区标题、开关 aria label、tray 菜单项、break prompt 标题和 README 功能描述。

## Component Tree
- `App`
- `renderActiveCategory() -> schedule`
- shared desktop locale map
- runtime break locale strings
- tray locale strings
- legacy app locale resources

## Interaction States
- hover: 不涉及新增 hover 行为；沿用现有控件样式。
- active: 不涉及新增 active 行为；只替换标签文本。
- focus: 改名后焦点态仍由原控件承担，aria label 与可见文案保持一致。
- disabled: 开关关闭时，“微休息 / 休息”标签仍应清晰可读。
- loading: N/A
- empty: N/A
- error: N/A
- skeleton: N/A
- optimistic (if applicable): N/A

## Responsive Rules
- “微休息”与“休息”的宽度变化不应破坏当前紧凑设置页的左右对齐。
- 英文 `Microbreak / Break` 在按钮、标题和说明里不应造成额外换行回归。

## Accessibility (a11y)
- keyboard navigation: 不改控件键盘路径。
- focus order: 不改设置页既有 focus 顺序。
- aria labels: 开关和分组朗读内容应直接使用“微休息 / 休息”，避免出现界面与读屏术语不一致。
- contrast: 不涉及色彩变化。
- reduced motion: 不涉及动画变化。

## Design Tokens / Tailwind Mapping
- typography: 沿用当前设置页与 break surface 的既有排版。
- spacing: 沿用当前分区与标签间距，不因换词额外加结构。
- color usage: 不变。
- key classes: 不变。

## Micro-animations (optional)
- 无。

## Edge Cases
- long text:
- 旧中文“长休息 / 小憩”和新中文“微休息 / 休息”长度不同，需确保分区标题和开关对齐不失衡。
- slow network: N/A
- empty datasets: N/A
- permission denied: N/A
- offline: N/A

## Acceptance Criteria (UI)
- 当前界面里的两类休息在中文统一显示为“微休息 / 休息”。
- 当前界面里的两类休息在英文统一显示为 `Microbreak / Break`。
- 用户不会在同一活跃界面里同时看到新旧两套术语混用。

## Open Questions
- 无。
