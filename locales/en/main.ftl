## Nav
nav-features = Features
nav-download = Download
nav-lang-label = Language

## Hero
hero-title-1 = Local files.
hero-title-2 = Connected services.
hero-desc = Kopuz plays music from local folders, Jellyfin, Subsonic, Nextcloud, YouTube Music, Apple Music, SoundCloud, and Spotify.
hero-cta-download = Download
hero-cta-github = Open Source on GitHub
hero-screenshot-alt = Kopuz home screen

## Features
features-title = Features
features-chip = Local playback, connected services, lyrics, playlists, and themes.
features-source-local = Local files
features-source-jellyfin = Jellyfin
features-source-navidrome = Navidrome
features-source-subsonic = Subsonic API
features-source-nextcloud = Nextcloud
features-source-ytmusic = YouTube Music
features-source-applemusic = Apple Music
features-source-soundcloud = SoundCloud
features-source-spotify = Spotify

feat-local-title = Music Sources
feat-local-desc = Add local folders or connect Jellyfin, Subsonic, Nextcloud, YouTube Music, Apple Music, SoundCloud, and Spotify. Tracks from these sources appear in one library.
feat-theming-title = Theming
feat-theming-desc = Use a built-in theme or set the interface colors yourself.
feat-native-title = Desktop Controls
feat-native-desc = Use system media controls on Linux, macOS, and Windows.
feat-android-title = Android
feat-android-desc = ARM64 build for Android 7.0 and newer, with swipe gestures, shuffle and repeat controls in the media notification, and AMOLED mode.
feat-lyrics-title = Synced Lyrics
feat-lyrics-desc = Displays synced, word-timed, or plain lyrics with instrumental breaks, a centered active line, depth blur, and manual or automatic timing offset.
feat-eq-title = Equalizer
feat-eq-desc = 10-band equalizer with built-in presets and custom settings.
feat-fav-title = Favorites & Playlists
feat-fav-desc = Keep favorites locally or sync them with your server. Create playlists and add complete albums.
feat-scrobble-title = Scrobbling
feat-scrobble-desc = Scrobble your listening history to Last.fm, Libre.fm, or ListenBrainz. Jellyfin users can also use the ListenBrainz plugin.
feat-radio-title = Internet Radio
feat-radio-desc = Play internet radio stations with stream and now-playing metadata.
feat-offline-title = Offline Downloads
feat-offline-desc = Download supported remote tracks for offline playback.
feat-fonts-title = Custom UI Fonts
feat-fonts-desc = Set a custom interface font.
feat-discord-title = Discord RPC
feat-discord-desc = Show the current track on Discord.
feat-search-title = Search
feat-search-desc = Search artists, albums, and tracks from the main interface or the quick-search overlay.
feat-genre-title = Genre Browsing
feat-genre-desc = Browse local and server tracks by genre.
feat-logs-title = Listening Logs
feat-logs-desc = Play counts are stored locally and can be reviewed by track.
feat-i18n-title = Languages
feat-i18n-desc = Kopuz provides 18 interface languages.
feat-ytdlp-title = yt-dlp Downloader
feat-ytdlp-desc = Download from YouTube and other supported sites in the best available audio format or as MP3, FLAC, Opus, WAV, or MP4 video. yt-dlp is required. SponsorBlock, chapters, cookies, and rate limiting are also supported.
feat-crossfade-title = Crossfade & Transitions
feat-crossfade-desc = Crossfade tracks during native desktop playback.
feat-channels-title = Channel Modes
feat-channels-desc = Choose stereo, mono, left-only, right-only, or swapped channels.
feat-youtube-title = YouTube Music
feat-youtube-desc = Browse YouTube Music, search its catalog, view artist pages, and start mix radio. Sign in to access your library, Liked Music, and playlists. Anonymous browsing is also available.
feat-applemusic-title = Apple Music
feat-applemusic-desc = Browse the catalog and your library, sync favorites, add or remove playlist tracks, start radio, download tracks, and use word-timed lyrics. Desktop playback requires Widevine.
feat-nextcloud-title = Nextcloud
feat-nextcloud-desc = Connect over WebDAV, choose per-server library folders, stream or download tracks, cache cover art, and probe durations from file headers.
feat-metadata-title = Artist Images
feat-metadata-desc = Use an album cover or fetch an artist photo from Jellyfin or Subsonic. Kopuz falls back to album art when no photo is available.
feat-debug-title = Logs and Crash Reports
feat-debug-desc = Find and share logs and crash reports.
feat-cleanup-title = Library Cleanup
feat-cleanup-desc = Rescans remove entries for missing or deleted files.
feat-soundcloud-title = SoundCloud
feat-soundcloud-desc = Sign in once through a browser to search SoundCloud, play progressive MP3 or Go+ AAC/HLS streams, access your Liked tracks as favorites, and view read-only playlists.
feat-spotify-title = Spotify
feat-spotify-desc = Browse saved tracks, albums, playlists, and Discover; search and scrobble; then play through the Web Playback SDK or your Spotify Connect devices. Premium is required for playback.
feat-miniplayer-title = Mini-Player
feat-miniplayer-desc = Open a compact now-playing window from the bottom bar.
feat-tray-title = Minimize to Tray
feat-tray-desc = Keep Kopuz running in the system tray after closing the window. Linux requires an appindicator library.
feat-badges-title = File-Type Badges
feat-badges-desc = Local track rows show the file format, such as MP3, FLAC, or WAV.

