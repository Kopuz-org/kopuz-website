## Language

## Nav
nav-features = 功能
nav-download = 下载
nav-lang-label = 语言

## Hero
hero-cta-download = 下载
hero-cta-github = 在GitHub上查看
hero-screenshot-alt = Kopuz 主页

## Features
features-title = 功能
features-source-local = 本地文件
features-source-jellyfin = Jellyfin
features-source-navidrome = Navidrome
features-source-subsonic = Subsonic API
features-source-nextcloud = Nextcloud
features-source-ytmusic = YouTube Music
features-source-applemusic = Apple Music
features-source-soundcloud = SoundCloud

feat-theming-title = 主题
feat-native-title = 原生集成
feat-android-title = Android
feat-android-desc = 面向手机屏幕的 ARM64 版本，支持滑动手势、媒体通知中的随机播放和重复播放控制，以及 AMOLED 模式，适用于 Android 7.0 及更高版本。
feat-lyrics-title = 同步歌词
feat-lyrics-desc = 支持同步歌词、逐词定时歌词和纯文本歌词，并提供器乐间奏提示、当前歌词居中、景深模糊，以及手动或自动调整时间偏移。
feat-eq-title = 均衡器
feat-eq-desc = 5段均衡器,带内置预设,完全控制你的声音。
feat-fav-title = 收藏和播放列表
feat-scrobble-title = Scrobbling
feat-scrobble-desc = 内置ListenBrainz scrobbling。Jellyfin用户也可使用listenbrainz插件。
feat-radio-title = 互联网电台
feat-radio-desc = 通过电台目录播放互联网电台，并显示流媒体和当前播放曲目的元数据。
feat-offline-title = 离线下载
feat-offline-desc = 下载支持的远程曲目，无需网络连接也能收听。
feat-fonts-title = 自定义界面字体
feat-fonts-desc = 搭配内置主题，选择你喜欢的界面字体。
feat-discord-title = Discord RPC
feat-discord-desc = 通过嵌入式Discord Rich Presence向朋友展示你在听什么。无需设置。
feat-search-title = 搜索
feat-search-desc = 跨艺术家、专辑和曲目的实时搜索。输入时即时显示结果。
feat-genre-title = 按流派浏览
feat-genre-desc = 可按流派浏览本地文件和服务器音乐。
feat-logs-title = 收听日志
feat-logs-desc = 本地跟踪播放次数。查看你实际上最常听的内容。
feat-i18n-title = i18n
feat-channels-title = 声道模式
feat-channels-desc = 立体声、单声道、仅左、仅右和L/R交换。精细的音频声道控制。
feat-youtube-title = YouTube Music
feat-applemusic-title = Apple Music
feat-applemusic-desc = 浏览 Apple Music 曲库和你的音乐库、同步收藏、在播放列表中添加或移除曲目、启动电台、下载曲目，并使用逐词定时歌词。桌面端播放需要 Widevine。
feat-nextcloud-title = Nextcloud
feat-nextcloud-desc = 通过 WebDAV 连接，可为每个服务器分别选择音乐库文件夹、流式播放或下载曲目、缓存封面，并从文件头探测时长。
feat-metadata-title = 艺术家图片
feat-metadata-desc = 选择艺术家图片的来源方式:首张专辑封面(默认)或从你的Jellyfin或Subsonic服务器获取的真实艺术家照片,并自动回退。
feat-debug-title = 日志和崩溃报告
feat-debug-desc = 直接从设置中打开或导出日志,带有可在Speedscope或Perfetto中打开的可选性能跟踪。崩溃时自动写入报告。
feat-cleanup-title = 自动清理
feat-cleanup-desc = 重新扫描时,缺失或已删除的曲目会自动从你的音乐库中移除。没有幽灵条目。
feat-soundcloud-title = SoundCloud
feat-soundcloud-desc = 一次性浏览器登录后即可从SoundCloud流式传输。搜索、渐进式MP3和Go+ AAC/HLS播放、将你的Liked曲目作为收藏,以及只读播放列表。
feat-miniplayer-title = 迷你播放器
feat-miniplayer-desc = 一个紧凑的正在播放浮层,可从底部栏切换以获得更小的视图。
feat-tray-title = 最小化到托盘
feat-tray-desc = 关闭时缩小为系统托盘图标而非退出,以便播放在后台继续运行。可在设置中切换。
feat-badges-title = 文件类型徽章
feat-badges-desc = 本地曲目行会显示 MP3、FLAC、WAV 等格式徽章。

