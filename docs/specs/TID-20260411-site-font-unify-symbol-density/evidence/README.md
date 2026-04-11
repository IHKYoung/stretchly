# Task-ID: TID-20260411-site-font-unify-symbol-density

## Evidence Summary
- Scope: 全站统一 `LXGW` 字体、`Pauza>` 同字体、粒子密度与目标字符覆盖
- Environment: 本地静态服务器 `http://127.0.0.1:43210/`
- Date: 2026-04-11

## Artifacts
- `dom-snapshot.md`：浏览器 DOM 快照
- `console.log`：浏览器控制台摘要

## Findings
- 字体统一：Playwright 读取到 `body / prompt / button / hint / particle` 的 `font-family` 全部为 `\"LXGW WenKai Screen\", \"Kaiti SC\", STKaiti, serif`。
- `Pauza>` 同字体：`promptFont` 与 `bodyFont` 完全一致，确认不再走另一套 monospace。
- 输出区宽度：`.typewriter-copy` `outputWidth = 1152`、`viewportWidth = 1440`，`outputWidthRatio = 0.8`。
- 粒子密度：点击后读取到 `particleCount = 23`，明显高于上一版。
- 目标字符覆盖：纯点击 burst 中稳定采样到 `0 / 1 / # / @ / ！ / ¥ / $` 全集，例如 `[\"1\",\"0\",\"1\",\"#\",\"0\",\"1\",\"#\",\"@\",\"！\",\"¥\",\"$\",...]`。
- Console: `Total messages: 0 (Errors: 0, Warnings: 0)`。

## Reality Notes
- 本轮只统一字体和提高粒子密度，不影响打字机 `10s hold`、下载按钮和提醒气泡逻辑。
