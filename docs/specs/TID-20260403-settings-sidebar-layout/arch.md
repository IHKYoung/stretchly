# Task-ID: TID-20260403-settings-sidebar-layout

## Architecture Scope
- 本任务不修改 Rust host、Tauri command contract、settings persistence 或调度状态机。
- 仅重排 `apps/desktop/src/App.tsx` 的前台展示层，把已有 `PauzaSettings` 字段按新的单页信息架构组织出来。

## Data / Contract Notes
- 继续消费既有 `DesktopSnapshot` 与 `PauzaSettings`。
- `update_settings`、`toggle_autostart`、`pause_breaks`、`resume_breaks` 等 command 调用面保持不变。
- 侧边栏分类只是视图层状态，不写入持久化设置。

## Boundaries
- In scope:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/styles.css`
  - `apps/desktop/src/locales/{zh-CN,en}.json`
- Out of scope:
  - `apps/desktop/src-tauri/src/*.rs`
  - `app/preferences.html` 本身
  - 新增设置字段、依赖或远程数据源

## Risk Notes
- 主要风险是信息重排后遗漏当前已暴露的设置项，或让保存/状态区的 handoff 不再明显。
- 通过逐项映射当前 `PauzaSettings` 字段和 build + UI 自检来控制风险。
