# Task-ID: TID-20260411-site-font-unify-symbol-density

## Goals
- 让首页从视觉上彻底统一为一套书写字体。
- 保留终端层级，但不再靠第二套 monospace 制造差异。
- 让粒子更像“码字/写代码现场”的字符喷发。

## Non-Goals
- 不改首页结构、文案池和下载入口。
- 不把首页做成代码编辑器界面。

## Screens & User Flows
- Primary flow:
  - 打开首页
  - `Pauza>`、主文案、下载按钮、互动提示都以同一套字体出现
  - 点击或移动时看到更大更密的 `0 / 1 / # / @ / ！ / ¥ / $` 粒子
- Fallback / secondary flow:
  - reduced-motion 下仍保留统一字体和字符语义，只弱化动态数量感
- User-visible boundary:
  - `apps/site` 首页首屏
- Entrypoints / handoff cues:
  - 左上 `Pauza>`、中央主文案、右上下载按钮、字符粒子

## Component Tree
- `download-button`
- `interaction-layer`
- `typewriter-stage`
- `prompt-line`
- `prompt-mark`
- `terminal-output`
- `typewriter-copy`
- `interaction-hint`
- `particle`

## Interaction States
- hover:
  - 下载按钮维持统一字体
  - 移动时出现字符 trail
- active:
  - 点击后 burst 中稳定出现指定符号集合
- focus:
  - 舞台仍可键盘触发提醒
- disabled:
  - N/A
- loading:
  - N/A
- empty:
  - 默认文案兜底
- error:
  - 不应出现 console error
- skeleton:
  - N/A
- optimistic (if applicable):
  - N/A

## Responsive Rules
- 桌面端：整页统一 LXGW，输出区保持 `80vw`。
- 移动端：整页仍统一 LXGW，输出区保持 `80vw` 并维持单屏。

## Accessibility (a11y)
- keyboard navigation:
  - 下载按钮和舞台焦点顺序不变
- focus order:
  - 下载按钮 -> 舞台
- aria labels:
  - 沿用现有 aria，不额外制造朗读噪音
- contrast:
  - 统一字体后仍保持清晰对比
- reduced motion:
  - 只弱化动态反馈，不改变统一字体和结构

## Design Tokens / Tailwind Mapping
- typography:
  - 全站统一 `LXGW WenKai Screen`
- spacing:
  - 沿用现有终端布局和 80vw 输出区
- color usage:
  - 粒子仍用冷灰蓝系，但通过字符本身增强语义
- key classes:
  - `prompt-line`
  - `download-button`
  - `interaction-hint`
  - `particle`

## Micro-animations (optional)
- 保留 aura / ripple / nudge bubble
- 字符粒子数量、字号和目标字符密度同步上调

## Edge Cases
- long text:
  - 宽输出区下仍需居中换行
- slow network:
  - 字体回退时结构仍要成立，但本轮目标是优先让主加载路径统一 LXGW
- empty datasets:
  - 默认文案兜底
- permission denied:
  - N/A
- offline:
  - 不受影响

## Acceptance Criteria (UI)
- 全站文字看起来是同一种字体。
- `Pauza>` 仍像 prompt，但不再跳出成另一套字体。
- 粒子明显更像字符而不是抽象光点。

## Open Questions
- 无。