## What's new
new-title = What’s new in Kopuz

## Performance
perf-skip-label = Skip already indexed
perf-skip-desc = Rescans index new and changed files instead of rebuilding the library.
perf-parallel-label = Concurrent loading
perf-parallel-desc = Library data, playlists, and settings load concurrently at startup.
perf-art-label = Album art caching
perf-art-desc = Album covers are cached for reuse.
perf-lazy-label = Lazy image loading
perf-lazy-desc = Images load when they enter the viewport.
perf-io-label = Background scanning
perf-io-desc = Library scans run in the background.
perf-http-label = HTTP art caching
perf-http-desc = Cached artwork avoids repeated downloads.
perf-sort-label = Large-list sorting
perf-sort-desc = Sorting is optimized for large lists.

## Install
install-cargo-title = Cargo (crates.io)
install-cargo-desc = Install the latest published version directly with Cargo:
install-nix-title = Nix / NixOS
install-nix-run = Run without installing:
install-nix-profile = Or add to your profile:
install-nix-note = NixOS flake support includes a Cachix binary cache.
install-aur-title = AUR (Arch Linux)
install-aur-desc = Install with your preferred helper:
install-aur-note-1 = Requires
install-aur-note-2 = { " " }installed first at a version matching dioxus 0.7.x.
install-flatpak-title = Flatpak
install-flatpak-desc = Install Kopuz from Flathub:
install-flatpak-note = Updates arrive through your normal Flatpak update flow.
install-appimage-title = AppImage
install-appimage-desc-1 = Download from GitHub Releases and run directly. On Linux, make sure
install-appimage-desc-2 = { " " }and
install-appimage-desc-3 = { " " }are installed on your system.
install-appimage-note-1 = Arch users: if it crashes with a WebKitNetworkProcess error, prefix the command with
install-appimage-note-2 = .
install-macos-title = macOS
install-macos-desc-1 = Download the
install-macos-desc-2 = { " " }from GitHub Releases. If macOS blocks it, clear the quarantine flag:
install-macos-homebrew = Install the Apple Silicon build from the project’s Homebrew tap:
install-android-title = Android
install-android-desc = Download the signed arm64-v8a APK from GitHub Releases. Requires Android 7.0 / API 24 or newer.
install-android-note = If another source signed your installed APK, uninstall it before installing the project release.

