# Release Evidence

本目录记录 `Pauza 0.1.5` 的构建、签名公证、GitHub Release、远端下载与官网生产链路证据。

发布前状态：

- Public latest release: `v0.1.4`
- Target version: `v0.1.5`
- Target repository: `IHKYoung/Pauza`
- Root branch push target: `origin/baseline`（仅快进）
- Divergent `pauza/baseline`: 不更新
- Existing `v0.1.5` tag/release: none

最终 commit、tag、DMG hash/size、Apple 验证、Release URL 和官网回验结果将在发布完成后回填。

## Release Candidate Gates

- Locale registry：`50` languages；连续生成 SHA-256 `53e00d6ec14bec7786f689f489ef32c3dc24b16233a08472197984a85816bf57`，幂等。
- Break ideas registry：`50` languages / `50` bundles / `1` managed batch；连续生成 SHA-256 `474f7ef9c8dd7aa01b32e16a4062182e26b94bbbc85b66a5d1313388542cb8ec`，幂等。
- Vitest：`8` files / `132` tests PASS。
- TypeScript：`tsc --noEmit` PASS。
- Rust：`31` tests PASS。
- Desktop frontend production build：`1827` modules PASS；保留既有 `4,561.25 kB` JS chunk warning，本次未新增依赖。
- Site runtime smoke：latest success、API failure fallback、missing asset fallback 三路径 PASS。
- Site static contract：`index.html` 与 `targets.js` 同为 v0.1.5；latest API 仍为主路径。
- Changed JS Standard、Python compile、root/site `git diff --check` PASS。
- Root/site workflow docs PASS；agent config 因未配置 multi-agent 而按设计 SKIP。
- 环境说明：默认 Anaconda Python 3.10 缺少 `tomllib`；workflow validation 与 Git hooks 使用已安装的 `/opt/homebrew/bin/python3` 3.14.6。

## Source And Tag

- Release feature commit: `93742661026bc841e6f3c919d6a8f78f2a5ba510`。
- Release candidate audit commit: `f755754916bd682969d50b3a10815262be07546c`。
- Root `origin/baseline`: 已快进包含上述提交；未更新历史分叉的 root `pauza/baseline`。
- Annotated tag: `v0.1.5`，tag object `b77791f6768455d1cbf83e713bc0fbbca1eff876`。
- Tag target: `f755754916bd682969d50b3a10815262be07546c`；`origin` 与 `pauza` 两个远端一致。
- Tag annotation: 记录 asset name、size、SHA-256 与 app/DMG Apple notarization submission IDs。

## Signed Build

- Artifact: `Pauza_0.1.5_aarch64.dmg`。
- Size: `19,970,212` bytes。
- SHA-256: `558a96dceb4eddf025e5891a38d16ff1b70e5be67efec563c01c460e09364fff`。
- App notarization: `90fa7020-6d4c-4635-ab96-d130c4895a66`，status=`Accepted`，ticket stapled。
- DMG notarization: `3126bfab-4ad1-4b1b-804b-ef3e3740701f`，status=`Accepted`，ticket stapled。
- Signature authority: `Developer ID Application: AHAKNOW LLC (HC559NT2NP)`。
- Local app/DMG: strict codesign PASS；Gatekeeper `Notarized Developer ID` accepted；stapler validate PASS。
- DMG integrity: protective MBR/GPT/HFS/backup GPT CRC 全部 verified；read-only mount PASS。
- Mounted app: Mach-O arm64；CFBundleShortVersionString/CFBundleVersion=`0.1.5`；codesign/Gatekeeper/stapler PASS。

## GitHub Release

- Release URL: `https://github.com/IHKYoung/Pauza/releases/tag/v0.1.5`。
- Published: `2026-08-03T14:22:43Z`；draft=false；prerelease=false；latest=`v0.1.5`。
- Asset ID: `RA_kwDOR_-wb84dz6Hg`；content type=`application/x-apple-diskimage`；state=`uploaded`。
- GitHub digest/size: `sha256:558a96dceb4eddf025e5891a38d16ff1b70e5be67efec563c01c460e09364fff` / `19,970,212` bytes，与本地一致。
- Remote re-download: 完整文件 SHA-256/size 一致；DMG codesign/Gatekeeper/stapler PASS；远端镜像装载后 app 的 arm64/0.1.5/codesign/Gatekeeper/stapler 全部 PASS。

## Website Production

- Site release/audit commits: `e71826d1882fcfce2fbec19486af18f88de79daa` / `b13ac9ba53c59e6d67512c7aa281d6cf97ae8b9e`。
- Site evidence/audit commits: `51573f60c71ce09d0857aa93f056be5162ff4d8e` / `17fcac4a9defd12a25016c04e73cc5aa558fcd72`。
- Runtime deployment: GitHub deployment `5728041914`，ref=`b13ac9b...`，Production success，Vercel URL=`https://pauza-nav753wrl-ahaknow.vercel.app`。
- Final docs-only deployment: `5728108624`，ref=`17fcac4...`，Production success，Vercel URL=`https://pauza-nwj4nugfp-ahaknow.vercel.app`。
- Custom domain: `https://pauza.ahaknow.com/` HTTP 200，server=Vercel。
- Production initial fallback: tagged `v0.1.5/Pauza_0.1.5_aarch64.dmg`。
- Production live resolver: final href=`https://github.com/IHKYoung/Pauza/releases/latest/download/Pauza_0.1.5_aarch64.dmg`；console warnings=`0`。
- Latest URL: HTTP 302 到 tagged v0.1.5 asset；fixed URL: HTTP 302 到 GitHub release asset CDN。
- Visual evidence: 本轮未改页面视觉，且之前已记录 Browser runtime 无实例；用 production HTML/JS、live DOM probe、HTTP、deployment 与真实资产下载替代截图，未把未验证视觉状态声明为通过。

## Result

PASS。`f755754` 源码快照、annotated tag、签名公证 DMG、GitHub latest Release、官网 primary/fallback 和生产部署形成完整可追溯链；`v0.1.4` 保持可用作为历史稳定版本。
