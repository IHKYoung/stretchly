# Evidence Report · TID-20260403-tauri-core-parity

## Scope
- 任务：Tauri 核心功能补齐并废弃默认 Electron 入口
- 证据类型：文档化证据报告 + 已有构建/测试结果引用 + 相邻任务旁证
- 状态：partial

## Available Verification Evidence
- 命令级验证：
  - `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - `npm --prefix apps/desktop run typecheck`
  - `npm --prefix apps/desktop run build`
- 运行路径验证：
  - 根 `package.json` 默认 `start/dev/build/pack/dist` 已切至 `desktop:*`
  - `npm run desktop:dev` 已作为默认入口运行 Tauri dev runtime
- 相邻任务可复用旁证：
  - `docs/specs/TID-20260403-tauri-usable-core/evidence/settings-snapshot.md`
  - `docs/specs/TID-20260403-tauri-usable-core/evidence/break-snapshot.md`
  - `docs/specs/TID-20260403-tauri-usable-core/evidence/browser-console.log`

## What This Proves
- Tauri host 与前台在编译、类型检查、单测层面可闭合。
- 默认开发/构建入口已经切到 Tauri。
- 主设置页与 break prompt 的基础结构在相邻阶段已有快照旁证，不是空壳。

## Missing Fresh Evidence
- 本任务目录下缺少独立截图/录屏，仍未直接证明以下完成态：
  - 偏 macOS 小工具风格的偏好页中文默认态
  - 默认仅显示 `节奏 / 信号 / 通用` 三组的完成态
  - 高级项折叠态与展开态
  - break prompt 的 `manualAwaiting` 态
  - strict break 下 tray 菜单显隐策略
  - tray 中 skip/focus/pause 子菜单的实际桌面效果

## Recommended Capture Next
- 启动 `npm run desktop:dev`
- 采集：
  - 主设置页默认态截图
  - 主设置页仅显示 `节奏 / 信号 / 通用` 的偏好页态截图
  - 主设置页高级项折叠/展开截图
  - 手动触发 break 后的 prompt 截图
  - `manualAwaiting` 态截图
  - tray 菜单截图或录屏

## Assessment
- 结论：代码与文档层面已收口，但交互证据仍是 `partial`，下一轮如要做 release-level 结案，需补新鲜可视化证据。