## What's new

## Install chooser

## Performance
perf-skip-label = 跳过已索引
perf-skip-desc = 重新扫描只处理新文件。10k曲目+5新=只读5个。
perf-parallel-label = 并行启动
perf-art-label = 专辑封面缓存
perf-art-desc = 封面提取一次,保存到磁盘。重复查看时永不重新解码。
perf-lazy-label = 懒加载图像
perf-lazy-desc = 专辑封面进入可视区域时加载。
perf-io-label = 非阻塞I/O
perf-http-label = HTTP封面缓存
perf-sort-label = 更智能的排序

## Install
install-nix-title = Nix / NixOS
install-nix-run = 无需安装即可运行:
install-nix-profile = 或添加到你的配置:
install-nix-note = NixOS flake通过Cachix二进制缓存支持。
install-aur-title = AUR (Arch Linux)
install-aur-desc = 使用你偏好的助手安装:
install-flatpak-title = Flatpak
install-flatpak-desc = 从 Flathub 安装 Kopuz：
install-flatpak-note = 你会通过常规 Flatpak 更新流程收到更新。
install-appimage-title = AppImage
install-appimage-desc-1 = 从GitHub Releases下载并直接运行。需要
install-appimage-desc-2 = { " " }和
install-appimage-desc-3 = { " " }在你的系统上。
install-appimage-note-1 = Arch用户:如果出现WebKitNetworkProcess错误崩溃,请使用以下前缀运行
install-appimage-note-2 = 。
install-macos-title = macOS
install-macos-desc-1 = 从GitHub Releases下载
install-macos-desc-2 = { " " }。如果macOS阻止,清除隔离标志:

## Feature guides
guides-title = 指南
## Privacy and storage

## Requirements

## Community
community-title = 参与 Kopuz 项目

## Platforms
platforms-title = 下载Kopuz
platforms-windows = Windows
platforms-macos = macOS
platforms-macos-note = 仅Apple Silicon
platforms-linux = Linux

## Support
support-title = 支持Kopuz
support-subtitle = Kopuz 是免费开源软件。捐款用于支持开发。
support-gh = GitHub Sponsors
support-bmc = Buy Me a Coffee
support-usdt-note = (Solana链)


## Nextcloud
nextcloud-subtitle = 在设置 → 媒体服务器 → 添加 → Nextcloud 中添加。
nextcloud-connect-title = 使用应用密码
nextcloud-connect-desc = 输入 Nextcloud 基础 URL、用户名，以及在 Nextcloud 设置 → 安全中创建、可随时撤销的应用密码。
nextcloud-folders-title = 选择音乐库文件夹
nextcloud-folders-desc = 可为每个来源选择一个或多个文件夹。Kopuz 会读取 WebDAV 文件夹树、缓存可用封面，并探测文件头以获取曲目时长。
nextcloud-webdav-title = 纯 WebDAV 模式
nextcloud-webdav-desc = 音乐库按文件夹结构呈现，不提供播放列表或电台功能。如果 Nextcloud Music 提供 Subsonic 接口，可改用 Subsonic 来源，以获得更丰富的元数据和播放列表。
nextcloud-playback-title = 流式播放或下载
nextcloud-playback-desc = 直接从你的服务器播放文件，或下载后离线收听；音频不会经过 Kopuz 的基础设施转发。

## YouTube Music
ytmusic-subtitle = 从设置 → 媒体服务器 → 添加 → YouTube Music 中添加。
ytmusic-token-title = 无需助手
ytmusic-signin-title = 通过浏览器登录
ytmusic-signin-desc = Kopuz 会在单独的浏览器配置文件中打开 Google 登录，并读取会话 Cookie。登录后可使用音乐库、Liked Music、播放列表和已关注的艺术家。
ytmusic-anon-title = 匿名模式
ytmusic-anon-desc = 无需登录,无需cookie。浏览、搜索、打开艺术家、专辑和播放列表页面、启动混合电台,以及播放公开曲目。收藏和音乐库视图已禁用。

