# Task-ID: TID-20260723-release-014-github-publish

## Test Strategy
- Unit: `npm test`、Rust tests。
- Integration: registry 重建幂等、TypeScript typecheck、desktop frontend build、Tauri DMG build。
- E2E (if applicable): site pinned URL 与 GitHub release/asset/download hash 端到端核验。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 版本 `rg` 检查、changelog/readme/site URL 检查。
- AC2 -> generators、`npm test`、`npm run typecheck`、`cargo test`、frontend build、workflow validators。
- AC3 -> 新构建时间、文件名/大小/SHA-256、`codesign`、`spctl`、`stapler`。
- AC4 -> git status/log/remote refs、`gh release view`、GitHub asset metadata、重新下载 SHA-256。

## Interaction Contract Coverage
- Interaction impact: indirect
- Primary flow -> tests/evidence: 检查首页初始 href 与发布后 URL 响应。
- Fallback / secondary flow -> tests/evidence: 静态检查 pinned fallback 为 0.1.4，并复用现有 latest-version guard tests。
- Visible states / transitions -> tests/evidence: 记录 0.1.3 -> 0.1.4 URL diff、release absent/present 与错误门禁。
- Validator expectation: primary/fallback/visible states 均有命令或 metadata 证据。

## Governance Gates
- Agent Config Validation: `python3 scripts/validate_agent_configs.py`
- Workflow Docs Validation: `python3 scripts/validate_workflow_docs.py --mode manual`
- Approval Escalation Owner: orchestrator

## False-pass Cases
- generator 改写 registry 后仍继续发布。
- 测试通过但 DMG 来自旧构建时间或旧 commit。
- `codesign`/`stapler` 失败却把包描述为已签名/公证。
- GitHub release 存在但 tag target、资产名或下载 hash 不匹配。
- site 指向 0.1.4，但 release 资产尚不可下载。
- push/release 任一步失败却宣称整体发布完成。

## Evidence Capture (UI / E2E)
- Required: yes
- Owner: orchestrator
- Artifacts path: docs/specs/TID-20260723-release-014-github-publish/evidence/
- What to capture:
  - Screenshots: 不要求；release metadata 比页面截图更可复现。
  - Video/trace (optional): 不要求。
  - HAR/console logs (optional): 记录 GitHub API/CLI metadata 与 hash。

## Quality Gates (Non-functional)
- a11y: 下载链接语义与 aria-label 不变。
- perf budget: 无新增运行时代码或资源。
- error handling / observability: 所有失败分类显性；不使用 fallback 掩盖签名/上传失败。
- security / privacy: 不输出或写入 token、Apple 密码。

## Boundary / Invalid Input Cases
- 资产名不匹配 `Pauza_0.1.4_aarch64.dmg` 时发布脚本必须拒绝。
- 已有远端 tag/release 时先比较 target 与 asset，再决定 upload；不盲目 clobber。
- GitHub auth 无效时不得更新外部状态。

## Concurrency / Race Cases (if applicable)
- 串行发布；禁止同时修改同名 release 资产。
- site pinned URL 先作为本地提交存在，但只有 release 资产验证通过后才视为对外可用。

## Mocks & Test Data
- 使用当前真实 0.1.4 source 与本机 arm64 构建产物；不使用旧 DMG 作为成功证据。

## Commands to Run
- `python3 scripts/sync_desktop_locales.py`
- `python3 scripts/sync_desktop_break_ideas.py`
- `git diff --check`
- `npm test`
- `npm run typecheck`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_agent_configs.py`
- `python3 scripts/validate_workflow_docs.py --mode manual`
- `npm --prefix apps/desktop run tauri build -- --bundles app,dmg`
- `codesign -dv --verbose=4 <app>`、`spctl --assess ...`、`xcrun stapler validate <app/dmg>`
- `shasum -a 256 <dmg>`
- `gh release view v0.1.4 --repo IHKYoung/Pauza --json ...`

## Expected Results
- PASS criteria: 所有本地门禁通过；新 DMG 可追溯到发布 commit；签名状态真实记录；远端 release/tag/asset/site URL 对齐且下载 hash 相同。
- Outputs to keep (10~20 lines snippet): 测试计数、build 完成行、签名/stapler 结论、DMG size/hash、commit/tag/release metadata。
