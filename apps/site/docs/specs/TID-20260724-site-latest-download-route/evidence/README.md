# TID-20260724-site-latest-download-route Evidence

## Scope

- 验证官网以 GitHub Latest Release API 的资产名构造 `releases/latest/download/<asset-name>`。
- 验证 API 失败或无匹配资产时保留 pinned v0.1.4。
- 验证独立官网 `baseline` 的 Vercel Production 部署与线上最终 href。

## Pre-deploy Evidence

- Root cause: 线上 Vercel Production 仍为 `bad94ee`，`index.html` 与 `download/targets.js` pinned 为 v0.1.2；旧 `script.js` 在 latest 资产名不等于 pinned 时主动返回 pinned。
- GitHub latest: `tag_name=v0.1.4`；asset=`Pauza_0.1.4_aarch64.dmg`；digest=`sha256:20c4be6cbe3d8f4897e4b99ef07c5a4be701a7ba6b6f5d9d3d1c4658d5acea10`。
- Redirect probe: `releases/latest/download/Pauza_0.1.4_aarch64.dmg` 返回 HTTP 302，`Location` 指向 `/releases/download/v0.1.4/Pauza_0.1.4_aarch64.dmg`。
- Runtime smoke:
  - `PASS latest release overrides pinned fallback`
  - `PASS failed API keeps pinned fallback`
  - `PASS missing asset keeps pinned fallback`
- Static/project checks: `node --check` PASS；`git diff --check` PASS；Vitest `113 passed`；发布脚本 `--skip-release` 幂等 PASS。

## Production Evidence

- GitHub deployment ID: `5585280999`。
- Deployment ref/SHA: `9bf74565c72dc2f5aa9328ab351a7056df3b2f80`。
- Environment/state: `Production` / `success`；description=`Deployment has completed`。
- Vercel target: `https://pauza-a3bx1zgt5-ahaknow.vercel.app`。
- Custom domain: `https://pauza.ahaknow.com/` 返回 HTTP 200，`server: Vercel`。
- Live resources: `download/targets.js` 包含 pinned v0.1.4 与 `latestDownloadBaseUrl`；`script.js` 使用 API 资产名构造 latest 路径。
- Live DOM probe: `PASS final href: https://github.com/IHKYoung/Pauza/releases/latest/download/Pauza_0.1.4_aarch64.dmg`。
- Live console probe: `PASS console warnings: 0`。
- Latest redirect: HTTP 302；`Location: https://github.com/IHKYoung/Pauza/releases/download/v0.1.4/Pauza_0.1.4_aarch64.dmg`。
- Result: PASS。生产官网已自动解析 latest release，并保留 v0.1.4 pinned failure fallback。
