# Evidence Report · TID-20260411-site-typewriter-redesign

## Scope
- Task-ID: `TID-20260411-site-typewriter-redesign`
- Evidence date: `2026-04-11`
- Coverage:
  - 官网首页桌面端极简纸面 / 打字机设计
  - 官网首页移动端适配
  - 首页下载按钮进入既有稳定下载路由
  - 下载目标未配置时的 fallback 状态

## Environment
- Local server: `npm run site:dev`
- Preview URL: `http://127.0.0.1:43210/`
- Browser check: Playwright MCP

## Artifacts
- `home-desktop.png`：桌面端首页，验证白底网格纸质感、单舞台结构与右上角下载按钮
- `home-mobile.png`：移动端首页，验证单舞台布局与无滚动溢出
- `download-fallback.png`：`/download/releases/` 在目标地址未配置时的 fallback 状态

## Reality Check
- 首页桌面端（`1440x980`）：
  - 控制台 `0 error / 0 warning`
  - 页面主视线只落在 `pauza>` 与动态提醒文案，下载按钮保持在右上角
- 首页移动端（`390x844`）：
  - `scrollHeight == innerHeight`
  - 文案在舞台内自然换行，没有额外滚动
- 打字机循环：
  - 第一次读取：`水杯空了？你的身体可不是仙人掌。`
  - 等待 7 秒后第二次读取：`检查一下你的坐姿：背挺直了吗？别像一只虾一样蜷着。`
  - 证明首页文案确实在轮换，而不是静态首句
- 下载稳定路由：
  - 首页按钮进入 `/download/releases/`
  - 当前 `apps/site/download/targets.js` 仍为空时，下载页显示 fallback，并输出预期 `console.warn`

## Notes
- 当前 evidence 主要验证设计方向、交互节奏和下载路由边界。
- 后续接入真实 GitHub 下载地址后，建议补一张 redirect success 状态截图。
