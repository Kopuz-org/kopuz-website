## Language
lang-name = English

## Nav
nav-new = New
nav-announce = YouTube Music & SoundCloud streaming, Discover, mix radio, 20+ languages
nav-features = Features
nav-install = Install
nav-download = Download
nav-sponsors = Sponsors
nav-github = GitHub ↗
nav-gallery = Gallery
nav-lang-label = Language

## Hero
hero-title-1 = Your music.
hero-title-2 = Your way.
hero-desc = Kopuz is a modern, lightweight music player application built with Rust and the Dioxus framework. Scan local folders, stream from Jellyfin or Navidrome, connect YouTube Music, and organize your library the way you want.
hero-cta-download = Download
hero-cta-github = View on GitHub
hero-screenshot-alt = Kopuz — home view

## Features
features-title = Everything you need.
features-chip = No subscriptions. No tracking. Just your music.
features-works-with = Works with
features-source-local = Local files
features-source-jellyfin = Jellyfin
features-source-navidrome = Navidrome
features-source-subsonic = Subsonic API
features-source-ytmusic = YouTube Music
features-source-soundcloud = SoundCloud

feat-local-title = Local + Streaming
feat-local-desc = Point at a local folder or connect to Jellyfin / Subsonic (Navidrome). Mix and match as you like.
feat-theming-title = Theming
feat-theming-desc = Dynamic theming with full color variable control. Build your own theme from scratch or pick a preset.
feat-native-title = Native Integration
feat-native-desc = MPRIS on Linux, Now Playing on macOS, System Media Transport on Windows. Fully wired up.
feat-lyrics-title = Synced Lyrics
feat-lyrics-desc = Real-time scrolling lyrics, synced or plain, auto-scrolling to follow along with your music.
feat-eq-title = Equalizer
feat-eq-desc = 5-band equalizer with built-in presets and full custom control over your sound.
feat-fav-title = Favorites & Playlists
feat-fav-desc = Star tracks locally or sync favorites with your server. Create playlists, add whole albums at once.
feat-scrobble-title = Scrobbling
feat-scrobble-desc = ListenBrainz scrobbling built in. Jellyfin users can also use the listenbrainz plugin.
feat-discord-title = Discord RPC
feat-discord-desc = Show friends what you're listening to with embedded Discord Rich Presence. No setup needed.
feat-search-title = Search
feat-search-desc = Real-time search across artists, albums, and tracks. Instant results as you type.
feat-genre-title = Genre Browsing
feat-genre-desc = Browse your entire library by genre — works for both local files and server music.
feat-logs-title = Listening Logs
feat-logs-desc = Play counts tracked locally. See what you actually listen to most over time.
feat-i18n-title = i18n
feat-i18n-desc = English, Russian, German, French, Spanish, Turkish, Ukrainian, Polish, Arabic, Greek, Hebrew, Hungarian, Indonesian, Japanese, Korean, Romanian, Brazilian Portuguese, Toki Pona, Simplified Chinese, and more.
feat-ytdlp-title = yt-dlp Downloader
feat-ytdlp-desc = Download audio directly from YouTube. Output as MP3, FLAC, WAV, MP4, or best quality. SponsorBlock, chapter splitting, and rate limiting included.
feat-crossfade-title = Crossfade & Transitions
feat-crossfade-desc = Blend track transitions for smoother playback. Crossfade support on native desktop builds.
feat-channels-title = Channel Modes
feat-channels-desc = Stereo, Mono, Left-only, Right-only, and L/R swap. Fine-grained audio channel control.
feat-youtube-title = YouTube Music
feat-youtube-desc = Full streaming backend with a Spotify-style Discover page, rich artist profiles, and mix radio. Sign in for your library, Liked Music, and playlists — or browse anonymously.
feat-metadata-title = Artist Images
feat-metadata-desc = Choose how artist images are sourced: first album cover (default) or real artist photos fetched from your Jellyfin or Subsonic server, with automatic fallback.
feat-debug-title = Logs & Crash Reports
feat-debug-desc = Open or export logs straight from Settings, with an optional performance trace you can open in Speedscope or Perfetto. Crashes write a report automatically.
feat-cleanup-title = Auto-Cleanup
feat-cleanup-desc = Missing or deleted tracks are removed from your library automatically when rescanning. No ghost entries.
feat-soundcloud-title = SoundCloud
feat-soundcloud-desc = Stream from SoundCloud after a one-time browser sign-in. Search, progressive MP3 and Go+ AAC/HLS playback, your Liked tracks as favorites, and read-only playlists.
feat-miniplayer-title = Mini-Player
feat-miniplayer-desc = A compact now-playing overlay you can toggle from the bottom bar for a smaller view.
feat-tray-title = Minimize to Tray
feat-tray-desc = Close to a system tray icon instead of quitting, so playback keeps running in the background. Toggle in Settings.
feat-badges-title = File-Type Badges
feat-badges-desc = Local tracks show a small format badge — MP3, FLAC, WAV, and more — right in the track row.

