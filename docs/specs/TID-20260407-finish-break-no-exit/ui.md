# Task-ID: TID-20260407-finish-break-no-exit

## Goals
- 点击 break prompt 的“完成休息”后，窗口应正常消失，应用继续在后台运行。
- `稍后` / `跳过` 共享同一 break command close timing 修复，不引入新的行为分叉。

## Non-Goals
- 不改 break prompt 视觉样式、文案、布局或按钮顺序。
- 不改设置页。

## Screens & User Flows
- Primary flow:
  - break prompt 可见 -> 用户点击“完成休息” -> break prompt 关闭 -> 应用继续后台运行
- Fallback / secondary flow:
  - `稍后` / `跳过` 沿用同一 deferred close 时序
- User-visible boundary:
  - 仅 break prompt 关闭后的宿主行为
- Entrypoints / handoff cues:
  - break prompt 底部 CTA：`完成休息`、`稍后`、`跳过`

## Component Tree
- `BreakWindow`
- `Button(done)`
- `Button(later)`
- `Button(skip)`

## Interaction States
- hover:
- 维持现状，无新增 hover 语义
- active:
  - 点击 CTA 后进入命令执行态，按钮按既有 `busyAction` 禁用
- focus:
  - 维持现状
- disabled:
  - 无 `currentBreak` 或命令执行中时禁用
- loading:
  - 维持现状，无新增 loading UI
- empty:
  - break 结束后窗口应直接关闭，而不是停留在“已清空”界面后导致宿主退出
- error:
  - 命令失败时继续沿用现有错误展示
- skeleton:
  - N/A
- optimistic (if applicable):
  - 前端仍依赖命令回包的 `DesktopSnapshot`，不改为纯前端乐观关闭

## Responsive Rules
- 维持现状；本轮不改布局尺寸或断点。

## Accessibility (a11y)
- keyboard navigation:
- 维持现状
- focus order:
  - 维持现状
- aria labels:
  - 维持现状
- contrast:
  - 维持现状
- reduced motion:
  - 维持现状

## Design Tokens / Tailwind Mapping
- typography:
  - 维持现状
- spacing:
  - 维持现状
- color usage:
  - 维持现状
- key classes:
  - 维持现状；本轮只改宿主逻辑

## Micro-animations (optional)
- N/A

## Edge Cases
- long text:
- 不受本轮影响
- slow network:
  - N/A；本地宿主命令
- empty datasets:
  - 无 active break 时 CTA 继续禁用
- permission denied:
  - 不新增权限请求
- offline:
  - N/A；不依赖网络

## Acceptance Criteria (UI)
- “完成休息”关闭 break prompt 后，应用不应表现为直接退出。
- `稍后` / `跳过` 不因本轮时序修复产生可见回归。

## Open Questions
- 待用户本机验证：macOS 上点击“完成休息”后是否仍可在 menu bar 中看到 Pauza。
