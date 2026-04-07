# Task-ID: TID-20260403-settings-sidebar-layout

## Evidence Summary
- Artifact: `settings-sidebar-layout-desktop.png`
- Captured on: 2026-04-03
- Source: 浏览器 preview（`http://127.0.0.1:43179/`）

## What It Verifies
- 设置页已切换为左侧分类导航、中间当前分类、右侧 save rail 的三栏结构。
- 当前截图中激活的是 `节奏` 分类，说明 sidebar category 切换后主内容区会切到对应分类。
- 右侧保存按钮在修改数值后已从禁用切为可用，证明 `dirty -> save enabled` handoff 仍然存在。

## Supporting Runtime Checks
- Playwright 检查 `保存` 按钮在修改前为 disabled、修改后为 enabled。
- Playwright console error 计数为 0。