## Performance
perf-title = Built to be fast.
perf-subtitle = Large libraries. Instant startup. No freezes.
perf-skip-label = Skip already indexed
perf-skip-desc = Rescans only process new files. 10k tracks + 5 new = only 5 read.
perf-parallel-label = Parallel startup
perf-parallel-desc-1 = Library, config, playlists, and favorites all load in parallel with
perf-parallel-desc-2 = . Near-instant open.
perf-art-label = Album art caching
perf-art-desc = Covers extracted once, saved to disk. Never re-decoded on repeat views.
perf-lazy-label = Lazy image loading
perf-lazy-desc = Hundreds of album covers in search results — none load until they're in view.
perf-io-label = Non-blocking I/O
perf-io-desc-1 = Heavy work runs on
perf-io-desc-2 = { " " }threads. UI stays responsive during full library scans.
perf-http-label = HTTP art caching
perf-http-desc-1 = Custom
perf-http-desc-2 = { " " }protocol serves covers with 1-year cache headers. Webview never re-fetches.
perf-sort-label = Smarter sorting
perf-sort-desc-1 = Library views sort with
perf-sort-desc-2 = { " " }so sort keys are computed once, not on every comparison.

## Install
install-title = Installation
install-nix-title = Nix / NixOS
install-nix-run = Run without installing:
install-nix-profile = Or add to your profile:
install-nix-note = NixOS flake supported with Cachix binary cache.
install-aur-title = AUR (Arch Linux)
install-aur-desc = Install with your preferred helper:
install-aur-note-1 = Requires
install-aur-note-2 = { " " }installed first at a version matching dioxus 0.7.x.
install-flatpak-title = Flatpak
install-flatpak-desc = Install from source manifest:
install-flatpak-note = Flathub listing coming soon.
install-appimage-title = AppImage
install-appimage-desc-1 = Download from GitHub Releases and run directly. Requires
install-appimage-desc-2 = { " " }and
install-appimage-desc-3 = { " " }on your system.
install-appimage-note-1 = Arch users: if it crashes with a WebKitNetworkProcess error, run with
install-appimage-note-2 = { " " }prefix.
install-macos-title = macOS
install-macos-chip = Apple Silicon
install-macos-desc-1 = Download the
install-macos-desc-2 = { " " }from GitHub Releases. If macOS blocks it, clear the quarantine flag:

## YouTube Music
ytmusic-title = YouTube Music Setup
ytmusic-subtitle = Add it from Settings → Media servers → Add → YouTube Music.
ytmusic-token-title = No helper needed
ytmusic-token-desc-1 = Anonymous playback needs a content PO token, which Kopuz now mints in-app with a hidden WebView running YouTube's BotGuard. The old
ytmusic-token-desc-2 = { " " }subprocess is gone — nothing to install, and it works inside Flatpak.
ytmusic-signin-title = Sign in with a browser
ytmusic-signin-desc = Kopuz opens Google sign-in in an isolated browser profile — your normal browsing is never touched — and extracts the session cookies. Unlocks your library, Liked Music, playlists, and followed artists.
ytmusic-signin-note = On Windows, browser sign-in is currently disabled; Windows users get anonymous mode automatically. Sign-in works on Linux and macOS.
ytmusic-anon-title = Anonymous mode
ytmusic-anon-desc = No sign-in, no cookies. Browse, search, open artist, album, and playlist pages, start mix radio, and play public tracks. Liking and library views are disabled.
ytmusic-premium-title = Premium tracks
ytmusic-premium-desc-1 = Music Premium-locked tracks fall back to a local
ytmusic-premium-desc-2 = { " " }resolve when the primary path returns UNPLAYABLE, so having it installed helps. Anonymous mode can't play Premium-only content at all.

## SoundCloud
soundcloud-title = SoundCloud Setup
soundcloud-subtitle = Add it from Settings → Media servers → Add → SoundCloud.
soundcloud-signin-title = One-time browser sign-in
soundcloud-signin-desc = No URL or password to type. Kopuz opens soundcloud.com/signin in an isolated browser profile — your normal browsing is never touched — and pulls the session's oauth_token. Pick which Chromium-family browser to use (Chrome, Chromium, Brave, Edge, or Vivaldi).
soundcloud-features-title = What you get
soundcloud-features-desc = Search, track playback (progressive MP3 plus Go+ AAC/HLS streams), your Liked tracks as favorites, read-only playlists, and like/unlike. Removing the source cleans up its isolated profile.


