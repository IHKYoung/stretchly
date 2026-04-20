# Changelog

## 2026-04-20
- `site`: Release Playbook 示例资产与标准命令更新到 `Pauza_0.1.3_aarch64.dmg`。
- `site`: 首页 pinned 下载链接与 `download/targets.js` 将随本轮 `0.1.3` release 一起更新。

## 2026-04-12
- `site`: 首页下载按钮与 pinned fallback 更新到 `Pauza_0.1.2_aarch64.dmg`。
- `site`: 下载解析逻辑增加 pinned 资产名优先保护，避免 GitHub latest 仍返回 `0.1.1` 时把按钮回退到旧版本。
- `workflow`: 初始化本仓库的 docs/specs、daily plans/logs、hooks 与校验脚本，补齐后续任务门禁与审计基础。
- `workflow`: 新增 `scripts/publish_site_release.py` 与 `docs/ReleasePlaybook.md`，把站点 pinned 链接更新与 GitHub release 发布收敛成可复用流程。
- `repo`: 新增 `.gitignore`，忽略 Python 缓存与 Playwright 临时目录。
