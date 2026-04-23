# Task-ID: TID-20260423-smart-reminder-v2-scheduling

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 收敛设置页运行时状态与 tray 文本的可见语义，让用户知道 Pauza 是在“等空档”“检测到离开”“按离开时长结算”还是“真正开始休息”。
- 不改布局，只改已有 runtime status / detail / last action 的文案与状态切换。

## Non-Goals
- 不改设置页信息架构、分类或新增显式控件。
- 不改 break window 主视觉、CTA 排布或 tray 菜单结构。

## Screens & User Flows
- Primary flow:
  - 用户正常工作，break due 后若仍在连续输入，设置页状态 / tray 文本显示“等待空档”，并明确最多还会再等多久。
  - 若用户已经离开达到恢复 credit 区间，Pauza 不直接弹 break，而是在返回时自动结算并更新状态文案。
- Fallback / secondary flow:
  - `forced` 模式继续到点直接开始 break。
  - pause / focus / DND / app exclusion 进入后，状态切为对应 blocker；解除后不再显示“节奏已重置”，而是恢复冻结前的剩余节奏。
- User-visible boundary:
  - 设置页顶部 runtime status 卡
  - tray tooltip / 状态文本
  - break window 是否弹出以及弹出时机
- Entrypoints / handoff cues:
  - settings `DesktopSnapshot.status/statusDetail`
  - tray 刷新后的状态文本
  - `last_action` 作为用户可感知的最近动作说明

## Component Tree
- 不新增组件。
- 继续复用：
  - 设置页 runtime status 行
  - tray status / tooltip
  - 现有 break prompt 开窗链路

## Interaction States
- hover:
  - N/A（无新增 hover 交互）
- active:
  - `heads-up`
  - `waiting for opportunity`
  - `recovery hold`
  - `natural break reset`
- focus:
  - N/A（不改焦点顺序）
- disabled:
  - forced break 下的既有锁定语义保持不变
- loading:
  - N/A
- empty:
  - 至少保留原有 `idle / no break scheduled` 语义
- error:
  - N/A（无新增错误 UI）
- skeleton:
  - N/A
- optimistic (if applicable):
  - N/A

## Responsive Rules
- 不新增布局变化；所有可见变化都通过现有状态文本适配桌面端 settings / tray。

## Accessibility (a11y)
- keyboard navigation:
  - 不改键盘路径
- focus order:
  - 不改
- aria labels:
  - 不新增
- contrast:
  - 继续使用现有状态文本区域，不改色板
- reduced motion:
  - 不新增动画

## Design Tokens / Tailwind Mapping
- typography:
  - 沿用现有 runtime status 文字层级
- spacing:
  - 不改
- color usage:
  - 不改
- key classes:
  - 不改前台 class，仅更新 host 文本输出

## Micro-animations (optional)
- N/A

## Edge Cases
- long text:
  - 新增文案需控制在 tray 与设置页状态区可读范围内，避免过长。
- slow network:
  - N/A
- empty datasets:
  - 所有 break disabled 时保持现有 idle 状态
- permission denied:
  - N/A
- offline:
  - N/A

## Acceptance Criteria (UI)
- 用户在 smart mode 下能从状态文案区分“单纯等空档”和“已离开，回来后会自动结算”。
- 等待空档状态要能表达“最多还会再等多久”，避免提醒看起来像随机失效。
- pause/focus/DND/app exclusion 解除后，状态不再误导为“整轮节奏已重置”，除非确实命中 full natural break reset。

## Open Questions
- 无
