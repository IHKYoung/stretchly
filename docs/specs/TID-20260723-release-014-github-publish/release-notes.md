# Pauza 0.1.4

Pauza 0.1.4 将当前 Tauri 2 桌面端的提醒与内容资源收口为一个可分发版本。

主要变化：

- 智能提醒采用固定空档阈值与最长等待模型，避免持续工作时无限等待，同时保留强制提醒的直接投递语义。
- break ideas 从界面 locale 中独立为单独资源层，并扩充英文、简体中文与繁体中文提示内容。
- 修复自然休息 credit 可能提前消耗尚未到点微休息的问题。
- 拆分 Rust host 的 settings schema、持久化与测试模块，保持运行时调度行为不变但边界更清晰。
- 官网固定下载入口同步到本次 Apple Silicon macOS 安装包。

安装包：`Pauza_0.1.4_aarch64.dmg`

系统要求：Apple Silicon Mac；首次打开时请按 macOS 的安全提示完成确认。
