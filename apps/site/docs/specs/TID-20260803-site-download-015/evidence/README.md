# Site Download Evidence

发布前状态：

- Site branch: `baseline` at `78a1dea`，与 `origin/baseline` 一致。
- Primary route: GitHub latest release API。
- Previous pinned fallback: `v0.1.4/Pauza_0.1.4_aarch64.dmg`。
- Preserved unrelated dirty files: `copy.js`、`docs/ReleasePlaybook.md`、`docs/specs/_template/arch.md`、`docs/specs/_template/ui.md`。

最终 site commit、deployment 和 production HTTP 证据将在 GitHub `v0.1.5` Release 验证后回填。

## Pre-deploy Validation

- `node --check script.js` / `download/targets.js`: PASS。
- Static contract: `index.html` href 与 `targets.js` fallback 均为 `v0.1.5/Pauza_0.1.5_aarch64.dmg`；latest API 与 `_aarch64.dmg` matcher 保持。
- Runtime smoke: latest release override、API failure fallback、missing asset fallback 三路径 PASS。
- `git diff --check`: PASS。
- Workflow docs: PASS；agent config 未配置 multi-agent，按设计 SKIP。
- Existing unrelated dirty files remain unstaged candidates: `copy.js`、`docs/ReleasePlaybook.md`、`docs/specs/_template/arch.md`、`docs/specs/_template/ui.md`。

## GitHub Release Authority

- Release: `https://github.com/IHKYoung/Pauza/releases/tag/v0.1.5`。
- Latest API: `tag_name=v0.1.5`，draft/prerelease 均为 false。
- Asset: `Pauza_0.1.5_aarch64.dmg`，`19,970,212` bytes。
- Digest: `sha256:558a96dceb4eddf025e5891a38d16ff1b70e5be67efec563c01c460e09364fff`。
- Direct URL: `https://github.com/IHKYoung/Pauza/releases/download/v0.1.5/Pauza_0.1.5_aarch64.dmg`。
- Remote re-download: SHA-256/size 与本地一致；DMG/app codesign、stapler、Gatekeeper、read-only mount、arm64 与 bundle version `0.1.5` 全部 PASS。

## Production Evidence

- Site release commit: `e71826d1882fcfce2fbec19486af18f88de79daa`。
- Site audit commit: `b13ac9ba53c59e6d67512c7aa281d6cf97ae8b9e`。
- GitHub deployment: `5728041914`，ref/SHA=`b13ac9ba53c59e6d67512c7aa281d6cf97ae8b9e`。
- Deployment state: `Production / success`，description=`Deployment has completed`。
- Vercel deployment URL: `https://pauza-nav753wrl-ahaknow.vercel.app`。
- Custom domain: `https://pauza.ahaknow.com/` 返回 HTTP 200，`server: Vercel`。
- Production HTML initial href: `https://github.com/IHKYoung/Pauza/releases/download/v0.1.5/Pauza_0.1.5_aarch64.dmg`。
- Production targets: pinned v0.1.5；latest API/base 和 `_aarch64.dmg` matcher 均存在。
- Live runtime probe: final href=`https://github.com/IHKYoung/Pauza/releases/latest/download/Pauza_0.1.5_aarch64.dmg`，console warnings=`0`。
- Latest URL: HTTP 302 到 tagged v0.1.5 asset；fixed URL: HTTP 302 到 GitHub release asset CDN。
- Result: PASS。官网 primary/fallback 都已指向同一份经过签名、公证和远端 hash 回验的 v0.1.5 DMG。
