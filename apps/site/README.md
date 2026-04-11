# Pauza Site

`apps/site/` 是 Pauza 的单页官网目录。

当前版本采用极简打字机式单页设计：

- 纯白纸面 / 网格纸背景
- 中央只保留一块提醒文案展示区
- 右上角保留单一下载按钮
- 文案循环展示来自 App 内嵌提醒文案的精选句子

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

首页下载按钮不会直接写死第三方下载地址，而是先进入站内稳定路由：

- `/download/macos-apple-silicon/`
- `/download/macos-intel/`
- `/download/releases/`

真实目标 URL 统一维护在：

```text
apps/site/download/targets.js
```

后续如果从 GitHub Releases 换到 R2 / S3，只需要修改这个文件。

首页循环展示的官网文案维护在：

```text
apps/site/copy.js
```

站点自带字体资源位于：

```text
apps/site/fonts/LXGWWenKaiScreen.ttf
```

## 部署建议

- 推荐直接把 Vercel 的 root directory 指到 `apps/site/`
- 当前站点是纯静态 HTML/CSS/JS，不依赖额外构建工具
