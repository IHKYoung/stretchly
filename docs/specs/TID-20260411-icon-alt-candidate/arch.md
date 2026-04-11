# Task-ID: TID-20260411-icon-alt-candidate

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 为网页端现有图标建立一版透明底、开口朝西北 45 度附近的备选几何。
- 将该版本作为桌面端 `icons` 目录下的不接线候选资产保存，避免影响当前正式打包图标链路。

## Non-Goals
- 不替换当前正式 `icon.png` / `icon.icns` / `icon.ico`
- 不修改图标生成脚本或 Tauri 配置

## Constraints & Assumptions
- 透明底是必须条件。
- 新增候选图标文件不能影响当前打包流程。
- 候选 PNG 需要保留 alpha 通道，便于直接做视觉对比。

## System Boundaries
- Modules:
  - `apps/site/favicon.svg`
  - `apps/desktop/src-tauri/icons/icon-alt-nw45.svg`
  - `apps/desktop/src-tauri/icons/icon-alt-nw45.png`
- Ownership:
  - 网页 favicon 负责站点图标显示
  - 桌面端 `icon-alt-nw45.*` 只作为候选素材，不参与当前正式打包
- Dependency direction:
  - `favicon.svg` -> `icon-alt-nw45.svg` -> `icon-alt-nw45.png`

## API / Contract
- Signatures / Endpoints: N/A（静态素材）
- Request/Response schema (typed): N/A
- Error model (codes, retryability): N/A

## Data Model / Storage
- N/A（静态矢量/位图素材）

## Invariants
- 候选图标必须保持透明底。
- 开口朝向应稳定落在左上方向，而不是继续沿用原先更偏上的朝口。
- 当前正式打包图标文件名与配置不变。

## Concurrency / Lifecycle / Memory Model
- N/A

## Observability Plan (Debug-Driven)
- Logs: N/A
- Metrics: N/A
- Traces: 通过 `sips` / `identify` 校验尺寸和 alpha；通过本地图片预览做目视检查
- Debug flags: N/A

## Security & Privacy Considerations
- 无网络、无密钥、无用户数据

## Risks & Rollback
- Failure modes:
- 导出的 PNG 丢失透明通道
- 开口角度与预期不符
- Rollback steps:
  - 回退 `apps/site/favicon.svg`
  - 删除 `icon-alt-nw45.svg` 与 `icon-alt-nw45.png`

## Acceptance Criteria (System)
- `apps/site/favicon.svg` 变为透明底
- 桌面端 `icons` 目录下存在可直接预览的候选 `svg/png`
- PNG 为 `1024x1024` 且 `hasAlpha: yes`

## Open Questions / Decision Requests
- 当前仅保存为候选素材；是否替换正式图标，留待后续单独决策。
