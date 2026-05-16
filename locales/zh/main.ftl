## Language
lang-name = 中文

## Nav
nav-new = 新
nav-announce = 现代UI、Vaxry主题、yt-dlp下载器、20+种语言
nav-features = 功能
nav-install = 安装
nav-download = 下载
nav-sponsors = 赞助者
nav-github = GitHub ↗
nav-gallery = 画廊
nav-lang-label = 语言

## Hero
hero-title-1 = 你的音乐。
hero-title-2 = 你的方式。
hero-desc = Kopuz是用Rust和Dioxus构建的现代轻量级音乐播放器。扫描本地文件夹、从Jellyfin或Navidrome流式传输、按你喜欢的方式浏览音乐库。
hero-cta-download = 下载
hero-cta-github = 在GitHub上查看
hero-screenshot-alt = Kopuz — 主页视图

## Features
features-title = 你需要的一切。
features-chip = 无订阅。无追踪。只有你的音乐。
features-works-with = 支持
features-source-local = 本地文件
features-source-jellyfin = Jellyfin
features-source-navidrome = Navidrome
features-source-subsonic = Subsonic API

feat-local-title = 本地 + 流媒体
feat-local-desc = 指向本地文件夹或连接到Jellyfin / Subsonic (Navidrome)。随意混用。
feat-theming-title = 主题
feat-theming-desc = 完全色彩变量控制的动态主题。从零构建自己的主题或选择预设。
feat-native-title = 原生集成
feat-native-desc = Linux上的MPRIS、macOS上的Now Playing、Windows上的System Media Transport。完全连接。
feat-lyrics-title = 同步歌词
feat-lyrics-desc = 实时滚动歌词,同步或纯文本,跟随音乐自动滚动。
feat-eq-title = 均衡器
feat-eq-desc = 5段均衡器,带内置预设,完全控制你的声音。
feat-fav-title = 收藏和播放列表
feat-fav-desc = 本地标记曲目或与服务器同步收藏。创建播放列表,一次性添加整个专辑。
feat-scrobble-title = Scrobbling
feat-scrobble-desc = 内置ListenBrainz scrobbling。Jellyfin用户也可使用listenbrainz插件。
feat-discord-title = Discord RPC
feat-discord-desc = 通过嵌入式Discord Rich Presence向朋友展示你在听什么。无需设置。
feat-search-title = 搜索
feat-search-desc = 跨艺术家、专辑和曲目的实时搜索。输入时即时显示结果。
feat-genre-title = 按流派浏览
feat-genre-desc = 按流派浏览整个音乐库 — 适用于本地文件和服务器音乐。
feat-logs-title = 收听日志
feat-logs-desc = 本地跟踪播放次数。查看你实际上最常听的内容。
feat-i18n-title = i18n
feat-i18n-desc = 英语、俄语、德语、法语、西班牙语、土耳其语、乌克兰语、波兰语、阿拉伯语、希腊语、希伯来语、匈牙利语、印尼语、日语、韩语、罗马尼亚语、葡萄牙语、中文等。
feat-ytdlp-title = yt-dlp下载器
feat-ytdlp-desc = 直接从YouTube下载音频。输出为MP3、FLAC、WAV、MP4或最佳质量。包含SponsorBlock、章节分割和速率限制。
feat-crossfade-title = 淡入淡出和过渡
feat-crossfade-desc = 混合曲目过渡以获得更流畅的播放。原生桌面构建支持淡入淡出。
feat-channels-title = 声道模式
feat-channels-desc = 立体声、单声道、仅左、仅右和L/R交换。精细的音频声道控制。

## Performance
perf-title = 为速度而生。
perf-subtitle = 大型音乐库。瞬时启动。无卡顿。
perf-skip-label = 跳过已索引
perf-skip-desc = 重新扫描只处理新文件。10k曲目+5新=只读5个。
perf-parallel-label = 并行启动
perf-parallel-desc-1 = 音乐库、配置、播放列表和收藏全部并行加载,使用
perf-parallel-desc-2 = 。几乎瞬时打开。
perf-art-label = 专辑封面缓存
perf-art-desc = 封面提取一次,保存到磁盘。重复查看时永不重新解码。
perf-lazy-label = 懒加载图像
perf-lazy-desc = 搜索结果中数百张专辑封面 — 在可见之前不加载任何一张。
perf-io-label = 非阻塞I/O
perf-io-desc-1 = 重型工作运行在
perf-io-desc-2 = { " " }线程上。完整音乐库扫描期间UI保持响应。
perf-http-label = HTTP封面缓存
perf-http-desc-1 = 自定义
perf-http-desc-2 = { " " }协议以1年缓存头提供封面。Webview从不重新获取。