## Platforms
platforms-title = Download Kopuz
platforms-subtitle = Free and open source. All releases on GitHub.
platforms-windows = Windows
platforms-macos = macOS
platforms-macos-note = Apple Silicon only
platforms-linux = Linux
platforms-download = Download →

## Support
support-title = Support Kopuz
support-subtitle = Kopuz is free and open source. Support keeps it alive.
support-gh = GitHub Sponsors
support-bmc = Buy Me a Coffee
support-crypto-divider = — or send crypto —
support-usdt-note = (Solana chain)

donate-title = Crypto Donations
donate-subtitle = Support development by sending crypto to these addresses.
donate-solana = Solana: 2fapJYRztnTRLpJbmyEUnsuZ36AzLK2JrMmmLEfDqKpN
donate-bitcoin = Bitcoin: bc1qz94yz9xvufa6hxlvjzaajgd2zyfu86arn68hu4
donate-monero = Monero: 86mz3HxTrKyYpuvx78m6pufbXdwAnoyoZBztz6HyYrnM1XP5YVrMy9jTVRY5vzgGtkizACLpFwHEdafKTMoj6y8mAVgvWMz
donate-ethereum = Ethereum: 0xa490D50470cdFf837B6663F7f6cBe50B157224e5
donate-usdt-sol = USDT (Solana): GYmnAcrA5MbF6cUxT2m5d5cwdfr14qSY9WFYRwXxaibW

## Sponsors
sponsors-title = Sponsors
sponsors-subtitle = People keeping Kopuz going.
sponsors-cta = Become a Sponsor

## Gallery
gallery-page-title = Gallery — Kopuz Music Player
gallery-page-desc = Screenshots of Kopuz in action — home, library, playlist, fullscreen player, lyrics, theme editor, and more.
gallery-title = Gallery
gallery-subtitle = Screenshots of Kopuz — Normal and Modern styles.
gallery-home = Home
gallery-home-styles = Normal / Modern
gallery-library = Library
gallery-library-styles = Normal / Vaxry
gallery-playlist = Playlist
gallery-playlist-styles = Normal / Modern
gallery-fullscreen-title = Fullscreen player
gallery-fullscreen-desc = Immersive full-window playback view.
gallery-fullscreen-lyrics-title = Fullscreen lyrics
gallery-fullscreen-lyrics-desc = Synced lyrics in full-window mode.
gallery-search-title = Search
gallery-search-desc = Real-time search across artists, albums, and tracks.
gallery-theme-title = Theme editor
gallery-theme-desc = Full color variable control. Build or pick a preset.
gallery-settings-title = Player settings
gallery-settings-desc = Configure audio, behavior, integrations, and more.
gallery-downloader-title = Downloader
gallery-downloader-desc = Download tracks directly from within Kopuz.
gallery-prev = ← Prev
gallery-next = Next →
gallery-label-normal-home = Normal — Home
gallery-label-modern-home = Modern — Home
gallery-label-normal-library = Normal — Library
gallery-label-vaxry-library = Vaxry — Library
gallery-label-normal-playlist = Normal — Playlist
gallery-label-modern-playlist = Modern — Playlist
gallery-label-fullscreen = Fullscreen player
gallery-label-fullscreen-lyrics = Fullscreen lyrics
gallery-label-search = Search
gallery-label-theme-editor = Theme editor
gallery-label-player-settings = Player settings
gallery-label-downloader = Downloader

## Footer
footer-license = MIT License — Free & Open Source
footer-github = GitHub
footer-releases = Releases
footer-issues = Issues
footer-discord = Discord

## Page meta
home-title = Kopuz — Music Player

home-meta-desc = Kopuz is a modern, lightweight music player built with Rust and Dioxus. Stream from Jellyfin or Navidrome, connect YouTube Music, browse local files, enjoy synced lyrics, themes, and a built-in equalizer.
home-meta-keywords = Kopuz, music player, Rust, Dioxus, Jellyfin, Navidrome, YouTube Music, SoundCloud, open source, Linux, macOS, Windows, MPRIS, lyrics
og-title = Kopuz — Music Player
og-desc = Modern, lightweight music player built with Rust. Local files, Jellyfin, Navidrome, YouTube Music, SoundCloud streaming, synced lyrics, equalizer, Discord RPC, and more. Free and open source.
og-image-alt = Kopuz music player
twitter-title = Kopuz — Music Player
twitter-desc = Modern, lightweight music player built with Rust. Free and open source. Stream from Jellyfin, Navidrome, or YouTube Music.
