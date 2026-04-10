# Task-ID: TID-20260409-apps-locale-single-source-refactor

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 保持当前 break prompt 的可见文案、按钮和状态语义不变。
- 把这些文案的维护入口统一收口到 `messages/*.json`，避免再出现第二套文案真源。

## Non-Goals
- 不改设置页布局。
- 不改 break prompt 的排版、背景、按钮层级或状态转换。
- 不新增新的 CTA、标签或视觉状态。

## Screens & User Flows
- Primary flow: 用户在 `?window=break` 看到 break prompt；普通状态显示默认/随机提示语和 `Done / Later / Skip`，`manualAwaiting` 状态显示“等待完成”提示并把主 CTA 切到“继续工作”。
- Fallback / secondary flow: 当 `breakIdeasEnabled` 关闭或随机列表为空时，回退到 `ui.breakCopy.defaultPrompt.{kind}`。
- User-visible boundary: 仅 break prompt 的文案来源变化；页面布局、交互状态和设置项入口保持不变。
- Entrypoints / handoff cues: `App.tsx` 的 break window 渲染分支、`breakIdeasEnabled` 开关、当前语言设置。

## Component Tree
- `App.tsx`
  - `BreakWindow`
    - prompt text
    - countdown
    - progress meter
    - `awaitingFinish` hint
    - CTA buttons
- `lib/break-prompt.ts`
  - `pickBreakPrompt()`
    - `tList(language, 'ui.breakCopy.prompts.{kind}')`

## Interaction States
- hover: 保持现有按钮 hover 样式
- active: 保持现有按钮 active 样式
- focus: 保持现有键盘 focus ring 逻辑
- disabled: busyAction 或当前 break 能力受限时，按钮状态保持现状
- loading: N/A（本任务不新增异步加载态）
- empty: `currentBreak = null` 时显示 `ui.breakCopy.clearedDetail`
- error: N/A（本任务不新增新的前台错误态）
- skeleton: N/A
- optimistic (if applicable): 保持现有 preview/runtime action 行为，不新增 optimistic UI

## Responsive Rules
- 保持现有 break prompt 在 window / fullscreen 下的布局与字号规则，不因文案迁移而引入新的断点逻辑。

## Accessibility (a11y)
- keyboard navigation: 保持现有按钮 tab 顺序
- focus order: 主 CTA -> postpone -> skip，保持现状
- aria labels: 复用现有按钮文案，无新增 aria 要求
- contrast: 维持当前 break prompt 的对比度方案
- reduced motion: 保持现有倒计时与进度条动效，不新增额外 motion

## Design Tokens / Tailwind Mapping
- typography: 不变
- spacing: 不变
- color usage: 不变
- key classes: 维持 `BreakWindow` 当前 glass card / meter / button classes，不因为文案源收口做视觉调整

## Micro-animations (optional)
- N/A

## Edge Cases
- long text: `ui.breakCopy.*` 继续通过现有 break prompt 文本容器承接，不新增专门截断逻辑
- slow network: N/A（本任务不引入网络）
- empty datasets: 若随机 prompt 列表为空，则回退到默认 prompt
- permission denied: N/A
- offline: N/A

## Acceptance Criteria (UI)
- break prompt 在普通态、manualAwaiting 态和无 active break 态的可见文案保持语义一致。
- 用户修改 break prompt 文案时，只需要编辑 `apps/desktop/src/locales/messages/*.json`，不再需要找第二个文件。
- 删除 `break-message-copy.*` 后，页面仍能根据当前语言正确显示 break prompt 文案。

## Open Questions
- 无
