# TID-20260803-current-worktree-handover Evidence

## Scope
- 当前周期验证 desktop smart-wait snapshot/tray、settings custom/cadence/typography、locale registry、site latest source contract 和 workflow/commit closure。
- `apps/site/docs/specs/TID-20260724-site-latest-download-route/evidence/README.md` 保留该任务已完成的生产 deployment、最终 DOM href 与 GitHub 302 证据；本轮不重复外部发布。

## Automated Evidence
- `python3 scripts/sync_desktop_locales.py`：PASS，50 languages。
- `python3 scripts/sync_desktop_break_ideas.py`：PASS，50 languages / 50 bundles。
- `npm test`：PASS，7 files / 127 tests。
- `npm run typecheck`：PASS。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：PASS，31 tests。
- `npm --prefix apps/desktop run build`：PASS，1827 modules；保留既有大 chunk warning（JS 4,303.48 kB / gzip 1,330.25 kB）。
- `node --check apps/site/{script.js,download/targets.js}`：PASS。
- `/private/tmp/pauza-site-download-smoke.mjs`：PASS latest override、API reject pinned fallback、missing asset pinned fallback。
- changed tests targeted Standard：PASS；`git diff --check`：PASS。
- root workflow docs：PASS（2026-08-03）；site workflow docs：PASS（2026-07-24）；agent/kit validators 均按仓库状态 SKIP。
- `cargo fmt --check`：BLOCKED，stable toolchain 未安装 `cargo-fmt`/`rustfmt`，未安装新组件。
- 根 `npm run lint`：FAIL；主要原因是命令扫描 ignored Tauri `target/` 生成 JS，并命中仓库既有 site/test 尾逗号与 no-void 风格。当前新增的三份 JS tests 已通过 targeted Standard。

## UI / Reality Evidence
- `npm --prefix apps/desktop run dev`：PASS，Vite ready at `http://127.0.0.1:43179/`。
- Browser runtime setup 成功，但 browser discovery 返回空列表；无法取得真实 settings preview 截图。
- 视觉证据结论：BLOCKED（环境无 browser backend）。源码回归、typecheck 和 build 只证明结构/编译正确，不替代视觉截图。

## Exclusions
- `scratch/locale-translations/**` 为 ignored 本地中间物，不属于证据或提交内容。
- 前序讨论的休息提示语尚未写入，不属于本 task evidence。
