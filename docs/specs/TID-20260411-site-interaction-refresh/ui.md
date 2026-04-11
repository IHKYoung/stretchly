# Task-ID: TID-20260411-site-interaction-refresh

## Goals
- 让首页的打字机舞台真正成为视觉中心，而不是“居中容器里偏左的一段字”。
- 增加轻量、有趣、带一点调侃感的互动，让页面更像 Pauza 本身的语气。

## Non-Goals
- 不增加额外版块，不引入功能式 dashboard。
- 不让交互反馈压过主文案和下载按钮。

## Screens & User Flows
- Primary flow:
  - 打开首页
  - 看到居中的打字机舞台和右上角下载按钮
  - 文案逐字打印并在完整显示后停留约 10 秒
  - 鼠标移动时获得轻微光晕/粒子反馈
  - 点击空白处或舞台时出现一条调侃提醒
- Fallback / secondary flow:
  - reduced-motion 下保留中心布局和静态文案，弱化持续动态效果
- User-visible boundary:
  - 仅首页首屏
- Entrypoints / handoff cues:
  - 中央舞台、下载按钮、舞台下方轻提示

## Component Tree
- `download-button`
- `interaction-layer`
- `typewriter-stage`
- `prompt-line`
- `typewriter-copy`
- `interaction-hint`

## Interaction States
- hover:
  - 下载按钮轻微抬升
  - 舞台获得极轻的空间感和 pointer aura
- active:
  - 点击后出现 burst 粒子和一句调侃提醒气泡
- focus:
  - 下载按钮维持清晰 focus ring
  - 舞台可聚焦并通过键盘触发提醒
- disabled:
  - N/A
- loading:
  - N/A
- empty:
  - 文案池为空时回退到默认句子
- error:
  - 控制台不应报错；下载未配置不影响首页
- skeleton:
  - N/A
- optimistic (if applicable):
  - N/A

## Responsive Rules
- 桌面端：舞台宽度约占视口 80%，文案块居中且保持舒展留白。
- 移动端：舞台宽度放宽到接近屏宽，交互反馈保留但密度下降，仍保持单屏。

## Accessibility (a11y)
- keyboard navigation:
  - 下载按钮可 Tab 聚焦
  - 舞台可通过 Enter / Space 触发提醒
- focus order:
  - 下载按钮 -> 舞台
- aria labels:
  - 舞台需提供“点击或按键触发提醒”的语义说明
- contrast:
  - 主文案与背景保持高对比，辅助提示维持次一级但可读
- reduced motion:
  - 关闭高频粒子和过强动画，只保留最小反馈

## Design Tokens / Tailwind Mapping
- typography:
  - 继续使用 `LXGW WenKai Screen` + monospace prompt
- spacing:
  - 舞台留白增加，文本重心居中
- color usage:
  - 仍以白纸、灰线和少量冷灰/微蓝交互色为主
- key classes:
  - 继续以原生 CSS 为主，不引入新的样式体系

## Micro-animations (optional)
- 打字完成后 10 秒 hold
- pointer aura 轻微跟随
- click burst 粒子扩散
- nudge bubble 浮现并自动淡出

## Edge Cases
- long text:
  - 长句需自动换行且仍保持居中视觉
- slow network:
  - 本地字体未加载时仍可回退到 serif，不影响结构
- empty datasets:
  - 回退默认文案
- permission denied:
  - N/A
- offline:
  - 不受影响

## Acceptance Criteria (UI)
- 打字机舞台在桌面端明显居中，主文案不再偏左。
- 用户移动或点击页面时能感知到轻量反馈，但页面整体仍然安静、克制。
- 点击产生的提醒短句具有调侃感，且不会长期停留遮挡舞台。

## Open Questions
- 无；按当前单页极简方向执行。
