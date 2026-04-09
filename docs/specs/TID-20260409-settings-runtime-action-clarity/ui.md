# Task-ID: TID-20260409-settings-runtime-action-clarity

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 把运行时动作从“混在顶部的一组按钮”收敛成“当前状态 + 独立节奏控制”的清晰结构。

## Non-Goals
- 不重做设置页整体布局。
- 不引入新的 reminder mode、pause 类型或 tray 行为。

## Screens & User Flows
- Primary flow: 用户打开设置页时，如果当前处于暂停态，顶部出现一个紧凑状态卡，左侧显示当前状态与说明，右侧只出现一个 `恢复提醒` 按钮。
- Fallback / secondary flow: 若当前处于 focus session，顶部状态卡改为 `结束专注`；若处于普通运行态，则顶部不出现恢复动作。
- User-visible boundary: 仅影响 `apps/desktop` 主设置窗口，不改 tray、break prompt 或宿主菜单。
- Entrypoints / handoff cues: 顶部上下文状态卡、`节奏` 分类顶部/上部的独立 `节奏控制` 区。

## Component Tree
- `SettingsRuntimeStatusCard`
  - `status title`
  - `status detail`
  - contextual button: `恢复提醒` or `结束专注`
- `ScheduleActionCard`
  - `重置节奏`
  - 解释文案：只重算节奏，不自动解除暂停
- existing `SchedulePresetCard` groups

## Interaction States
- hover:
- active: 运行中默认不显示 `恢复提醒`
- focus: 所有新增按钮沿用现有 `Button` 组件 focus 样式
- disabled: `重置节奏` 在 busy / strict break lock 等不可执行场景下禁用
- loading:
- empty: N/A
- error: 继续复用设置页顶部的错误条
- skeleton:
- optimistic (if applicable):

## Responsive Rules
- 顶部状态卡在窄宽度下允许纵向堆叠；按钮仍需保持单手可点击尺寸。
- `节奏控制` 卡与既有 preset 卡使用同一宽度和圆角语言。

## Accessibility (a11y)
- keyboard navigation: 状态卡按钮和 `重置节奏` 按钮必须可 Tab 访问
- focus order: 先顶部状态卡，后 `节奏控制`，再进入具体节奏设置
- aria labels: 复用按钮可见文本，不新增模糊 icon-only 控件
- contrast: 延续当前白底/深字对比，不依赖淡色提示区传达核心含义
- reduced motion: 不新增额外动画

## Design Tokens / Tailwind Mapping
- typography: 沿用 12/13px 设置页字级；状态标题不做大号 hero 化
- spacing: 与现有 `SettingsCard` / `SettingsRow` 统一，保持 `space-y-3`
- color usage: 继续使用白色卡片 + 中性色文字；`重置节奏` 用次级按钮而非危险红
- key classes: 复用 `SectionLabel`、`SettingsCard`、`SettingsRow`、`Button`

## Micro-animations (optional)
- N/A

## Edge Cases
- long text:
- slow network: N/A（本轮无网络）
- empty datasets:
- permission denied: N/A
- offline:

## Acceptance Criteria (UI)
- 暂停态顶部只出现 `恢复提醒`
- focus 态不再出现“恢复提醒”误导性命名
- `重置节奏` 独立成区，并有“不自动解除暂停”的说明
- 默认态界面不再出现这两个动作并列混排

## Open Questions
- 若后续要恢复更多运行时动作，应继续放在上下文状态卡或独立动作区，而不是混成“设置项”
