# Task-ID: TID-20260411-site-interaction-refresh

## Evidence Summary
- Scope: 官网首页打字机停留时长、居中舞台布局、移动/滚动/点击交互反馈、移动端稳定性
- Environment: 本地静态服务器 `http://127.0.0.1:43210/`
- Date: 2026-04-11

## Artifacts
- `site-home-desktop.png`：桌面端首页静态状态
- `site-home-interaction.png`：桌面端点击后出现粒子与调侃提醒气泡
- `site-home-mobile.png`：移动端首页单屏状态
- `console.log`：浏览器控制台摘要

## Findings
- AC1 / 10 秒停留：通过 Playwright 重新载入首页后观察首条文案，完整显示后 `5.5s` 仍保持原句，`10.7s` 后已进入删除/切换流程，符合 `10s hold` 预期。
- AC2 / 居中与宽度：桌面端 `innerWidth = 1470`、`stageWidth = 1176`，主舞台宽度约等于视口 `80%`；`stageCenter = viewportCenter = 735`，水平居中成立。
- AC3 / 交互反馈：桌面端移动鼠标、滚动和点击后，DOM 中出现 `particle = 17`、`ripple = 1`，交互层正常响应。
- AC4 / 点击调侃提醒：点击舞台后出现 `nudge-bubble = 1`，文案示例为“起来两分钟，别把屁股焊在椅子上。”。
- AC5 / 移动端稳定性：移动端 `scrollHeight = innerHeight = 844`，首页仍保持单屏，无额外滚动溢出。
- Console: `Total messages: 0 (Errors: 0, Warnings: 0)`。

## Reality Notes
- 下载按钮未被互动层拦截，仍维持固定右上角入口。
- `prefers-reduced-motion` 路径通过代码审查确认会关闭高频粒子与舞台倾斜，仅保留最小反馈。
