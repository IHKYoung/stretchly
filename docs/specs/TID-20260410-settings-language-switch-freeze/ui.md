# Task-ID: TID-20260410-settings-language-switch-freeze

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 修复设置页在切换语言时的卡住问题，并保持语言选择的自动保存体验。

## Non-Goals
- 不改设置页布局、控件样式或任何翻译文本。

## Screens & User Flows
- Primary flow:
  - 用户打开设置页 -> `偏好 / 语言` -> 选择一个新 locale -> 下拉正常关闭 -> autosave 成功后，整页文案与 `dir/lang` 再切到新语言。
- Fallback / secondary flow:
  - 若保存失败，语言下拉保留草稿选择，但页面其余文案维持已持久化语言；错误提示继续走现有 save error 路径。
- User-visible boundary:
  - 只调整“何时真正切换可见语言”，不调整可选语言列表和自动保存机制。
- Entrypoints / handoff cues:
  - 设置页 `偏好` 分类中的 `语言` 下拉。

## Component Tree
- `App.tsx` 设置页壳
- `SettingsRow(label=ui.language)` + `Select`
- `document.documentElement/body` 的 `lang` / `dir` 属性同步

## Interaction States
- hover:
  - 维持现有 select hover，不新增样式变化
- active:
  - 选中某个 locale 时，不再立刻让整页 labels 翻译与文档方向跳变
- focus:
  - 下拉选项选择和关闭流程应保持可交互，不被整页 rerender 打断
- disabled:
  - 沿用现有 busy/save 状态
- loading:
  - 初始 snapshot 加载期间仍显示现有 loading 文案
- empty:
  - N/A
- error:
  - 保存失败时沿用现有错误提示；不会把整个页面切到半套新语言
- skeleton:
  - N/A
- optimistic (if applicable):
  - 仅语言下拉的值保留乐观草稿；整页文案不做乐观切换

## Responsive Rules
- 沿用现有设置页响应式规则；本任务不改布局。

## Accessibility (a11y)
- keyboard navigation:
  - 语言下拉继续可用键盘展开、移动和确认
- focus order:
  - 选择 locale 后，焦点不应因整页重翻译而丢失或卡死
- aria labels:
  - 继续使用 `ui.language`
- contrast:
  - 沿用现有样式
- reduced motion:
  - 沿用现有样式

## Design Tokens / Tailwind Mapping
- typography:
  - 沿用现有设置页 token
- spacing:
  - 沿用现有 `SettingsRow` 与 `SelectTrigger`
- color usage:
  - 沿用现有设置页 token
- key classes:
  - `SelectTrigger className="h-7 min-w-[210px] text-[12px]"`

## Micro-animations (optional)
- N/A

## Edge Cases
- long text:
  - 某些 locale 名称较长，但不影响本次行为边界
- slow network:
  - Tauri 本地保存即使稍慢，也只会延后可见语言切换，不应卡住窗口
- empty datasets:
  - N/A
- permission denied:
  - N/A
- offline:
  - N/A

## Acceptance Criteria (UI)
- 语言下拉选择后，设置页不会在下拉关闭阶段卡住。
- 页面 labels 和 `dir/lang` 只在 autosave 成功后切到新语言。
- 当前语言下拉仍即时反映用户刚选中的 locale。

## Open Questions
- 无。
