# Repository Guidelines

## 项目结构与模块组织
- `index.html` / `styles.css` / `script.js`：官网单页入口、样式与交互逻辑。
- `download/targets.js`：下载目标配置；当前维护 macOS Apple Silicon 的 pinned release 与 GitHub latest API。
- `copy.js`：打字机长文案与点击提醒气泡文案池。
- `fonts/` / `favicon.svg`：站点静态资源。
- `docs/`：workflow 计划、日志、spec、changelog 与代码地图。
- `scripts/` / `.githooks/`：workflow 校验、任务脚手架、提交审计与站点 release 发布脚本。

## 构建、测试与开发命令
- 本地静态预览：`python3 -m http.server 43210`
- monorepo 方式预览：在上层仓库根目录执行 `npm run site:dev`
- workflow 文档校验：`python3 scripts/validate_workflow_docs.py --mode manual`
- agent 配置校验：`python3 scripts/validate_agent_configs.py`
- workflow 资产同步校验：`python3 scripts/validate_workflow_kit_sync.py`
- 站点 release 一键发布：`python3 scripts/publish_site_release.py --asset /abs/path/Pauza_<version>_aarch64.dmg`
- 提交前应确保下载链接、`docs/logs`、`docs/plans`、`docs/CHANGELOG.md` 同步更新并进入暂存区
