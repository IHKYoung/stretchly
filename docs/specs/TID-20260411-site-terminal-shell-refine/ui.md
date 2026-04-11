# Task-ID: TID-20260411-site-terminal-shell-refine

## Goals
- 去掉首页中央舞台的卡片壳，让页面更像直接印在纸面上的终端输出。
- 把 `Pauza>` 提示头放到左上角，强化“终端 prompt + 中央输出”的层级。

## Non-Goals
- 不改打字机节奏、文案池、下载按钮逻辑或互动脚本。
- 不把首页改成左对齐的长文本终端页。

## Screens & User Flows
- Primary flow:
  - 打开首页
  - 左上角先看到 `Pauza>`
  - 中央继续看到居中的打字机文案
  - 右上角下载按钮和底部互动提示保持可用
- Fallback / secondary flow:
  - 移动端与 reduced-motion 下维持相同结构，只保留更克制的动效存在感
- User-visible boundary:
  - `apps/site` 首页首屏
- Entrypoints / handoff cues:
  - 左上角 prompt、中央主文案、底部互动提示、右上角下载按钮

## Component Tree
- `download-button`
- `interaction-layer`
- `typewriter-stage`
- `prompt-line`
- `terminal-output`
- `typewriter-copy`
- `interaction-hint`

## Interaction States
- hover:
  - 下载按钮轻微抬升
  - 互动层 aura / 粒子仍存在
- active:
  - 点击后继续出现 burst 粒子与短句气泡
- focus:
  - 舞台仍可聚焦并通过键盘触发提醒
- disabled:
  - N/A
- loading:
  - N/A
- empty:
  - 文案池为空时回退默认句子
- error:
  - 首页结构调整不应引入 console error
- skeleton:
  - N/A
- optimistic (if applicable):
  - N/A

## Responsive Rules
- 桌面端：`Pauza>` 位于舞台宽度左上角，主文案继续视觉居中。
- 移动端：提示头仍保留左上对齐，但不能挤压主文案首屏空间。

## Accessibility (a11y)
- keyboard navigation:
  - 下载按钮和舞台焦点顺序保持不变
- focus order:
  - 下载按钮 -> 舞台
- aria labels:
  - 沿用现有舞台说明与下载按钮 aria
- contrast:
  - 黑灰文字对纯白背景保持清晰可读
- reduced motion:
  - 仅弱化动态反馈，不影响无卡片结构

## Design Tokens / Tailwind Mapping
- typography:
  - `Pauza>` 使用 monospace prompt 风格
  - 主文案继续使用 `LXGW WenKai Screen`
- spacing:
  - 顶部 prompt 与中央输出拆成上下两层
- color usage:
  - 保留白纸、灰线和黑灰文字
- key classes:
  - `typewriter-stage`
  - `prompt-line`
  - `terminal-output`

## Micro-animations (optional)
- 保留当前 pointer aura、粒子与点击短句
- 不再强调卡片本体的浮起感

## Edge Cases
- long text:
  - 主文案长句仍需居中换行
- slow network:
  - 字体未加载时结构仍成立
- empty datasets:
  - 回退默认文案
- permission denied:
  - N/A
- offline:
  - 不受影响

## Acceptance Criteria (UI)
- 首页不再像一张悬浮卡片，而更像纸面上的终端输出。
- `Pauza>` 位于左上角，主文案仍然稳定居中。
- 既有交互反馈与下载入口保持可用。

## Open Questions
- 无；按当前极简终端感执行。
