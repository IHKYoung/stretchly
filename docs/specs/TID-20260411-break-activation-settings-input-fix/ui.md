# Task-ID: TID-20260411-break-activation-settings-input-fix

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 让 break 到点时在当前全屏工作区有明确可见的 Pauza 提示。
- 让设置页数字输入具备正常的“先编辑草稿，再提交”的桌面输入体验。

## Non-Goals
- 不改变 settings 页的整体布局、分组、文案或 break prompt 的视觉样式。

## Screens & User Flows
- Primary flow: 用户正在其它应用的全屏工作区时，break 到点后当前 Space 会出现 Pauza break prompt。
- Fallback / secondary flow: 用户在设置页编辑分钟/秒数字时，可以清空、输入多位数，并在 blur / Enter / 加减按钮时提交。
- User-visible boundary: `BreakWindow` 宿主浮出行为；设置页 `CompactNumber` 行内输入。
- Entrypoints / handoff cues: 自动到点 break；设置页中的自然休息/延后/提前提示数字控件。

## Component Tree
- `App`
  - `SettingsRow`
    - `CompactNumber`
  - `BreakWindow`（宿主激活路径不改 React 结构）

## Interaction States
- hover: 加减按钮 hover 样式保持现状
- active: 点击加减按钮立即提交一步
- focus: 输入框聚焦时保持 draft，不立即 clamp
- disabled: 到达 `min/max` 时对应加减按钮禁用
- loading: autosave `busyAction` 维持现状，不新增独立 loading UI
- empty: 输入框允许暂时为空
- error: save error 继续沿用顶部 error 提示
- skeleton: unchanged
- optimistic (if applicable): 输入框本地 draft 属于轻量 optimistic editing，但最终值仍以提交结果为准

## Responsive Rules
- 不新增断点逻辑；仅允许数字输入宽度略放宽以容纳多位数。

## Accessibility (a11y)
- keyboard navigation:
  - 输入框支持 `Enter` 提交、`Escape` 恢复当前值
- focus order:
  - 维持 `SettingsRow` 现有顺序，不新增跳转
- aria labels:
  - 沿用现有 `aria-label`
- contrast:
  - 沿用现有样式，不改色板
- reduced motion:
  - unchanged

## Design Tokens / Tailwind Mapping
- typography:
  - 沿用 settings 行内 13px 数字与 11px suffix
- spacing:
  - 输入框宽度允许从极窄值放宽，避免 3 位数过度拥挤
- color usage:
  - 沿用当前中性玻璃化配色
- key classes:
  - `CompactNumber` 仍使用现有行内按钮/输入排版类，不新增视觉主题

## Micro-animations (optional)
- unchanged

## Edge Cases
- long text:
  - 与本任务无关，unchanged
- slow network:
  - N/A（本地桌面端）
- empty datasets:
  - 数字输入允许空草稿，但提交后恢复为当前或 clamp 后值
- permission denied:
  - N/A
- offline:
  - unchanged

## Acceptance Criteria (UI)
- 当前全屏工作区可以直接看到 break prompt。
- 数字输入不再因为每击键 clamp 而“卡住”。
- `Enter` / `blur` / `Escape` 的交互语义清晰且稳定。

## Open Questions
- 无。