## Install
install-title = 安装
install-nix-title = Nix / NixOS
install-nix-run = 无需安装即可运行:
install-nix-profile = 或添加到你的配置:
install-nix-note = NixOS flake通过Cachix二进制缓存支持。
install-flatpak-title = Flatpak
install-flatpak-desc = 从源清单安装:
install-flatpak-note = Flathub列表即将推出。
install-appimage-title = AppImage
install-appimage-desc-1 = 从GitHub Releases下载并直接运行。需要
install-appimage-desc-2 = { " " }和
install-appimage-desc-3 = { " " }在你的系统上。
install-appimage-note-1 = Arch用户:如果出现WebKitNetworkProcess错误崩溃,请使用以下前缀运行
install-appimage-note-2 = 。
install-macos-title = macOS
install-macos-chip = Apple Silicon
install-macos-desc-1 = 从GitHub Releases下载
install-macos-desc-2 = { " " }。如果macOS阻止,清除隔离标志:

## Platforms
platforms-title = 下载Kopuz
platforms-subtitle = 免费且开源。所有版本都在GitHub上。
platforms-windows = Windows
platforms-macos = macOS
platforms-macos-note = 仅Apple Silicon
platforms-linux = Linux
platforms-download = 下载 →

## Support
support-title = 支持Kopuz
support-subtitle = Kopuz是免费且开源的。支持让它得以存续。
support-gh = GitHub Sponsors
support-bmc = Buy Me a Coffee
support-crypto-divider = — 或发送加密货币 —
support-usdt-note = (Solana链)

## Sponsors
sponsors-title = 赞助者
sponsors-subtitle = 让Kopuz持续运转的人。
sponsors-cta = 成为赞助者

## Gallery
gallery-page-title = 画廊 — Kopuz Music Player
gallery-page-desc = Kopuz运行截图 — 主页、音乐库、播放列表、全屏播放器、歌词、主题编辑器等。
gallery-title = 画廊
gallery-subtitle = Kopuz截图 — Normal和Modern风格。
gallery-home = 主页
gallery-home-styles = Normal / Modern
gallery-library = 音乐库
gallery-library-styles = Normal / Vaxry
gallery-playlist = 播放列表
gallery-playlist-styles = Normal / Modern
gallery-fullscreen-title = 全屏播放器
gallery-fullscreen-desc = 沉浸式全窗口播放视图。
gallery-fullscreen-lyrics-title = 全屏歌词
gallery-fullscreen-lyrics-desc = 全窗口模式下的同步歌词。
gallery-search-title = 搜索
gallery-search-desc = 跨艺术家、专辑和曲目的实时搜索。
gallery-theme-title = 主题编辑器
gallery-theme-desc = 完全色彩变量控制。构建或选择预设。
gallery-settings-title = 播放器设置
gallery-settings-desc = 配置音频、行为、集成等。
gallery-downloader-title = 下载器
gallery-downloader-desc = 从Kopuz内部直接下载曲目。
gallery-prev = ← 上一个
gallery-next = 下一个 →
gallery-label-normal-home = Normal — 主页
gallery-label-modern-home = Modern — 主页
gallery-label-normal-library = Normal — 音乐库
gallery-label-vaxry-library = Vaxry — 音乐库
gallery-label-normal-playlist = Normal — 播放列表
gallery-label-modern-playlist = Modern — 播放列表
gallery-label-fullscreen = 全屏播放器
gallery-label-fullscreen-lyrics = 全屏歌词
gallery-label-search = 搜索
gallery-label-theme-editor = 主题编辑器
gallery-label-player-settings = 播放器设置
gallery-label-downloader = 下载器

## Footer
footer-license = MIT许可证 — 免费且开源
footer-github = GitHub
footer-releases = 发布
footer-issues = Issues
footer-discord = Discord

## Page meta
home-title = Kopuz — Music Player
home-meta-desc = Kopuz是用Rust和Dioxus构建的现代轻量级音乐播放器。从Jellyfin或Navidrome流式传输、浏览本地文件、同步歌词、均衡器、主题等。
home-meta-keywords = Kopuz, 音乐播放器, Rust, Dioxus, Jellyfin, Navidrome, 开源, Linux, macOS, Windows, MPRIS, 歌词
og-title = Kopuz — Music Player
og-desc = 用Rust构建的现代轻量级音乐播放器。本地文件、Jellyfin、Navidrome、同步歌词、均衡器、Discord RPC等。免费且开源。
og-image-alt = Kopuz音乐播放器
twitter-title = Kopuz — Music Player
twitter-desc = 用Rust构建的现代轻量级音乐播放器。免费且开源。