## Apple Music
applemusic-subtitle = 在设置 → 媒体服务器 → 添加 → Apple Music 中添加。
applemusic-signin-title = 登录或粘贴令牌
applemusic-signin-desc = 在桌面端，Kopuz 会在隔离的 Chromium 系浏览器配置文件中打开 Apple Music。你也可以手动粘贴 media-user-token。
applemusic-playback-title = 桌面端的 Widevine
applemusic-playback-desc = Apple Music 的播放需要来自已安装浏览器或 Mozilla 插件服务的 Widevine CDM。Kopuz 不附带该 CDM。
applemusic-features-title = 可用功能
applemusic-features-desc = 浏览曲库和音乐库、同步收藏、在播放列表中添加或移除曲目、启动曲目电台和播放列表电台、下载曲目，以及使用逐词定时歌词。
applemusic-android-title = Android 支持情况
applemusic-android-desc = 可通过应用内 WebView 登录，但 Android 目前尚不支持播放 Apple Music。

## SoundCloud
soundcloud-subtitle = 从设置 → 媒体服务器 → 添加 → SoundCloud 中添加。
soundcloud-signin-title = 一次性浏览器登录
soundcloud-features-title = 你能获得什么
soundcloud-features-desc = 搜索、曲目播放(渐进式MP3加上Go+ AAC/HLS流)、将你的Liked曲目作为收藏、只读播放列表,以及点赞/取消点赞。移除该来源会清理其隔离的配置。

## Sponsors
sponsors-cta = 成为赞助者

## Gallery

## Footer
footer-license = EUPL v1.2 许可证（免费开源）
footer-github = GitHub
footer-releases = 发布
footer-issues = Issues
footer-discord = Discord

## Page meta
home-title = Kopuz:免费开源音乐播放器
og-title = Kopuz:免费开源音乐播放器
og-image-alt = Kopuz音乐播放器
twitter-title = Kopuz:免费开源音乐播放器
## User-friendly aliases (synced with EN updates)
perf-parallel-desc = { perf-parallel-desc-1 } { perf-parallel-desc-2 }
perf-io-desc = { perf-io-desc-1 } { perf-io-desc-2 }
perf-http-desc = { perf-http-desc-1 } { perf-http-desc-2 }
perf-sort-desc = { perf-sort-desc-1 } { perf-sort-desc-2 }

## Deep link (/j)
join-title = 正在打开 Kopuz…
join-opening = 正在打开 Kopuz…
join-fallback = Kopuz 没有打开。还没有安装吗？
join-no-payload = 此链接未附带播放队列。
join-download = 下载 Kopuz

## Privacy policy
footer-privacy = 隐私
privacy-english-note = 本政策以英文维护，以确保措辞准确。网站其余部分的翻译不会改变其内容。
privacy-back = 返回首页

