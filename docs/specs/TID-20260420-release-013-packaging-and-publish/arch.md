# Task-ID: TID-20260420-release-013-packaging-and-publish

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。

## Goals
- 统一 `0.1.3` 的版本真源、发布文档、site pinned URL 与 macOS 打包输入。
- 让本地代码工作树、站点 release 流程和 GitHub release 资产形成一条可追溯的发布链。

## Non-Goals
- 清理或提交根目录所有翻译 scratch 文件。
- 重构代码仓库与 release 仓库的分离关系。
- 引入新的产品功能或改动既有 UI 交互语义。

## Constraints & Assumptions
- 当前 `apps/site` 的发布脚本与 pinned URL 都以 `IHKYoung/Pauza` 为 release 仓库。
- 当前代码工作树仍在本仓库内管理，commit push 先按当前代码远端执行。
- macOS 打包允许使用当前机器架构的无签名 dmg 作为 release 资产输入。

## System Boundaries
- Modules:
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src/locales/messages/*.json`
  - `apps/desktop/src/locales/registry.generated.json`
  - `apps/site/{index.html,script.js,download/targets.js,README.md}`
  - `apps/site/{docs,scripts,.githooks}/**`
  - `README.md`
  - `docs/**`
- Ownership:
  - Desktop runtime/build: `apps/desktop/**`
  - Site release/distribution: `apps/site/**`
  - Release narration and audit: `docs/**`
- Dependency direction:
  - `messages/*.json` -> `registry.generated.json` -> desktop frontend/Rust host
  - 本地 dmg 产物 -> `apps/site/scripts/publish_site_release.py` -> GitHub release + pinned URL
  - `docs/**` 记录上述边界与验证结果，不反向驱动运行时

## API / Contract
- Signatures / Endpoints:
  - `python3 scripts/sync_desktop_locales.py`
  - `python3 apps/site/scripts/publish_site_release.py --asset <dmg>`
  - `gh release create|upload --repo IHKYoung/Pauza`
  - `git push <code-remote> <branch>`
- Request/Response schema (typed):
  - 资产命名约定：`Pauza_<version>_aarch64.dmg`
  - release tag 约定：`v<version>`
  - site pinned URL 约定：`https://github.com/IHKYoung/Pauza/releases/download/v<version>/Pauza_<version>_aarch64.dmg`
- Error model (codes, retryability):
  - locale registry 未同步：阻断 release，修正后可重试
  - macOS dmg 构建失败：阻断 release，修正构建环境后可重试
  - GitHub release 上传失败：可在同版本上重新 upload `--clobber`
  - Git push 失败：不得 force push，按失败原因重试或保留本地 commit

## Data Model / Storage
- 版本真源：`package.json`、`package-lock.json`、`apps/desktop/package*.json`、`Cargo.toml`、`tauri.conf.json`
- 运行时 locale 快照：`apps/desktop/src/locales/registry.generated.json`
- 发布资产：`apps/desktop/src-tauri/target/release/bundle/dmg/Pauza_0.1.3_aarch64.dmg`
- 站点分发真源：`apps/site/index.html` 与 `apps/site/download/targets.js`

## Invariants
- 版本号必须在所有正式真源中统一为 `0.1.3`
- locale registry 必须与 `messages/*.json` 同步
- GitHub release 资产文件名、tag 与站点 pinned URL 必须保持同一版本
- 根目录翻译 scratch 文件不应进入正式版本提交

## Concurrency / Lifecycle / Memory Model
- 无新增运行时并发模型；本轮仅发布既有改动
- 需要关注的是发布生命周期：先统一真源与 docs，再验证，再打包，再 release，再 commit/push

## Observability Plan (Debug-Driven)
- Logs:
  - 保留 `gh release view`、构建输出、validator 输出和 `git status` 结果到日志
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - 打包阶段使用 `--no-sign` 以减少与签名环境相关的不确定性

## Security & Privacy Considerations
- 不新增外部依赖或密钥
- GitHub release 与 push 使用当前已登录的 `gh`/git 凭据
- 不执行历史重写或破坏性操作

## Risks & Rollback
- Failure modes:
  - 代码仓库与 release 仓库分离，可能导致版本追踪歧义
  - dmg 构建失败或产物命名不符合 release 脚本约定
  - release 上传成功但 site pinned URL 未同步
- Rollback steps:
  - 回退版本真源、文档与 site pinned URL
  - 对错误 release 重新上传正确资产或删除错误 tag
  - 对错误代码推送使用新的 `git revert` 提交回滚

## Acceptance Criteria (System)
- `0.1.3` 版本真源统一且 docs/changelog/readme 完整
- locale registry 与 50 个 locale 消息文件同步
- 本地 macOS dmg 构建成功并生成 `Pauza_0.1.3_aarch64.dmg`
- GitHub release `v0.1.3` 存在且带有对应资产
- 代码仓库存在可追溯的 `0.1.3` 提交并完成 push

## Open Questions / Decision Requests
- 若代码 push 的长期目标仓库不是当前代码远端，应在后续单独收口“代码仓库 / release 仓库分离”问题，而不是在本轮 release 中临时改写结构。
