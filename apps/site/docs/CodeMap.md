# CodeMap

> 维护规则：该文件描述当前仓库文件树与职责，确保与实际目录一致。

## Top Level
- `.githooks/`: pre-commit / commit-msg / post-commit / post-merge hooks，负责 workflow 门禁与提交审计。
- `docs/`: 仓库规范、每日计划/日志、spec 模板、任务包与变更记录。
- `docs/ReleasePlaybook.md`: 站点 release 发布手册与一键脚本说明。
- `download/`: 下载目标配置，当前包含官网下载按钮的 release 路由。
- `fonts/`: 官网字体资源。
- `scripts/`: workflow bootstrap、校验、任务脚手架、commit audit 与站点 release 发布脚本。
- `README.md`: 站点说明、本地预览方式与下载配置说明。
- `copy.js`: 首页循环文案与气泡提醒文案。
- `favicon.svg`: 站点 favicon。
- `index.html`: 单页官网入口与下载按钮 DOM。
- `.gitignore`: 忽略 Python 缓存、Playwright 工件与系统临时文件。
- `script.js`: 打字机动画、互动粒子与下载按钮解析逻辑。
- `styles.css`: 站点样式与响应式规则。