## Home and hero
hero-title-1 = 一个播放器，
hero-title-2 = 播放你的所有音乐。
hero-desc = Kopuz 把你的本地文件、NAS、自己的服务器和流媒体账号放进同一个音乐库。它免费且开源。
hero-release = 最新版本
hero-cta-download-for = 下载：
hero-cta-all = 所有平台
home-meta-desc = Kopuz 是一款适用于桌面端和 Android 的免费音乐播放器。在同一个音乐库中播放本地文件、SMB 共享、Jellyfin、Subsonic、YouTube Music、Spotify 等。
home-meta-keywords = Kopuz, 音乐播放器, Jellyfin, Navidrome, Subsonic, SMB, NAS, Clippsly, Nextcloud, YouTube Music, Apple Music, SoundCloud, Spotify, 开源, Android, Linux, macOS, Windows, 歌词, 均衡器
og-desc = 一款免费的音乐播放器，支持桌面端和 Android 上的本地文件、服务器与流媒体服务，提供同步歌词、主题和 10 段均衡器。
twitter-desc = 在同一个音乐库中播放本地文件、SMB 共享、Jellyfin、Subsonic、YouTube Music、SoundCloud、Spotify 等。
home-sources-title = 可播放的来源
source-local = 本地文件夹
source-navidrome = Navidrome 和 Subsonic
source-radio = 互联网电台
source-smb = SMB 和 NAS
source-clippsly = Clippsly
moment-lyrics-title = 与节拍同步的歌词
moment-lyrics-alt = Kopuz 正在播放曲目，并显示全屏同步歌词
moment-library-title = 所有来源，一个音乐库
moment-library-alt = 包含多个来源专辑的 Kopuz 音乐库
moment-themes-title = 用你的配色，或用壁纸的配色
moment-themes-alt = Kopuz 主题编辑器
moment-themes-extra = 也可以让 Kopuz 根据桌面壁纸生成主题。
theme-default = Gruvbox dark soft
theme-amoled = AMOLED
theme-wallpaper = 壁纸
wallpaper-sunset = 日落
wallpaper-lake = 湖泊
home-see-all-features = 查看所有功能
home-get-title = 获取 Kopuz
home-get-download = 下载
home-also-on = 你也可以通过 Cargo、Nix、AUR、Flatpak 或 Homebrew 安装 Kopuz。
home-sponsors-title = 赞助者
home-sponsor-cta = 在 GitHub 上赞助

## Common UI
nav-home = 首页
nav-primary-aria = 主导航
nav-menu-open = 打开菜单
nav-menu-close = 关闭菜单
copy = 复制
copied = 已复制
copy-address = 复制 { $coin } 地址
footer-nav-aria = 页脚
footer-license-line = EUPL 1.2，免费开源
footer-made-with = 网站制作
footer-love = 爱
footer-made-by = 作者：
shelf-link-to-us = 用 88×31 按钮链接到我们
webbutton-alt = Kopuz 88x31 网站按钮
theme-use-light = 使用浅色主题
theme-use-dark = 使用深色主题
theme-leave-moe = 退出萌模式
mode-simple = 简洁站点
mode-full = 完整站点

## Page titles
features-page-title = Kopuz 功能:音乐源、歌词、主题与播放
download-page-title = 下载 Kopuz:支持 Windows、macOS、Linux 和 Android
guides-page-title = Kopuz 设置指南:服务器与流媒体服务
support-page-title = 支持 Kopuz:赞助、捐赠与社区
notfound-page-title = 找不到页面 | Kopuz
guide-title-suffix = 设置指南 | Kopuz
notfound-title = 找不到页面。
notfound-desc = 这个地址上没有正在播放的内容。
notfound-home = 前往首页

