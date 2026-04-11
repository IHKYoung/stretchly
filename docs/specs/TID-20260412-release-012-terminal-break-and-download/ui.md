# Task-ID: TID-20260412-release-012-terminal-break-and-download

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 将桌面端 break 页从普通静态文案升级为更接近官网的终端打字观感，但不破坏现有居中布局。
- 让官网只保留首页一个下载入口，点击后直接进入真实下载，不再出现中转页目录感。

## Non-Goals
- 不重做 break 页整体信息架构，不改倒计时、按钮文案和主色体系。
- 不在官网新增额外下载页面、版本页或安装引导页。

## Screens & User Flows
- Primary flow: 桌面端进入 break 窗口后，顶部先看到居中的 `Pauza>` prompt，正文按终端打字效果逐字出现；微休息停留单条文案，长休息在整句打完后停留 `30s` 再切换下一条。
- Fallback / secondary flow: 若用户开启 `prefers-reduced-motion` 或 break 处于 `manualAwaiting`，正文直接完整显示，不强制逐字动画。
- User-visible boundary: 仅影响桌面端 break 提示区域和官网首页下载按钮，不改设置页结构与 break CTA 行为。
- Entrypoints / handoff cues: `?window=break` preview、实际 break 窗口、官网首页主下载按钮。

## Component Tree
- `BreakWindow`
- `TypewriterPrompt`
- `rotateBreakPromptEntries`
- 官网首页 `downloadButton` + `script.js` 的 direct-download resolver

## Interaction States
- hover: break 页主体无 hover 依赖；官网按钮维持既有 hover 反馈。
- active: 打字进行中显示细光标；整句完成后进入停留态。
- focus: 键盘焦点继续落在 break CTA 区域，不转移到 prompt。
- disabled: 倒计时未结束时，已有不可点击按钮维持原逻辑。
- loading: 无独立 loading 壳；打字首帧即视为进入内容展示。
- empty: 若 prompt 池为空，沿用既有兜底文案，不让页面空白。
- error: GitHub latest release API 失败时，官网按钮直接回退到固定稳定链接。
- skeleton: 无。
- optimistic (if applicable): 无。

## Responsive Rules
- break 页文案区维持当前居中列宽，prompt 标头与正文都围绕中轴，不回退成左靠齐大段文本。
- 官网仍保持单页响应式布局，下载按钮不新增第二层路由。

## Accessibility (a11y)
- keyboard navigation: break 页键盘路径不变，CTA 顺序保持原样；官网按钮继续作为单一主操作。
- focus order: prompt 区域只读，不抢焦点。
- aria labels: 未新增额外可交互控件，不额外引入 aria 需求。
- contrast: 终端 prompt 与正文保持现有深浅对比，不弱化时间与按钮可读性。
- reduced motion: `prefers-reduced-motion` 下直接显示完整文本，不做逐字动画。

## Design Tokens / Tailwind Mapping
- typography: 延续 break 页现有 mono/terminal 气质，不引入新的字体体系。
- spacing: prompt 与正文之间保持紧凑，但要给时间和 CTA 留出呼吸感。
- color usage: 维持 break 页既有暖白背景与深色文字，终端 prompt 只作为语义锚点。
- key classes: `break-terminal-prompt`、`break-terminal-prompt-mark`、`break-terminal-output`、caret animation helpers。

## Micro-animations (optional)
- 逐字打字动画速度整体放慢一档，细光标闪烁保留；长休息在整句完成后增加 `30s` 停留，再切下一个 prompt。

## Edge Cases
- long text: 超长提示仍走既有换行/布局约束，避免把居中块拉成一整行。
- slow network: 官网不依赖页面级中转，只有点击下载时才请求 GitHub API。
- empty datasets: prompt 池为空时走兜底文案。
- permission denied: 无额外权限请求。
- offline: 官网无法访问 GitHub API 时，直接落到固定稳定下载链接。

## Acceptance Criteria (UI)
- break 页视觉上是居中的终端输出，而不是左对齐长段文字。
- 微休息只打一条 prompt；长休息在完整显示后停留 `30s` 再轮播。
- 官网下载按钮点击后直接进入下载，不再进入 `/download/*` 中转页面。

## Open Questions
- 无；本轮按用户确认的“居中终端打字效果”收口。
