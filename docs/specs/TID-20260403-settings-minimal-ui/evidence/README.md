# Task-ID: TID-20260403-settings-minimal-ui Evidence

## Summary
- 证据目标：证明设置页已从多卡片/多区域结构收敛为极简 split view，并且只保留核心真设置的主入口。
- 采集时间：2026-04-03
- 采集环境：浏览器 preview，窗口 `1440x810`

## Artifacts
- Screenshot: `./settings-minimal-1440x810.png`
- DOM snapshot: `./settings-minimal-snapshot-1440x810.md`
- Console log: `./browser-console.log`

## AC Coverage
- AC1 `1/4 : 3/4` 分栏：由 `settings-minimal-1440x810.png` 可见左侧导航区与右侧单主面板。
- AC2 仅 4 个分类：截图与 DOM snapshot 可见 `节奏 / 提醒与打断 / 智能暂停 / 通用`。
- AC3 去除 overview/quick actions/save rail：截图中不再存在 hero、概览卡片或第三列保存栏。
- AC4 保持可渲染且无明显前端异常：`browser-console.log` 无新增错误，`typecheck/build` 通过。

## Notes
- 本轮证据聚焦主设置页；`?window=break` 路径未重新采集截图，因为本任务未改 break prompt 主体交互。
