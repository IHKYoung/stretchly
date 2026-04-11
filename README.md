# Pauza

当前版本：`0.1.1`

当前仓库当前维护两块内容：

- `apps/desktop/`：Pauza 桌面端主实现
- `apps/site/`：Pauza 单页官网与下载跳转页

根目录脚本默认仍以桌面端为主，网站预览使用单独的 `site:dev` 入口。旧 Electron `app/` 壳已从当前工作树移除，如需追溯历史实现请查看 git 历史与 `docs/` 归档文档。

## 0.1.1 概览

`0.1.1` 是当前这一轮大规模收口后的可提交版本，重点不再是“继续 patch 旧 Electron 壳”，而是把产品、运行链路、文案真源和桌面体验统一收进当前的 `apps/desktop`。

这一版的核心变化：

- 桌面端默认运行链路已经统一到 `Tauri 2 + React + TypeScript + Rust host`
- 设置页和 break prompt 继续收敛成更克制、更像桌面工具的体验
- 多语言、break 文案、tray/runtime 文案已经统一到新的 locale registry
- 调度、smart reminder、tray、fullscreen break、语言切换等关键稳定性问题都做了修复
- 官网单页与站内稳定下载路由已经并入当前版本基线
- 仓库中与主体产品无关的旧资产、旧说明和过渡结构继续被清理，旧 Electron `app/` 壳也已从当前工作树完全移除

## 产品主张

Pauza 不是一个“粗暴打断工作”的提醒器。它更接近一个安静的桌面伴侣，核心观念是：

- 在保护专注的前提下提醒休息，而不是机械地插入干扰
- 尽量先给用户可见 cue，再进入真正的 break
- 把运行时动作、设置项、系统状态和语言文案都收口到单一真源，减少漂移和意外
- 界面应该像一个克制的桌面工具，而不是一个噪音很重的控制台

## 下一阶段：官网部署与下载链路

`0.1.1` 当前已经包含官网单页原型与下载稳定路由，下一阶段不再是“从 0 搭网站”，而是把现有 `apps/site` 正式部署出去，并接上真实下载产物与版本说明。接下来会重点推进：

1. 使用 Vercel 部署 `apps/site`
2. 将 `apps/site/download/targets.js` 接上真实下载地址或 Release
3. 在官网补充更稳定的版本说明、截图与下载说明

网站内容建议优先强调：

- “更自然的 break，而不是粗暴打断”
- “更克制的 desktop experience”
- “focus session / smart reminder / heads-up cue / break prompt” 这一套完整闭环

## 环境要求

- Node.js 20+ 与 npm
- Rust toolchain（`cargo` / `rustup`）
- macOS 打包需要 Xcode Command Line Tools
- 如果要正式签名/公证 macOS 安装包，请提前在 shell 中准备好 Apple Developer 相关环境变量

## 安装依赖

```bash
npm install
```

如果只想单独安装桌面端依赖：

```bash
npm run desktop:install
```

## 本地运行

启动桌面端开发环境：

```bash
npm run dev
```

显式写法：

```bash
npm run desktop:dev
```

当前链路会启动：

- Vite 前端开发服务
- Tauri 2 原生桌面壳

## 本地预览官网

启动网站静态预览：

```bash
npm run site:dev
```

然后打开：

```text
http://127.0.0.1:43210
```

说明：

- 官网目录位于 `apps/site/`
- 下载按钮统一先进入 `apps/site/download/`
- 真实下载地址集中维护在 `apps/site/download/targets.js`

## 常用校验命令

```bash
npm test
npm run typecheck
npm run test:coverage
python3 scripts/sync_desktop_locales.py
python3 graphics/generate_icon_assets.py
```

说明：

- `npm test`：运行当前保留的 Vitest 测试
- `npm run typecheck`：检查 `apps/desktop` 前端 TypeScript
- `npm run test:coverage`：运行测试并生成覆盖率
- `sync_desktop_locales.py`：根据 `apps/desktop/src/locales/{messages,config}` 生成共享 locale registry
- `generate_icon_assets.py`：重生成 Tauri / macOS 打包图标；改过 `graphics/*.svg` 后先跑这条

## 打包桌面端

根目录短入口：

```bash
npm run build
```

显式写法：

```bash
npm run desktop:build
```

两者都会走同一条 Tauri 打包链路。

### 打包当前机器架构的 macOS 版本

```bash
npm run desktop:build
```

如果当前 shell 里已经配置 Apple Developer 签名/公证环境变量，`tauri build` 会直接走签名与 notarization。

如果只想先本地出一个未签名包：

```bash
npm --prefix apps/desktop run tauri build -- --no-sign
```

### 打包 macOS universal 版本

先安装两个 Rust target：

```bash
rustup target add aarch64-apple-darwin x86_64-apple-darwin
```

然后构建 universal `.app` / `.dmg`：

```bash
npm --prefix apps/desktop run tauri build -- --target universal-apple-darwin --bundles app,dmg
```

## macOS 产物位置

默认产物目录：

```bash
apps/desktop/src-tauri/target/release/bundle/
```

常见文件：

- `apps/desktop/src-tauri/target/release/bundle/macos/Pauza.app`
- `apps/desktop/src-tauri/target/release/bundle/dmg/Pauza_0.1.1_*.dmg`

## 说明

- `apps/site/` 当前是独立静态站点目录，后续 Vercel 部署建议直接把 root directory 指到这里
- 旧 Electron 壳已从当前工作树移除；如需追溯历史行为，请查看 git 历史与 `docs/历史版本整理.md`
- 如果需要更细的仓库约束、目录职责和工作流说明，查看 `docs/RepositoryGuidelines.md` 与 `docs/CodeMap.md`