## Nextcloud
nextcloud-subtitle = Add it in Settings → Media servers → Add → Nextcloud.
nextcloud-connect-title = Use an app password
nextcloud-connect-desc = Enter your Nextcloud base URL, username, and a revocable app password created under Nextcloud Settings → Security.
nextcloud-folders-title = Choose library folders
nextcloud-folders-desc = Pick one or more folders for each source. Kopuz reads the WebDAV folder tree, caches available cover art, and probes file headers for track durations.
nextcloud-webdav-title = Raw WebDAV behavior
nextcloud-webdav-desc = The library follows the folder layout and does not provide playlists or radio. If Nextcloud Music exposes Subsonic, use that source for richer metadata and playlists.
nextcloud-playback-title = Stream or download
nextcloud-playback-desc = Play files directly from your server or download them for offline listening without routing audio through Kopuz infrastructure.

## YouTube Music
ytmusic-subtitle = Add it in Settings → Media servers → Add → YouTube Music.
ytmusic-token-title = No helper needed
ytmusic-token-desc = Anonymous playback does not require a helper. Kopuz handles the token flow, including inside Flatpak.
ytmusic-signin-title = Sign in with a browser
ytmusic-signin-desc = Kopuz opens Google sign-in in a separate browser profile and reads the resulting session cookies. Signing in enables your library, Liked Music, playlists, and followed artists.
ytmusic-signin-note = Choose an installed Chromium-family browser such as Chrome, Chromium, Brave, Edge, Vivaldi, or Helium.
ytmusic-anon-title = Anonymous mode
ytmusic-anon-desc = No sign-in, no cookies. Browse, search, open artist, album, and playlist pages, start mix radio, and play public tracks. Liking and library views are disabled.
ytmusic-premium-title = Premium tracks
ytmusic-premium-desc = Some Premium-only tracks may need local yt-dlp fallback. Anonymous mode cannot play Premium-only content.

## Apple Music
applemusic-subtitle = Add it in Settings → Media servers → Add → Apple Music.
applemusic-signin-title = Sign in or paste a token
applemusic-signin-desc = On desktop, Kopuz opens Apple Music in an isolated Chromium-family browser profile. You can instead paste a media-user-token manually.
applemusic-playback-title = Widevine on desktop
applemusic-playback-desc = Apple Music playback needs a Widevine CDM from an installed browser or Mozilla’s plugin service. Kopuz does not ship the CDM.
applemusic-features-title = Supported features
applemusic-features-desc = Catalog and library browsing, favorites sync, playlist add/remove, track and playlist radio, downloads, and word-timed lyrics.
applemusic-android-title = Android status
applemusic-android-desc = Sign-in uses an in-app WebView, but Apple Music playback is not yet supported on Android.

## SoundCloud
soundcloud-subtitle = Add it in Settings → Media servers → Add → SoundCloud.
soundcloud-signin-title = One-time browser sign-in
soundcloud-signin-desc = No URL, password, or manual token needed. Kopuz opens SoundCloud sign-in in an isolated profile using Chrome, Chromium, Brave, Edge, Vivaldi, or Helium. Removing the source removes that profile.
soundcloud-features-title = Supported features
soundcloud-features-desc = Search, progressive MP3 and Go+ AAC/HLS playback, Liked tracks as favorites, read-only playlists, and like/unlike controls.

## Spotify setup
spotify-guide-subtitle = Spotify uses its official Web API and browser playback. Kopuz does not receive your password or proxy the audio stream.
spotify-step-1-title = Create a Spotify app
spotify-step-1-desc = In Spotify’s developer dashboard, create an app, enable Web API and Web Playback SDK, add every listening account under User Management, and set this exact redirect URI:
spotify-step-2-title = Add your Client ID
spotify-step-2-desc = In Settings → Media servers → Add → Spotify, paste the Client ID and approve access. No Client Secret is needed; port 8898 must be free during sign-in.
spotify-step-3-title = Choose where to play
spotify-step-3-desc = Keep the browser-backed in-app player tab open, or select one of your Spotify Connect devices from Kopuz’s device menu.
spotify-requirement = Spotify Premium is required for playback. Use Chrome, Edge, Brave, Chromium, Vivaldi, Helium, or Safari on macOS; Firefox is not supported.
spotify-full-guide = Read the full Spotify setup and troubleshooting guide →

