# Pauza Desktop

`apps/desktop` 是 Pauza 当前唯一有效的桌面端主实现，使用 `Tauri 2 + React + TypeScript`。

这里既负责运行时，也负责桌面端自己的多语言、设置页、break prompt、Rust host 和迁移中的领域逻辑落点。

## Commands

- `npm install`
- `npm run tauri:dev`
- `npm run tauri:build`
- `npm run typecheck`

## Migration rule

- `apps/desktop` 是单一真源；新的桌面端能力只允许加在这里。
- 若旧 `app/**` 里仍有行为值得保留，先迁移到 `apps/desktop/**`，再继续演进；不要把新的运行链路接回 `app/**`。
- locale 只维护 `src/locales/messages/*.json`、`src/locales/config/*.json` 和 `src/locales/break-message-copy.json`。
- `app/**` 当前仅作为归档参考保留，不再参与默认运行、构建或测试链路。