## Features
features-group-sources = 来源
features-group-playback = 播放
features-group-library = 音乐库
features-group-desktop = 桌面端
features-group-android = Android
features-group-integrations = 集成
features-group-performance = 性能
features-group-privacy = 隐私
features-jump-aria = 功能分组
features-privacy-link = 阅读隐私政策
features-source-spotify = Spotify
features-source-smb = SMB 和 NAS
features-source-clippsly = Clippsly
feat-local-title = 文件夹和服务器
feat-local-desc = 添加本地文件夹，或连接 Jellyfin 或 Subsonic 服务器，例如 Navidrome。你添加的每个来源中的曲目都会显示在同一个音乐库中，Subsonic 还会在各个客户端之间同步你的播放队列。
feat-theming-desc = 从 24 个内置的浅色和深色主题中选择，或创建自己的主题并设置每一种颜色。
feat-native-desc = 用系统的媒体键和控件控制播放：Linux 上是 MPRIS，macOS 上是“正在播放”，Windows 上是媒体浮层。
feat-fav-desc = 把收藏保存在设备上，或与服务器同步。创建播放列表，一次添加整张专辑，并把播放列表同步到服务器。
feat-i18n-desc = Kopuz 支持 28 种语言。
feat-ytdlp-title = YouTube 下载器
feat-ytdlp-desc = 粘贴歌曲、专辑或播放列表的 YouTube 或 YouTube Music 链接。Kopuz 用自带的 YouTube 客户端下载，写入标题、艺术家、专辑和封面标签，并按专辑归档。“最佳音质”保留原始音频流，无需安装任何组件。如需转换为 MP3、FLAC 或 WAV，请安装 ffmpeg。
feat-crossfade-title = 淡入淡出和无缝播放
feat-crossfade-desc = 在桌面端播放时，让曲目之间淡入淡出；播放需要无缝衔接的专辑时，可以关闭淡入淡出。
feat-youtube-desc = 浏览、搜索，并从任意曲目开始混合电台。“发现”会推荐歌曲、专辑、艺术家和心情，艺术家页面会显示热门歌曲、发行作品和相关艺术家。登录后可使用你的音乐库、喜欢的音乐和播放列表，也可以不登录直接浏览。
feat-spotify-title = Spotify
feat-spotify-desc = 浏览你保存的曲目、专辑、播放列表和“发现”，还可以搜索和 scrobble。通过 Spotify 的 Web Playback SDK 播放，或在任意 Spotify Connect 设备上播放。播放音乐需要 Spotify Premium。
feat-smb-title = SMB 和 NAS
feat-smb-desc = 把 SMB 2 或 3 共享，或其中的某个文件夹，添加为音乐库。Kopuz 会读取内嵌标签和封面，并直接从共享中播放，支持拖动进度，因此无需挂载。
feat-clippsly-title = Clippsly
feat-clippsly-desc = 使用你的 Clippsly 用户名和在 clippsly.app 创建的应用密码登录。你可以从存储空间串流和下载、上传音频文件，并查看存储用量。暂不支持远程播放列表。
feat-updates-title = 自动更新
feat-updates-desc = 在桌面端，新版本发布时会弹出横幅提醒你，Kopuz 会为你下载并安装。
feat-cli-title = 守护进程和命令行
feat-cli-desc = 播放器作为守护进程 kopuzd 运行，窗口会连接到它。你可以在终端或脚本中用 kopuzctl 控制播放、设置来源和启动音乐库扫描，也可以直接调用 gRPC API。

## What's new, install, requirements
new-title = Kopuz 的新变化
install-other-title = 其他安装方式
install-cargo-title = Cargo (crates.io)
install-cargo-desc = 用 Cargo 安装最新版本：
install-macos-homebrew = 从 Homebrew tap 安装 Apple silicon 版本：
install-android-title = Android
install-android-desc = 从 Google Play 获取 Kopuz，或从 GitHub Releases 下载已签名的 arm64-v8a APK。需要 Android 7.0 (API 24) 或更高版本。
install-android-note = 如果你已安装的 APK 来自其他渠道，请先卸载，再安装这一个。
install-android-play = 在 Google Play 上获取
req-spotify = 播放音乐需要 Premium、你自己的 Client ID 和受支持的浏览器。
req-applemusic = 桌面端播放需要 Widevine CDM。你可以在 Android 上登录，但目前还无法播放。
req-nextcloud = 普通 WebDAV 不支持播放列表和电台。如果 Nextcloud Music 提供 Subsonic 端点，请改用它。
req-android = 发行版 APK 需要 arm64-v8a 手机和 Android 7.0 (API 24) 或更高版本。不支持 Discord 状态和系统托盘。
req-appimage = 需要 webkit2gtk-4.1 和 GTK 3。托盘还需要 appindicator 库。
req-ytmusic = 匿名模式无法播放 Premium 专属曲目。请使用 Music Premium 账号登录后播放。
req-crossfade = 淡入淡出和无缝播放适用于桌面端原生播放，不适用于在浏览器中播放的 Spotify 音频。
req-spotify-limits-label = Spotify 限制
req-spotify-limits = 在 Development Mode 下，一个应用最多允许五位听众，搜索每种类型最多返回十条结果，播放列表为只读，并且下载、电台、标签编辑、均衡器和淡入淡出均不可用。
requirements-title = 要求和限制