## Feature guides
guides-title = Guides
guides-subtitle = Setup guides for music services and major features.

## Privacy and storage
privacy-local-title = One local database
privacy-local-desc = Library data, playlists, favorites, play counts, and source data live in kopuz.db; preferences live beside it in settings.toml.
privacy-accounts-title = Separate source credentials
privacy-accounts-desc = Each connected media source keeps its own credentials and favorites. Browser sign-ins use isolated profiles instead of your normal browsing session.
privacy-files-title = Cache and logs
privacy-files-desc = Album art and offline tracks use your system cache directory. Open or export logs from Settings.

## Requirements
requirements-title = Requirements and limitations

## About the name
about-title = What is a kopuz?
about-desc-1 = The kopuz is an ancient Turkic string instrument, traditionally associated with bards and shamans and considered an ancestor of several Central Asian lutes.
about-desc-2 = Turkic legend links it to Dede Korkut, a legendary bard. The Kyrgyz komuz and Kazakh kobyz are related instruments, while the similarly named xomus is not.

## Community
community-title = Contribute to Kopuz
community-issues-title = Issues
community-issues-desc = Report a bug or pick up an open task.
community-discussions-title = Discussions
community-discussions-desc = Ask questions and propose changes.
community-discord-desc = Chat with users and contributors in real time.
community-contribute-title = Contribute
community-contribute-desc = Source code, development setup, and translations.

## Platforms
platforms-title = Download Kopuz
platforms-subtitle = Download current builds for Windows, macOS, Linux, and Android.
platforms-windows = Windows
platforms-macos = macOS
platforms-macos-note = Apple Silicon only
platforms-linux = Linux
platforms-android = Android
platforms-android-note = ARM64 · Android 7.0+ · API 24+

## Support
support-title = Support Kopuz
support-subtitle = Kopuz is free and open source. Donations support development.
support-gh = GitHub Sponsors
support-bmc = Buy Me a Coffee
support-usdt-note = (Solana chain)

## Sponsors
sponsors-cta = Become a Sponsor

## Footer
footer-license = EUPL v1.2 | Free and Open Source
footer-github = GitHub
footer-releases = Releases
footer-issues = Issues
footer-discord = Discord

## Page meta
home-title = Kopuz Music Player

home-meta-desc = Kopuz plays local files and connects to Jellyfin, Subsonic, Nextcloud, YouTube Music, Apple Music, SoundCloud, and Spotify on desktop and Android. Apple Music playback currently requires desktop.
home-meta-keywords = Kopuz, music player, Jellyfin, Navidrome, Subsonic, Nextcloud, YouTube Music, Apple Music, SoundCloud, Spotify, open source, Android, Linux, macOS, Windows, lyrics, equalizer
og-title = Kopuz Music Player
og-desc = Music player for local files and connected services on desktop and Android. Includes synced lyrics, themes, and a 10-band equalizer.
og-image-alt = Kopuz music player
twitter-title = Kopuz Music Player
twitter-desc = Kopuz supports local files, Jellyfin, Subsonic, Nextcloud, YouTube Music, Apple Music, SoundCloud, and Spotify.

## Deep link (/j)
join-title = Opening Kopuz…
join-opening = Opening Kopuz…
join-fallback = Kopuz didn't open. Don't have it yet?
join-no-payload = This link has no queue attached.
join-download = Download Kopuz

## Privacy policy
footer-privacy = Privacy
privacy-english-note = This policy is maintained in English. Other site translations do not alter it.
privacy-back = Back to the homepage

