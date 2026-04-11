# Evidence Report · TID-20260411-site-landing-page

## Scope
- Task-ID: `TID-20260411-site-landing-page`
- Evidence date: `2026-04-11`
- Coverage:
  - 官网首页首屏视觉与 CTA
  - 移动端首屏适配
  - 下载稳定路由在未配置真实地址时的 fallback 状态

## Environment
- Local server: `npm run site:dev`
- Preview URL: `http://127.0.0.1:43210/`
- Browser check: Playwright MCP

## Artifacts
- `landing-desktop.png`：桌面端首页首屏，验证“一屏讲清产品 + CTA 可见 + 信息层级成立”
- `landing-mobile.png`：移动端首页首屏，验证纵向堆叠和 CTA 可点击尺寸
- `download-fallback-mobile.png`：下载跳转页 fallback，验证 `targets.js` 为空时页面不会死链或空白

## Reality Check
- 首页桌面端（`1440x980`）：
  - 页面高度与视口一致，可在一屏内看到产品定位、能力摘要和下载入口
  - 控制台 `0 error / 0 warning`
- 首页移动端（`390x844`）：
  - 页面改为纵向堆叠，首屏保留品牌、核心文案与下载按钮
  - CTA 高度和间距满足触控点击
- 下载路由：
  - `/download/macos-apple-silicon/`
  - `/download/macos-intel/`
  - `/download/releases/`
  - 三条路由均可打开并进入统一 redirect/fallback 逻辑
  - 当 `apps/site/download/targets.js` 保持空值时，控制台出现预期 `console.warn`，页面明确提示“下载地址尚未配置”

## Notes
- 当前 evidence 以“站点结构、视觉方向、交互边界、fallback 完整性”为主。
- 真实下载地址填入后，建议补一次 redirect success 状态截图，确认自动跳转与手动入口都正常。