## YouTube Music, SoundCloud, Spotify
ytmusic-token-desc = 不登录也可以播放，也无需安装辅助程序。
ytmusic-signin-note = Kopuz 使用你的默认浏览器。你也可以改选 Firefox、Chrome、Chromium、Brave、Edge、Vivaldi、Helium、LibreWolf、Zen 或 Floorp。
ytmusic-premium-title = 高级音质
ytmusic-premium-desc = 使用 Music Premium 账号时，Kopuz 会通过自带的 YouTube 客户端播放和下载更高音质的音频流。匿名模式无法播放 Premium 专属曲目。
soundcloud-signin-desc = 你不需要 URL、密码或令牌。Kopuz 会在单独的浏览器配置文件中打开 SoundCloud 登录页面，并在你移除来源时删除该配置文件。
spotify-guide-subtitle = Kopuz 使用 Spotify 官方的 Web API 和浏览器播放。它不会看到你的密码，也不会转发音频。
spotify-step-1-title = 创建 Spotify 应用
spotify-step-1-desc = 在 Spotify 开发者控制台中创建应用，并开启 Web API 和 Web Playback SDK。在 User Management 下添加每个要收听的账号（Development Mode 下最多五个）。然后原样设置以下重定向 URI：
spotify-step-2-title = 添加 Client ID
spotify-step-2-desc = 在 设置 → 媒体服务器 → 添加 → Spotify 中，粘贴 Client ID 并批准访问。你不需要 Client Secret。登录期间，8898 端口必须空闲。
spotify-step-3-title = 选择播放位置
spotify-step-3-desc = 保持应用内播放器标签页打开，或在设备菜单中选择你的某个 Spotify Connect 设备。
spotify-requirement = 播放音乐需要 Spotify Premium。请使用 Chrome、Edge、Brave、Chromium、Vivaldi、Helium，或在 macOS 上使用 Safari。Firefox 不可用。
spotify-full-guide = 阅读完整的 Spotify 设置与故障排除指南

## Privacy
privacy-local-title = 一个本地数据库
privacy-local-desc = Kopuz 把你的音乐库、播放列表、收藏、播放次数和来源数据保存在 kopuz.db 中，把你的偏好设置保存在它旁边的 settings.toml 中。
privacy-accounts-title = 各自独立的登录
privacy-accounts-desc = 每个来源都有自己的登录和收藏。浏览器登录使用单独的配置文件，而不是你日常使用的浏览器会话。
privacy-files-title = 缓存和日志
privacy-files-desc = 专辑封面和离线曲目保存在系统缓存文件夹中。你可以在设置中打开或导出日志。

## About, community, support
about-title = kopuz 是什么？
about-desc-1 = kopuz 是一种古老的突厥弦乐器，吟游诗人和萨满都曾演奏它。人们认为它是几种中亚鲁特琴的祖先。
about-desc-2 = 在突厥传说中，它属于吟游诗人 Dede Korkut。吉尔吉斯的 komuz 和哈萨克的 kobyz 是相关的乐器。xomus 名字相似，却并不相关。
community-issues-title = Issues
community-issues-desc = 报告问题，或认领一项未完成的任务。
community-discussions-title = Discussions
community-discussions-desc = 提出问题，建议改动。
community-discord-desc = 与用户和贡献者聊天。
community-contribute-title = 参与贡献
community-contribute-desc = 获取源代码，搭建开发环境，或帮忙翻译。
platforms-android = Android
platforms-android-note = ARM64 · Android 7.0+ · API 24+
support-note = 我是 Temidaradev，Kopuz 由我开发。我是一名没有稳定收入的学生，正是赞助者让我在读书期间还能继续开发 Kopuz。
support-goal-label = GitHub Sponsors 目标
support-goal-amount = 每月 ${ $goal } 中的 ${ $current }
support-goal-aria = GitHub Sponsors 目标进度
support-crypto-summary = 加密货币地址
sponsors-tier-special = 特别
sponsors-tier-monthly = 每月
sponsors-tier-one-time = 一次性

