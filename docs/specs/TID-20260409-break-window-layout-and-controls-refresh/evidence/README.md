# Task-ID: TID-20260409-break-window-layout-and-controls-refresh Evidence

## Summary
- 证据目标：证明 break 页面已从展示型双栏收口为单列界面，并且主界面不再暴露 `Skip`。
- 采集时间：2026-04-09
- 采集方式：代码路径核对 + desktop build/test + workflow docs validator

## Available Evidence
- 代码路径：
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/lib/break-prompt.ts`
- 验证命令：
  - `npm test`
  - `npm --prefix apps/desktop run build`
  - `python3 scripts/validate_workflow_docs.py --mode manual`

## AC Coverage
- AC1:
  - `App.tsx` 的 break 页面现为单列主内容区，主结构围绕 eyebrow / title / body / countdown / meter / CTA 排列。
- AC2:
  - 倒计时改为数字主视觉，进度改为细条形 meter；`break-prompt.ts` 只负责 scene/palette，不再支撑旧展示型双区块。
- AC3:
  - CTA 区域只按 `manualAwaiting` / `canPostpone` 渲染 `Resume work` / `Later`，主界面不再渲染 `Skip`。
- AC4:
  - `npm test` 通过。
  - `npm --prefix apps/desktop run build` 通过。
  - workflow docs validator 通过。

## Gaps
- 当前没有保留同日截图或录屏资产；本轮证据主要依赖代码路径与构建验证。
- 若后续需要官网或设计回顾素材，应再补一次真实 break 页面截图。
