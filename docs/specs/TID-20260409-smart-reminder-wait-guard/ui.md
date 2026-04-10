# Task-ID: TID-20260409-smart-reminder-wait-guard

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 在设置页“智能暂停”分组直接暴露 idle 阈值，让用户不再只能接受 hidden default。
- 让 smart reminder 相关文案明确表达“会等短空档，但不会无限等待”。

## Non-Goals
- 不重新拆设置页导航信息架构。
- 不新增独立的“最长等待时间”前台控件。

## Screens & User Flows
- Primary flow: 用户进入设置页 `偏好 -> 智能暂停`，直接调整“等待空档”秒数。
- Fallback / secondary flow: 当前语言未翻译新 key 时，沿现有 locale fallback 链回退，不显示 raw key。
- User-visible boundary: 仅影响设置页智能暂停分组和 smart reminder 的状态/提示文案。
- Entrypoints / handoff cues: `SectionLabel = 智能暂停 / Smart pause` 下新增 `CompactNumber` 控件。

## Component Tree
- `App.tsx`
  - `SectionLabel`
  - `SettingsCard`
  - `SettingsRow(label=ui.waitForPause, detail=ui.waitForPauseHint)`
  - `CompactNumber(value=form.idleOpportunitySeconds, suffix=ui.suffix.secondsLong)`

## Interaction States
- hover:
  - `CompactNumber` 的加减按钮沿用现有 hover 样式
- active:
  - 输入框修改后沿现有自动保存链路落到 host
- focus:
  - 数字输入保留原生 focus
- disabled:
  - 无新增 disabled 态
- loading:
  - 无
- empty:
  - 无
- error:
  - 无新增前台错误提示
- skeleton:
  - 无
- optimistic (if applicable):
  - 无；沿用现有自动保存状态提示

## Responsive Rules
- 新控件沿 `SettingsRow` 现有布局，在窄宽度下继续右侧收纳为紧凑 number control。

## Accessibility (a11y)
- keyboard navigation:
  - 数字输入可直接键盘编辑
- focus order:
  - 位于 `智能暂停` 卡片顶部，先于 natural breaks / DND
- aria labels:
  - 复用 `CompactNumber` 原生 number input；label 通过 `SettingsRow` 文案提供上下文
- contrast:
  - 继续沿用设置页现有 token
- reduced motion:
  - 无新增动画

## Design Tokens / Tailwind Mapping
- typography:
  - 沿用设置页 `text-[13px] / text-[12px]`
- spacing:
  - 复用 `SettingsRow` 与 `CompactNumber`
- color usage:
  - 复用当前 neutral 设置页 palette
- key classes:
  - `CompactNumber` 继续使用现有 `rounded-md bg-black/[0.04]`

## Micro-animations (optional)
- 无

## Edge Cases
- long text:
  - locale 文案较长时应仍能在 `SettingsRow.detail` 中自然换行
- slow network:
  - N/A（本地桌面端）
- empty datasets:
  - N/A
- permission denied:
  - N/A
- offline:
  - N/A

## Acceptance Criteria (UI)
- 设置页“智能暂停”分组出现可调的 `等待空档 / Wait for a pause` 秒数控件。
- zh-CN / en 下都不显示 raw locale key。
- smart reminder 相关文案明确表达“不会无限等待”。

## Open Questions
- 无