## Player and downloads
player-prev-label = 上一节
player-next-label = 下一节
player-seek-label = 页面中的位置
player-queue-label = 本页各节
player-queue-title = 本页内容
dl-file-exe = 安装程序 (.exe)
dl-file-msi = 安装包 (.msi)
dl-file-dmg = 磁盘映像 (.dmg)
dl-file-appimage = AppImage
dl-file-deb = Debian 和 Ubuntu (.deb)
dl-file-rpm = Fedora 和 RHEL (.rpm)
dl-file-apk = APK
dl-release-page = 发行页面
dl-this-device = 此设备
dl-dialog-ready = 最新版本：{ $version }。请选择适合你设备的安装包。
dl-dialog-close = 关闭
dl-dialog-release = 在 GitHub 上查看发行版本

## Guides
guide-nextcloud-summary = 使用应用密码通过 WebDAV 连接，然后选择你的音乐库文件夹。
guide-ytmusic-summary = 通过浏览器登录，或不登录直接收听。
guide-applemusic-summary = 登录，在桌面端启用 Widevine，并了解 Android 上可用的功能。
guide-soundcloud-summary = 在浏览器中登录一次，并了解该来源支持的功能。
guide-spotify-summary = 创建开发者应用，添加它的 Client ID，并选择播放位置。
guide-smb-summary = 让 Kopuz 指向 NAS 上的共享，并使用你的 NAS 账号登录。
smb-subtitle = 在来源选择器中选择 SMB / NAS。你不需要先挂载共享。
smb-share-title = 指向一个共享
smb-share-desc = 输入共享的 URL，或其中某个文件夹的 URL，例如：
smb-share-port = 如果你的 NAS 使用默认端口以外的端口，请把端口添加到 URL 中：
smb-signin-title = 使用 NAS 账号登录
smb-signin-desc = 使用你的 NAS 用户名和密码。域账号请输入 DOMAIN\username。Kopuz 需要 SMB 2 或 3 和一个账号：它不会在网络中搜索共享，也不能以访客身份登录。
smb-library-title = 音乐库如何工作
smb-library-desc = Kopuz 会扫描共享及其子文件夹中的音频文件，并读取内嵌标签和封面。它直接从共享中播放，支持拖动进度，在 Android 上也一样。你的收藏保存在 Kopuz 中。
smb-limits-title = 共享保持只读
smb-limits-desc = Kopuz 不会更改共享上的任何内容，因此你无法在共享上编辑标签、保存播放列表，也无法下载其中的曲目用于离线收听。
guide-clippsly-summary = 使用应用密码登录，串流你的存储空间，并上传音乐。
clippsly-subtitle = 在来源设置中添加 Clippsly。
clippsly-signin-title = 使用应用密码登录
clippsly-signin-desc = 在 clippsly.app 创建一个应用密码。然后用你的 Clippsly 用户名和这个应用密码登录 Kopuz。
clippsly-locker-title = 你的存储空间
clippsly-locker-desc = 浏览并串流 Clippsly 存储空间中的音乐，也可以下载曲目以离线播放。
clippsly-upload-title = 上传音乐
clippsly-upload-desc = 在来源的设置卡片中上传音频文件。整批文件完成后，Kopuz 会刷新你的音乐库。卡片还会显示你已使用的存储空间，配额由 Clippsly 设定。
clippsly-bonus = 把 Kopuz 关联到你的 Clippsly 账号，Clippsly 会免费增加 3 GB 存储空间。
clippsly-limits-title = 播放列表
clippsly-limits-desc = 你保存在 Clippsly 上的播放列表暂时不会显示在 Kopuz 中。
clippsly-site = 前往 clippsly.app

## Chapters
chapter-home = 首页
chapter-sources = 来源
chapter-lyrics = 歌词
chapter-library = 音乐库
chapter-themes = 主题
chapter-download = 下载
chapter-sponsors = 赞助者
chapter-whats-new = 新变化
chapter-name = 名字
chapter-not-found = 未找到
chapter-privacy = 隐私
chapter-install = 安装
chapter-requirements = 要求
chapter-support = 支持
chapter-community = 社区
chapter-features = 功能
chapter-guides = 指南
