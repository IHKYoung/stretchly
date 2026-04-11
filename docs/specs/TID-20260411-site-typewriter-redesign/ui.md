# Task-ID: TID-20260411-site-typewriter-redesign

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 用一个中央文案舞台就让用户感受到 Pauza 的语气和产品气质。
- 让首页看起来像一张干净的草稿纸 / 网格纸，而不是常见 SaaS landing page。

## Non-Goals
- 不做 feature list、截图墙、FAQ、长页面滚动或多按钮 CTA。
- 不在首页同时展示多平台下载选择。

## Screens & User Flows
- Primary flow: 用户打开首页，直接看到打字机式提醒文案，理解 Pauza 的语气和定位，然后点击右上角下载按钮。
- Fallback / secondary flow: 用户点击下载按钮后进入既有 `/download/releases/` 路由；若目标地址未配置，则由下载页给出 fallback 提示。
- User-visible boundary: `apps/site/index.html` 与 `/download/releases/`
- Entrypoints / handoff cues: 打字机主文案、右上角下载按钮、下载页 fallback 文案

## Component Tree
- paper background
- floating download button
- central typewriter stage
- prompt line
- typed message output + caret

## Interaction States
- hover: 下载按钮轻微抬升
- active: 下载按钮轻微压缩
- focus: 下载按钮有明确 focus ring
- disabled: N/A（首页按钮始终可点，异常由下载页接管）
- loading: 打字机正在逐字输入
- empty: 首次加载时等待脚本写入第一条文案
- error: 下载地址未配置时由下载页显示 fallback
- skeleton: N/A
- optimistic (if applicable): N/A

## Responsive Rules
- Desktop: 中央舞台宽而低，整体保持一屏纯净留白
- Mobile: 依然只保留中央舞台和右上角下载按钮；允许文本在舞台内多行换行，但不引入额外信息块

## Accessibility (a11y)
- keyboard navigation: 进入页面后可直接聚焦右上角下载按钮
- focus order: 下载按钮 -> 页面其余静态内容
- aria labels: 下载按钮保留明确 `aria-label`
- contrast: 文字与纸面背景维持高对比
- reduced motion: 命中 `prefers-reduced-motion` 时停用打字机循环，静态展示首条文案

## Design Tokens / Tailwind Mapping
- typography: 主文案使用 `LXGW WenKai Screen`，辅助信息使用单宽字体
- spacing: 页面大量留白，中央舞台内部以 `16 / 24 / 40 / 64` 为主
- color usage: 白底、浅灰网格、深灰文字，不引入饱和品牌色块
- key classes: N/A（本轮不用 Tailwind）

## Micro-animations (optional)
- 逐字输入
- 停留约 5 秒
- 逐字退格
- 光标闪烁
- 下载按钮轻微 hover / active 反馈

## Edge Cases
- long text: 文案需要在中央舞台内自然换行，不能溢出容器
- slow network: 字体未加载完时允许先用回退字体显示，不阻塞结构
- empty datasets: 若 `copy.js` 为空，脚本需回退到默认句子
- permission denied: 若下载页无法自动跳转，既有 fallback 按钮继续可用
- offline: 首页仍可浏览；下载页离线时继续显示已存在的 fallback 状态

## Acceptance Criteria (UI)
- 首页视觉方向变成极简、纯净、纸面感强的单舞台设计。
- 打字机文案成为页面绝对主角，不再被其他信息块分散注意力。
- 右上角下载按钮始终清晰可见，但不抢主文案风头。

## Open Questions
- 若后续要加入英文站点或多语言官网，再决定是否为首页文案切换不同语言版本；本轮保持中文提醒文案。
