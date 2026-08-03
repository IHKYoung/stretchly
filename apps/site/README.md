# Pauza Site

`apps/site/` 是 Pauza 的单页官网目录。

当前版本采用极简打字机式单页设计：

- 纯白纸面 / 网格纸背景
- 中央只保留一块提醒文案展示区，桌面端宽度约占视口 80%
- 舞台本身不再带卡片边框、底色和阴影，而是直接贴在纸面上
- 左上角单独保留一个更醒目的 `Pauza>` 终端提示头
- 右上角保留单一下载按钮
- 主输出文案宽度按当前浏览器页面宽度的 `80%` 收敛
- 文案循环展示来自 App 内嵌提醒文案的精选句子，单条完整显示后停留约 10 秒
- 全站统一使用 `LXGW WenKai Screen`
- 鼠标移动 / 滚动 / 点击会触发轻量光晕，以及由 `0 / 1 / # / @ / ！ / ¥ / $` 为主的更大、更密的符号粒子和调侃提醒气泡

## 本地预览

在仓库根目录执行：

```bash
npm run site:dev
```

然后访问：

```text
http://127.0.0.1:43210
```

## 下载链接配置

首页只有一个下载按钮，直接请求真实下载链接。下载目标配置维护在：

```text
apps/site/download/targets.js
```

当前按钮会优先请求 GitHub Releases API，解析 latest release 中名称以 `_aarch64.dmg` 结尾的资产，再拼接 `releases/latest/download/<asset-name>` 下载路径；只有 API 不可用或响应中没有匹配资产时，才保留 `targets.js` 里配置的固定稳定链接。

后续如果从 GitHub Releases 换到 R2 / S3，只需要修改这个文件。

## Release 一键发布

站点内置了一个可复用发布脚本：

```bash
python3 scripts/publish_site_release.py \
  --asset /Users/changkunyang/CKProjects/Pauza/apps/desktop/src-tauri/target/release/bundle/dmg/Pauza_0.1.4_aarch64.dmg
```

它会自动：

- 从 dmg 文件名解析版本号
- 更新 `index.html` 与 `download/targets.js` 里的 pinned 下载链接
- 使用 `gh` 创建或更新 GitHub Release

完整说明见：

```text
apps/site/docs/ReleasePlaybook.md
```

首页循环展示的官网文案维护在：

```text
apps/site/copy.js
```

其中包含两类文案池：

- `PAUZA_SITE_LINES`：打字机长文案
- `PAUZA_SITE_NUDGES`：点击后弹出的短句提醒

站点自带字体资源位于：

```text
apps/site/fonts/LXGWWenKaiScreen.ttf
```

## 部署建议

- 推荐直接把 Vercel 的 root directory 指到 `apps/site/`
- 当前站点是纯静态 HTML/CSS/JS，不依赖额外构建工具
