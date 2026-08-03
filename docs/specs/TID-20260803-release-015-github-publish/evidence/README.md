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
