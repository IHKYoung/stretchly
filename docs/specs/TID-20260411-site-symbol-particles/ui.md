# Task-ID: TID-20260411-site-symbol-particles

## Goals
- 让左上角 `Pauza>` 更像真正的终端 prompt。
- 让交互粒子从抽象光点变成更有程序员/写作者语境的字符。
- 让主输出文案宽度明确收敛到浏览器宽度 `80%`。

## Non-Goals
- 不改动首页整体结构和信息层级。
- 不把首页改成满屏左对齐的代码编辑器。

## Screens & User Flows
- Primary flow:
  - 打开首页
  - 左上角先看到更醒目的 `Pauza>`
  - 中央主文案以 80% 页面宽度展示
  - 点击或移动时看到由符号构成的粒子
- Fallback / secondary flow:
  - reduced-motion 下保留 prompt 强度和输出宽度，只弱化粒子动态
- User-visible boundary:
  - `apps/site` 首页首屏
- Entrypoints / handoff cues:
  - 左上 prompt、中央主文案、交互粒子、右上下载按钮

## Component Tree
- `download-button`
- `interaction-layer`
- `typewriter-stage`
- `prompt-line`
- `prompt-mark`
- `terminal-output`
- `typewriter-copy`
- `interaction-hint`

## Interaction States
- hover:
  - prompt 本身常驻更醒目，不依赖 hover
  - 粒子层继续响应移动
- active:
  - 点击后出现字符粒子和短句提醒
- focus:
  - 舞台仍可键盘触发提醒
- disabled:
  - N/A
- loading:
  - N/A
- empty:
  - 继续回退默认文案
- error:
  - 不应出现 console error
- skeleton:
  - N/A
- optimistic (if applicable):
  - N/A

## Responsive Rules
- 桌面端：输出区宽度等于页面宽度 `80%`，prompt 位于左上角。
- 移动端：输出区仍按 `80vw` 收敛，保持单屏和居中秩序。

## Accessibility (a11y)
- keyboard navigation:
  - 下载按钮和舞台焦点顺序不变
- focus order:
  - 下载按钮 -> 舞台
- aria labels:
  - 沿用现有 aria，不新增噪音标签
- contrast:
  - prompt 和主文案都保持高对比
- reduced motion:
  - 仅弱化粒子动态，不改变结构和宽度

## Design Tokens / Tailwind Mapping
- typography:
  - prompt 使用更重的 monospace
  - 主文案继续使用 `LXGW WenKai Screen`
- spacing:
  - 保持现有终端层级，不新增额外容器
- color usage:
  - prompt 更深，粒子使用冷灰蓝字符色
- key classes:
  - `prompt-line`
  - `prompt-mark`
  - `typewriter-copy`
  - `particle`

## Micro-animations (optional)
- 保留 aura / ripple / nudge bubble
- 粒子改成字符飞散，而不是光点扩散

## Edge Cases
- long text:
  - 宽输出区下仍要保持居中换行
- slow network:
  - 字体回退不应影响 prompt 层级
- empty datasets:
  - 默认文案兜底
- permission denied:
  - N/A
- offline:
  - 不受影响

## Acceptance Criteria (UI)
- `Pauza>` 的终端存在感更强。
- 主输出区宽度等于页面宽度 `80%`。
- 粒子反馈显得更像代码/写作语境，而不是抽象发光点。

## Open Questions
- 无。
