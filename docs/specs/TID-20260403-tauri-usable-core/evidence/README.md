# Evidence Report · TID-20260403-tauri-usable-core

## Scope
- 记录 Tauri 可用核心闭环这一轮的前台结构证据与浏览器 preview 控制台输出。

## Artifacts
- `settings-snapshot.md`：主设置页结构快照，证明前台已从迁移展示页收敛为极简工具设置页。
- `break-snapshot.md`：`?window=break` 结构快照，证明 break prompt 已独立成小窗卡片。
- `browser-console.log`：浏览器 preview 控制台输出留档。

## What was verified
- 主窗口顶部只保留状态、下一次休息和少量快捷动作。
- 主内容收敛为 `Rhythm / Behavior / Signals / Apps` 四组设置。
- 右侧仅保留状态和 last action，不再展示迁移说明。
- break prompt 只保留标题、说明、倒计时和 Done / Postpone / Skip。

## Notes
- 本轮 Playwright 截图在当前页面字体加载阶段超时，因此保留结构快照而不是 PNG；对这类桌面工具页面，结构快照足以证明信息层级和交互骨架。
- 原生 Tauri 窗口已通过 `npm run desktop:dev` 启动到 `target/debug/pauza-desktop`，但桌面窗口本身未做自动化截图。
