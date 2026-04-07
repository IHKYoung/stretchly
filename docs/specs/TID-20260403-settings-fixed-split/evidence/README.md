# Task-ID: TID-20260403-settings-fixed-split

## Evidence Summary
- Scope: 验证设置页已从全局三栏回到稳定双栏 split view，并确认主窗口交付态围绕 `16:9` 最小尺寸设计；同时确认圆角与介绍文案明显收紧。
- Capture date: 2026-04-03
- Runtime: 浏览器 preview（Vite 本地 `http://127.0.0.1:43179/`）
- Viewport: `1440x810`

## Artifacts
- Screenshot: `settings-fixed-split-1440x810-final.png`
- DOM snapshot: `settings-fixed-snapshot-1440x810.md`
- Console log: `browser-console.log`
- Supplemental screenshots: `settings-fixed-split.png`、`settings-fixed-split-1440x810.png`、`settings-fixed-split-1440x810-trimmed.png`

## What The Evidence Shows
- 左侧 sidebar 单独占据稳定列宽，视觉上接近主窗口宽度的四分之一。
- 右侧主体占据剩余主要空间，save/status dock 已并回主体顶部，不再作为全局第三列存在。
- `概览` 分类激活态、`已同步` 状态徽标和保存 dock 在同一主体流中可同时感知。
- hero 标题已收回为普通分类标题，不再使用口号式介绍；按钮、卡片和表单控件的圆角也明显小于前一版。

## Coverage Notes
- Primary flow: 已覆盖 `sidebar -> main body -> save dock`
- Fallback flow: 桌面主窗口通过 `tauri.conf.json` 的 `minWidth=1280` 与 `minHeight=720` 避免进入窄屏堆叠，本 evidence 不再为窄屏桌面态单独截图
- Visible states: 已覆盖 `category-active`、`save-synced`
- Known gap: 未单独构造 error/loading 人工场景；本轮关注点是固定分栏、窗口尺寸和视觉收敛
