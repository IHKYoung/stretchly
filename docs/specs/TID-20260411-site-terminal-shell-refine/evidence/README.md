# Task-ID: TID-20260411-site-terminal-shell-refine

## Evidence Summary
- Scope: 官网首页去卡片化、左上角终端提示头、中央输出区稳定性
- Environment: 本地静态服务器 `http://127.0.0.1:43210/`
- Date: 2026-04-11

## Artifacts
- `site-terminal-shell-desktop.png`：桌面端无卡片终端布局
- `site-terminal-shell-mobile.png`：移动端无卡片终端布局
- `console.log`：浏览器控制台摘要

## Findings
- 桌面端结构：`stageBorderWidth = 0px`、`stageBackground = rgba(0, 0, 0, 0)`、`stageBoxShadow = none`，确认卡片壳已移除。
- 终端提示头：`promptText = Pauza>`，且 `promptLeftOffset = 0`，说明提示头已贴到舞台左上角。
- 中央输出区：桌面端 `outputCenter = 719.996`，`viewportCenter = 720`，主文案仍保持几乎完全居中。
- 移动端稳定性：`scrollHeight = innerHeight = 844`，仍为单屏；`promptLeftOffset = 0`，左上提示头在小屏下仍成立。
- Console: `Total messages: 0 (Errors: 0, Warnings: 0)`。

## Reality Notes
- 本轮只改结构与样式，不影响既有打字机节奏、互动粒子和下载按钮行为。
