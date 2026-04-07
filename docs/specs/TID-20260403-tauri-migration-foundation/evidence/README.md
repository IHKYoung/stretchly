# Task-ID: TID-20260403-tauri-migration-foundation Evidence

## Scope
- 本次证据覆盖 Tauri 2 基础壳的 dashboard 结构、浏览器 preview fallback，以及宿主层启动关键日志。

## Captured Evidence
- 浏览器 preview：访问 `http://127.0.0.1:1420`，页面标题为 `Pauza Desktop`，dashboard 可正常渲染。
- 结构快照：
  - `preview-default.md`
  - `preview-focus.md`
  - `preview-console.log`
- 运行日志：`npm --prefix apps/desktop run tauri:dev` 启动到 `Running target/debug/pauza-desktop`。
- 交互契约：preview 模式下不再因为缺失 Tauri runtime 而抛出 `invoke` 读取错误；CTA 使用本地模拟状态，focus session 与 launch on login 都能反馈到 UI。

## Primary Flow Check
- 主窗口存在 hero、metrics、shell capability、migration queue、legacy source basis 与 footer 状态文案。
- 主 CTA 支持 `Protect focus for 45m`、`Focus for 25m`、`Enable/Disable launch on login`。

## Fallback Flow Check
- 当页面在纯浏览器打开时，runtime 标记为 `Browser preview`。
- preview 模式可继续用于视觉评审和截图，不依赖原生宿主桥接。

## Visible States
- Empty: `No focus session active`
- Success: 触发 preview focus session 后，剩余时间进入 metrics，footer 更新最近动作
- Disabled: busyAction 期间 CTA 进入 disabled 态
- Error/Fallback: 真实错误走 error banner；缺失 Tauri runtime 走 preview fallback

## Notes
- 当前证据以 preview 和启动日志为主，尚未覆盖真正的系统级宿主行为录屏。
- 下一阶段如果迁移到真实 break flow，应补一轮更完整的桌面录屏与多状态截图。
