# Task-ID: TID-20260410-pre-break-heads-up-restoration

## Goals
- 让“提前提示”在用户可见层面重新成立，而不是只存在于设置字段和后台通知分支里。
- 让设置语义与实际体验一致：它描述的是 heads-up cue，不是承诺一定弹系统通知。

## Non-Goals
- 不新增 break 页面上的额外提示条或新按钮
- 不重做设置页布局

## Screens & User Flows
- Primary flow:
- 用户设置了某类 break 的提前提示后，在 break 到点前的 lead time 内，设置页状态卡和 tray 文本会切换到 `即将开始 / Up next`
- Fallback / secondary flow:
- 若系统通知未显示，heads-up 仍通过状态卡、tray tooltip 和菜单文本可见；到点后继续进入 `等待空档` 或正式 break
- User-visible boundary:
- 仅影响 break 之前的提示语义和状态文本
- Entrypoints / handoff cues:
- 设置页 `提前提示`
- tray 标题 / tooltip / 菜单状态
- 可选系统通知

## Component Tree
- 设置页：继续复用现有 `SectionLabel` + `SettingsRow`
- runtime 状态：复用现有 `DesktopSnapshot.status/statusDetail`
- tray：复用现有菜单文本与 tooltip 更新链

## Interaction States
- hover:
- 无新增
- active:
- due 前 heads-up
- due 后 waiting / break active
- focus:
- 无新增
- disabled:
- 关闭某类 break 的 heads-up 时不显示该阶段
- loading:
- N/A
- empty:
- N/A
- error:
- 系统通知失败只影响辅助提示，不影响 heads-up 状态本身
- skeleton:
- N/A
- optimistic (if applicable):
- N/A

## Responsive Rules
- 无新增前台布局；继续沿用当前设置页和 tray 结构

## Accessibility (a11y)
- keyboard navigation:
- 无新增键盘路径
- focus order:
- 无变化
- aria labels:
- 无变化
- contrast:
- 仅文案调整，无新色彩表面
- reduced motion:
- 无新增动效

## Design Tokens / Tailwind Mapping
- typography:
- 沿用现有设置页和 tray 文案体系
- spacing:
- 无变化
- color usage:
- 无变化
- key classes:
- 无新增

## Micro-animations (optional)
- 无

## Edge Cases
- long text:
- heads-up 文案必须比 waiting 文案更短、更接近“马上开始”
- slow network:
- N/A
- empty datasets:
- N/A
- permission denied:
- 系统层不显示通知时，仍保留 heads-up 状态
- offline:
- N/A

## Acceptance Criteria (UI)
- 设置页中该分组文案改为“提前提示 / Heads-up cues”
- due 前能看到 `即将开始 / Up next`
- due 后不再继续显示 heads-up，而是进入既有 waiting / active 文案

## Open Questions
- 是否要在后续版本加入更强的 in-app 轻提示；本轮先不做。
