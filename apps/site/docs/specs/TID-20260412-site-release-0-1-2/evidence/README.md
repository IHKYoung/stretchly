# Evidence Report

- Task-ID: TID-20260412-site-release-0-1-2
- Scope: 验证首页下载按钮在当前 GitHub latest 仍为 `v0.1.1` 的情况下，最终 href 仍保持 `Pauza_0.1.2_aarch64.dmg`
- Environment: 本地静态服务 `http://127.0.0.1:43211/`
- Artifacts:
  - Screenshot: `docs/specs/TID-20260412-site-release-0-1-2/evidence/site-download-release-0-1-2.png`
- GitHub latest evidence:
  - `tag_name = v0.1.1`
  - `asset = Pauza_0.1.1_aarch64.dmg`
- Browser evidence:
  - Playwright snapshot showed `[data-download-button]` 最终 URL 为 `https://github.com/IHKYoung/Pauza/releases/download/v0.1.2/Pauza_0.1.2_aarch64.dmg`
  - 页面标题：`Pauza | 更安静的久坐提醒`
- Result Summary: PASS。尽管 GitHub latest 仍返回 `v0.1.1`，页面最终 href 依然保持 `0.1.2`，说明 pinned-version 保护逻辑生效。
