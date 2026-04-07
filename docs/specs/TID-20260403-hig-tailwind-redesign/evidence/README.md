# Evidence Report · TID-20260403-hig-tailwind-redesign

## Summary
- 目标：证明新的 Tailwind + shadcn + Apple HIG 风格前台已在设置页和 break prompt 两条主可见链路上落地。
- 采集方式：浏览器 preview + Playwright；break prompt 通过 mocked `__TAURI_INTERNALS__` 注入 active break 状态。

## Artifacts
- `settings-redesign.png`
  - 主设置页 full-page 截图；覆盖 overview hero、grouped settings、advanced accordion、save rail 与 live rails
- `settings-snapshot.md`
  - 设置页结构快照；用于追溯可访问树和关键文案
- `break-redesign.png`
  - break prompt 截图；用于证明单卡片、环形倒计时、主次 CTA 的视觉结果
- `break-snapshot.md`
  - break prompt 最新结构快照；用于确认修正后的辅助文案为“剩余时间”
- `browser-console-errors.log`
  - 浏览器 console error 结果；当前为空/无错误

## Observations
- 设置页默认第一屏已不再是控制台式信息板，而是更接近 Apple HIG 的 hero + grouped settings。
- 高级能力已折叠进 accordion，默认视野更克制。
- break prompt 采用居中玻璃卡片、环形倒计时、Done/Later/Skip 三层按钮。
- 在 break prompt 证据采集过程中，修正了一处辅助文案问题：圆环下方从不准确的“暂无休息安排”改为“剩余时间”；该修正由 `break-snapshot.md` 明确证明。

## Caveats
- 浏览器 preview 默认不会进入真实 active break，因此 break prompt 证据依赖 mocked runtime；它用于验证前台渲染层，而不是替代真实 Tauri 运行时回归。
