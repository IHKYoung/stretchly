# Task-ID: TID-20260411-break-copy-line-layout

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 让 break prompt 中央文案维持“按整句阅读”的节奏，而不是被浏览器从中间生硬折断。

## Non-Goals
- 不改 CTA、倒计时、背景主题、字号体系或宿主窗口尺寸。

## Screens & User Flows
- Primary flow: break 弹出后，中央文案按整句优先显示；过长句再按逗号/分号拆成多行，每行独立居中。
- Fallback / secondary flow: 英文等有空格语言在多句场景下保持一句一行；超长单句才做分句拆行。
- User-visible boundary: `?window=break` 的中央文案区。
- Entrypoints / handoff cues: break 弹窗出现后的主文案区域。

## Component Tree
- `BreakWindow`
- `promptCopy.titleLines[]`
- `promptCopy.bodyLines[]`

## Interaction States
- hover: unchanged
- active: unchanged
- focus: unchanged
- disabled: unchanged
- loading: unchanged
- empty: `title/body` 为空时不渲染对应行容器
- error: unchanged
- skeleton: unchanged
- optimistic (if applicable): N/A

## Responsive Rules
- 文案区最大宽度由 `600px` 放宽到 `720px` 上限，减少被动折行压力。
- 标题行数达到 4 行及以上时，主字号会自动收紧一档，避免超长 prompt 占满屏幕。

## Accessibility (a11y)
- keyboard navigation: unchanged
- focus order: unchanged
- aria labels: unchanged
- contrast: unchanged
- reduced motion: unchanged

## Design Tokens / Tailwind Mapping
- typography: 沿用 `type-break`
- spacing: 文案行之间使用轻量 `mt-[0.28em]` / `mt-[0.18em]`
- color usage: unchanged
- key classes: `text-balance`, `max-w-[min(90vw,720px)]`, `text-[clamp(...)]`

## Micro-animations (optional)
- unchanged

## Edge Cases
- long text: 整句过长时降级为按分句换行，并在超多行时收紧标题字号
- slow network: N/A
- empty datasets: 回退到既有 default prompt
- permission denied: N/A
- offline: unchanged

## Acceptance Criteria (UI)
- 当前 break 文案不再在任意字之间生硬断开。
- 中文长句会优先按逗号/句号形成清晰的独立行。
- browser preview 中可直接观察到多行文案的节奏更接近“抄写/摘句”而不是段落自动换行。

## Open Questions
- 无。
