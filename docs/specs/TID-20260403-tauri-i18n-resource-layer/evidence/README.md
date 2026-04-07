# Task-ID: TID-20260403-tauri-i18n-resource-layer

本轮未新增独立 UI 证据文件。原因：

- 任务核心是将多语言管理从内联文案重构为共享 locale 资源层，而不是修改页面结构或交互流。
- 已通过 `npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`、`cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` 验证前后端调用点与资源接线。
- 正在运行的 `desktop:dev` 已完成热更新并成功重新编译。

若后续继续扩展多语言 UI 流程或欢迎页，再补充截图/录屏证据。
