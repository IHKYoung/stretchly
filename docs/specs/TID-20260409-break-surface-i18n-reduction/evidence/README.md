# Task-ID: TID-20260409-break-surface-i18n-reduction

## Evidence Summary
- `break-glass-preview.png`：验证休息界面仍为单列纯净结构，仅保留交互语、大号倒计时和细进度条；卡片透明度提高，并加入更明显的玻璃通透感。
- `settings-surface-preview.png`：验证设置页的“休息窗口 / 休息音效 / 语言”区域已经收敛为纯中文用户文案，且新增微休息/休息的开始音与结束音设置项。

## AC Mapping
- AC1 -> `break-glass-preview.png`
- AC4 -> `settings-surface-preview.png` + `apps/desktop/src/App.tsx` 的结束音调度逻辑
- AC5 -> `settings-surface-preview.png`
- AC6 -> `apps/desktop/src/locales/messages/*.json`、`apps/desktop/src/locales/config/*.json`、`apps/desktop/src/locales/registry.generated.json`
- AC7 -> `package.json`、`test/*.js`、`docs/{Architecture,CodeMap,RepositoryGuidelines}.md` 对 legacy `app/` 的引用审查

## Notes
- 当前浏览器证据采集链路已稳定覆盖 break 主界面与设置分组，但上传后的自定义壁纸预览态在采集环境中没有稳定保留；该项以 `apps/desktop/src/App.tsx` 的 `object-contain` 预览实现、前端构建通过和用户现场截图作为补充依据。
- legacy `app/` 本轮只完成审查，不做直接删除；证据目标是给出“不能整体删”的可追溯依据，而不是制造一次破坏性删除实验。
