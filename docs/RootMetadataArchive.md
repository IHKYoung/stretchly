# Root Metadata Archive

本文件归档原先位于仓库根目录、但已不再保留在主体开发路径中的说明/元数据文件，供后续需要恢复社区治理、法律说明或 Linux 发布元数据时参考。

## 归档范围

- 原 `README.md`
- 原 `CONTRIBUTING.md`
- 原 `CODE_OF_CONDUCT.md`
- 原 `LICENSE`
- 原 `net.hovancik.Pauza.desktop`
- 原 `net.hovancik.Pauza.metainfo.xml`

## 当前有效结论

- 当前主体开发入口是 `apps/desktop/`
- `app/` 仅保留为原版参考目录
- 当前默认运行、构建、测试链路不依赖上述六个根文件
- 如果后续恢复 GitHub 社区页面、Linux 打包/商店发布或正式开源法律说明，应优先从本文件提取信息，再决定是否重新放回根目录

## 原 README 的有用信息

- 当前唯一有效桌面端实现：`apps/desktop/`
- 原版参考代码：`app/`
- 常用命令：
  - `npm start`
  - `npm run build`
  - `npm test`
  - `python3 scripts/sync_desktop_locales.py`
  - `python3 graphics/generate_icon_assets.py`

## 原 CONTRIBUTING 的有用信息

- 使用 `package.json` 规定的 Node 环境
- 有行为或代码变更时尽量补测试
- 变更记录写入 `docs/CHANGELOG.md`
- 不随意改版本号
- 保持现有代码风格
- 如果开发入口或开发说明变化，应同步更新 `docs/`

## 原 Code of Conduct 的有用信息

- 使用 Contributor Covenant 1.4 作为社区行为准则
- 历史报告联系邮箱：`jan@hovancik.net`
- 如果未来恢复公开协作入口，可基于该准则重新生成新的社区治理文件

## 原 LICENSE 的有用信息

- 历史许可证类型：`BSD-2-Clause`
- 历史版权行：

```text
Copyright (c) 2016-, Jan Hovancik
All rights reserved.
```

- 原许可证正文：

```text
Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

* Redistributions of source code must retain the above copyright notice, this
  list of conditions and the following disclaimer.

* Redistributions in binary form must reproduce the above copyright notice,
  this list of conditions and the following disclaimer in the documentation
  and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```

## 原 Linux Desktop Entry

```ini
[Desktop Entry]
Name=Pauza
Exec=run.sh %U
Terminal=false
Type=Application
Icon=net.hovancik.Pauza
StartupWMClass=Pauza
Comment=The break time reminder app
Categories=Utility;
```

## 原 AppStream / Linux Store 元数据要点

- Component ID: `net.hovancik.Pauza`
- Launchable desktop id: `net.hovancik.Pauza.desktop`
- Metadata license: `CC0-1.0`
- Project license: `BSD-2-Clause`
- Developer name: `Clarke Young`
- Summary: `The break time reminder app`
- 历史描述：`Pauza is a cross-platform Electron app that reminds you to take breaks when working on your computer.`
- 历史 URL：
  - bugtracker: `https://github.com/hovancik/stretchly/issues`
  - donation: `https://hovancik.net/stretchly/sponsor/`
  - help: `https://hovancik.net/stretchly/features/`
  - homepage: `https://hovancik.net/stretchly/`
  - translate: `https://hosted.weblate.org/engage/stretchly/`
- 历史截图 URL：
  - `https://hovancik.net/stretchly/img/flathub/minibreak.png`
  - `https://hovancik.net/stretchly/img/flathub/welcome.png`
  - `https://hovancik.net/stretchly/img/flathub/dark.png`

## 恢复建议

- 如果未来恢复根 `README.md`，应优先以 `apps/desktop` 单真源结构重写，而不是恢复旧图文版。
- 如果未来恢复根 `LICENSE`，应先确认法律归属是否仍沿用历史 BSD-2-Clause 文本。
- 如果未来恢复 Linux 发布元数据，应基于当前真实 runtime（Tauri 而非 Electron）重写 `desktop` / `metainfo` 描述，而不是原样回填历史文案。
