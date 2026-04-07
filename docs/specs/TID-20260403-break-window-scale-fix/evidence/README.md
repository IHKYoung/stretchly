# Task-ID: TID-20260403-break-window-scale-fix Evidence

## Summary
- 证据目标：证明 break 页已不再是“外层窗口里再套一张小卡片”，并且浏览器 preview 可以直接显示 break UI。
- 采集时间：2026-04-03
- 采集环境：浏览器 preview，`http://127.0.0.1:43179/?window=break`

## Artifacts
- DOM snapshot: `./break-window-preview-snapshot.md`

## AC Coverage
- AC1 / AC3：snapshot 只剩顶部 meta、主体内容和 CTA 三段，不再出现内层 card 容器。
- AC2：preview route 直接显示 `微休息`、倒计时和按钮。
- AC5：配合 `cargo check`、`typecheck`、`build`，证明代码链路完整。

## Gaps
- Playwright screenshot 仍卡在 fonts wait，本轮未保留图片文件。