## Home page (2026 redesign)
nav-home = Home
footer-license-line = EUPL 1.2 · free and open source
shelf-link-to-us = Link to us · 88×31
hero-release = Latest release
hero-cta-download-for = Download for
hero-cta-all = All platforms
home-sources-title = Plays from
source-local = Local folders
source-navidrome = Navidrome and Subsonic
source-radio = Internet radio
moment-lyrics-title = The line you are on, and nothing louder
moment-lyrics-alt = Kopuz playing a track with full-screen synced lyrics
moment-library-title = One library, every server
moment-library-alt = The Kopuz library with albums from several sources
moment-themes-title = Your colors, or a wallpaper's
moment-themes-alt = The Kopuz theme editor
moment-themes-extra = 11 built-in themes, AMOLED, and a live theme from your wallpaper.
theme-default = Gruvbox dark soft
theme-amoled = AMOLED
theme-wallpaper = Wallpaper
wallpaper-sunset = Sunset
wallpaper-lake = Lake
home-see-all-features = See all features
home-get-title = Get Kopuz
home-get-download = Download
home-also-on = Also on Cargo, Nix, AUR, Flatpak and AppImage
home-sponsors-title = Kept alive by sponsors
home-sponsor-cta = Sponsor on GitHub

## Shared controls
copy = Copy
copied = Copied
copy-address = Copy { $coin } address

## Chrome
nav-primary-aria = Primary navigation
nav-menu-open = Open menu
nav-menu-close = Close menu
footer-nav-aria = Footer
theme-use-light = Use light theme
theme-use-dark = Use dark theme
theme-leave-moe = Leave moe mode
webbutton-alt = Kopuz 88x31 web button

## Page titles
features-page-title = Features | Kopuz
download-page-title = Download | Kopuz
guides-page-title = Guides | Kopuz
support-page-title = Support | Kopuz
notfound-page-title = Page not found | Kopuz
guide-title-suffix = guide | Kopuz

## Not found
notfound-title = Page not found.
notfound-desc = Nothing plays at this address.
notfound-home = Go to the home page

## Features page
features-group-sources = Sources
features-group-playback = Playback
features-group-library = Library
features-group-desktop = Desktop
features-group-android = Android
features-group-integrations = Integrations
features-group-performance = Performance
features-group-privacy = Privacy
features-jump-aria = Feature groups
features-privacy-link = Read the privacy policy

## Download page
download-latest-cta = Download the latest release
install-other-title = Other ways to install
req-spotify = Playback requires Premium, a personal Client ID, and a supported browser.
req-applemusic = Desktop playback requires a Widevine CDM. Sign-in works on Android, but playback is not yet supported there.
req-nextcloud = Raw WebDAV does not support playlists or radio. Use a Subsonic endpoint from Nextcloud Music when available.
req-android = Release APKs require arm64-v8a and Android 7.0 / API 24 or newer. Discord RPC and the system tray are unavailable.
req-appimage = Requires webkit2gtk-4.1 and GTK 3. The tray also requires an appindicator library.
req-ytmusic = Anonymous mode cannot play Premium-only tracks. Signed-in playback can fall back to yt-dlp.
req-crossfade = Crossfade works with native desktop playback. It does not apply to Spotify audio played in the browser.
req-spotify-limits-label = Spotify limits
req-spotify-limits = Development Mode limits search, makes playlists read-only, and disables downloads, radio, tag editing, and Kopuz audio effects.

## Support page
support-note = I’m Temidaradev, Kopuz’s developer. I am a student without a stable income, and sponsors are what let me keep working on Kopuz while I study.
support-goal-label = GitHub Sponsors goal
support-goal-amount = ${ $current } of ${ $goal } per month
support-goal-aria = GitHub Sponsors goal progress
support-crypto-summary = Crypto addresses
sponsors-tier-special = Special
sponsors-tier-monthly = Monthly
sponsors-tier-one-time = One-time
