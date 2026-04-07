# Settings Inventory

这份清单用于回答两个问题：

1. 现在设置页里已经有哪些设置。
2. 还有哪些设置可以加回设置页，或者值得在后续版本里重新实现。

源码依据：

- `apps/desktop/src/App.tsx`
- `apps/desktop/src-tauri/src/state.rs`
- `apps/desktop/src-tauri/src/commands.rs`
- `app/utils/defaultSettings.js`

## 1. 当前设置页已经提供的设置

当前前端设置页实际露出的，是下面这 26 项。

### 节奏

- 微休息开关
- 微休息间隔分钟数
- 微休息时长秒数
- 长休息开关
- 长休息每几次微休息触发
- 长休息时长分钟数

### 提醒与打断

- 微休息提前提醒开关
- 微休息提前提醒秒数
- 长休息提前提醒开关
- 长休息提前提醒秒数
- 微休息允许延后
- 微休息每次延后分钟数
- 长休息允许延后
- 长休息每次延后分钟数
- 微休息严格模式
- 长休息严格模式
- 休息显示方式：窗口 / 全屏
- 打断风格：`gentle / balanced / immersive`

### 智能暂停

- 自然休息开关
- 自然休息后多少分钟重置
- 监控 DND / 专注模式
- 应用排除开关
- 应用排除规则：`pause / resume`
- 应用排除命令列表

### 通用

- 开机自启动
- 语言

## 2. 当前 Tauri 已经支持，但前端还没放出来的设置

下面这些项已经有 Tauri 真源，或者已经有快捷键/运行时绑定。它们属于“可以直接加回设置页”的范围，不需要先重新设计后端数据结构。

### 延后与完成策略

- 微休息延后次数上限：`microbreak_postpones_limit`
- 长休息延后次数上限：`long_break_postpones_limit`
- 微休息手动结束：`microbreak_manual_finish`
- 长休息手动结束：`long_break_manual_finish`

### 多屏与显示细节

- 所有屏幕都显示：`show_breaks_on_all_screens`
- 目标屏幕：`target_screen`
- 休息窗口显示当前时间：`current_time_in_breaks`
- 严格模式下仍允许 tray menu：`show_tray_menu_in_strict_mode`

### 快捷键

- 打开设置：`reveal_settings_shortcut`
- 专注 45 分钟：`focus_45_shortcut`
- 切换暂停：`pause_toggle_shortcut`
- 暂停 30 分钟：`pause_30_shortcut`
- 暂停 60 分钟：`pause_60_shortcut`
- 暂停 120 分钟：`pause_120_shortcut`
- 暂停 300 分钟：`pause_300_shortcut`
- 跳到下一个计划休息：`skip_next_scheduled_shortcut`
- 跳到下一个微休息：`skip_next_microbreak_shortcut`
- 跳到下一个长休息：`skip_next_long_break_shortcut`
- 重置节奏：`reset_breaks_shortcut`

### 说明

- 这些项已经存在于 `PauzaSettings`，或者已经被 `shortcut_bindings()` 读取。
- 也就是说，如果你决定把它们重新加回设置页，主要是前端信息架构和交互表达的问题，不是后端能力缺失的问题。

## 3. 旧版 Electron 有过，但当前 Tauri 还没接回来的候选设置

下面这些项在旧版 `defaultSettings.js` 里存在，但当前 Tauri `PauzaSettings` 没有对应真源。它们不是“直接加 UI”就能生效的项，而是“要先恢复或重写后端能力”的候选项。

### 外观与主题

- 主题来源：`themeSource`
- 主色 / 微休息色：`mainColor` / `miniBreakColor`
- 透明模式：`transparentMode`
- 模糊背景：`blurredBackground`
- 窗口透明度：`opacity`

### 声音与通知体验

- 长休息提示音：`longBreakAudio`
- 微休息提示音：`miniBreakAudio`
- 微休息开始音：`miniBreakStartSound`
- 长休息开始音：`longBreakStartSound`
- 音量：`volume`
- 静音通知：`silentNotifications`

### 休息内容与想法

- 是否显示 ideas：`ideas`
- 是否使用设置里的 ideas：`useIdeasFromSettings`
- 长休息 ideas：`breakIdeas`
- 微休息 ideas：`microbreakIdeas`

### 更新与系统行为

- 新版本提醒：`notifyNewVersion`
- 检查更新：`checkNewVersion`
- 禁用应用更新功能：`disableAppUpdateFeatures`
- 锁屏 / 睡眠时自动暂停：`pauseForSuspendOrLock`
- 早晨时间：`morningHour`
- 暂停到早晨快捷键：`pauseBreaksUntilMorningShortcut`

### 托盘与图标

- Tray 图标风格：`trayIconStyle`
- 是否显示 tray 图标：`showTrayIcon`
- 单色 tray 图标：`useMonochromeTrayIcon`
- 反相单色 tray 图标：`useMonochromeInvertedTrayIcon`

### 休息窗口尺寸与模式

- 休息窗口宽度比例：`breakWindowWidth`
- 休息窗口高度比例：`breakWindowHeight`
- 以普通窗口显示休息：`showBreaksAsRegularWindows`

### 其他候选

- 自定义 preferences message：`customPreferencesMessage`
- 健康模式：`breakHealthMode`
- 地理位置：`posLatitude` / `posLongitude`

## 4. 不建议放进“设置”的运行时动作

这些在当前 Tauri 里是命令，不是设置。它们更适合放在 tray、快捷操作、快捷键或 break window CTA，而不是主设置页。

- 暂停提醒
- 恢复提醒
- 开启专注 session
- 清除专注 session
- 完成当前休息
- 跳过当前休息
- 延后当前休息
- 跳到下一个计划休息
- 跳到下一个微休息
- 跳到下一个长休息
- 重置节奏

## 5. 如果要继续做设置页，建议优先级

如果你想继续重做设置页，我建议优先把这些“已支持但没露出”的项加回来：

### 第一优先级

- 微休息 / 长休息延后次数上限
- 微休息 / 长休息手动结束
- 所有屏幕都显示
- 目标屏幕

### 第二优先级

- 休息窗口显示当前时间
- 严格模式下仍允许 tray menu
- 快捷键设置

### 第三优先级

- 主题 / 透明度 / 声音 / ideas / tray 图标 / 更新检查

第三优先级不是不重要，而是它们现在没有 Tauri 真源，先做会把任务从“重做前端设置页”升级成“恢复一批后端能力”。
