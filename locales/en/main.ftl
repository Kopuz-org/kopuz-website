## Nav
nav-features = Features
nav-download = Download
nav-lang-label = Language

## Hero
hero-title-1 = One player for
hero-title-2 = all your music.
hero-desc = Kopuz puts your local files, your NAS, your own server, and your streaming accounts in one library. It’s free and open source.
hero-cta-download = Download
hero-cta-github = View the source on GitHub
hero-screenshot-alt = Kopuz home screen

## Features
features-title = Features
features-source-local = Local files
features-source-jellyfin = Jellyfin
features-source-navidrome = Navidrome
features-source-subsonic = Subsonic API
features-source-nextcloud = Nextcloud
features-source-ytmusic = YouTube Music
features-source-applemusic = Apple Music
features-source-soundcloud = SoundCloud
features-source-spotify = Spotify

feat-local-title = Folders and servers
feat-local-desc = Add local folders or connect a Jellyfin or Subsonic server, such as Navidrome. Tracks from every source you add show up in one library, and Subsonic keeps your queue in sync across clients.
feat-theming-title = Themes
feat-theming-desc = Pick one of 24 built-in light and dark themes, or build your own and set every color yourself.
feat-native-title = System media controls
feat-native-desc = Control playback from your system’s media keys and controls: MPRIS on Linux, Now Playing on macOS, and the media overlay on Windows.
feat-android-title = Android
feat-android-desc = Runs on 64-bit ARM phones with Android 7.0 or later. Swipe between tracks, use shuffle and repeat from the notification, and switch to AMOLED black.
feat-lyrics-title = Synced lyrics
feat-lyrics-desc = Read synced, word-timed, or plain lyrics. The current line stays centered while the rest blur, instrumental breaks show as gaps, and you can shift the timing by hand or let Kopuz do it.
feat-eq-title = Equalizer
feat-eq-desc = Use a 10-band equalizer with built-in presets or your own settings.
feat-fav-title = Favorites and playlists
feat-fav-desc = Keep favorites on your device or sync them with your server. Create playlists, add whole albums at once, and sync playlists to your server.
feat-scrobble-title = Scrobbling
feat-scrobble-desc = Send what you play to Last.fm, Libre.fm, or ListenBrainz. On Jellyfin, you can also use the ListenBrainz plugin.
feat-radio-title = Internet radio
feat-radio-desc = Play internet radio stations and see what’s playing on each stream.
feat-offline-title = Offline downloads
feat-offline-desc = Download tracks from sources that allow it and play them without a connection.
feat-fonts-title = Custom font
feat-fonts-desc = Choose the font the interface uses.
feat-discord-title = Discord status
feat-discord-desc = Show the track you’re playing on your Discord profile.
feat-search-title = Search
feat-search-desc = Find artists, albums, and tracks from the main view or the quick search overlay.
feat-genre-title = Genres
feat-genre-desc = Browse tracks from your folders and servers by genre.
feat-logs-title = Play counts
feat-logs-desc = Kopuz counts your plays on your device, so you can see how often you’ve played each track.
feat-i18n-title = Languages
feat-i18n-desc = Use Kopuz in any of 28 languages.
feat-ytdlp-title = YouTube downloader
feat-ytdlp-desc = Paste a YouTube or YouTube Music link to a song, album, or playlist. Kopuz downloads it with its own YouTube client, tags it with title, artist, album, and cover art, and files it by album. Best audio keeps the original stream and needs nothing installed. To convert to MP3, FLAC, or WAV, install ffmpeg.
feat-crossfade-title = Crossfade and gapless playback
feat-crossfade-desc = Fade between tracks during desktop playback, or turn crossfade off for gapless albums.
feat-channels-title = Channel modes
feat-channels-desc = Play in stereo or mono, use only the left or right channel, or swap them.
feat-youtube-title = YouTube Music
feat-youtube-desc = Browse, search, and start mix radio from any track. Discover suggests songs, albums, artists, and moods, and artist pages show top songs, releases, and related artists. Sign in for your library, Liked Music, and playlists, or browse without an account.
feat-applemusic-title = Apple Music
feat-applemusic-desc = Browse the catalog and your library, sync favorites, edit playlists, start radio, download tracks, and read word-timed lyrics. Desktop playback needs Widevine.
feat-nextcloud-title = Nextcloud
feat-nextcloud-desc = Connect over WebDAV, choose library folders for each server, and stream or download tracks. Kopuz caches cover art and reads track lengths from file headers.
feat-metadata-title = Artist images
feat-metadata-desc = Show an album cover or an artist photo from Jellyfin or Subsonic. When there’s no photo, Kopuz uses album art.
feat-debug-title = Logs and crash reports
feat-debug-desc = Find your logs and crash reports, and share them when you report a bug.
feat-cleanup-title = Library cleanup
feat-cleanup-desc = When you rescan, Kopuz removes tracks whose files are gone.
feat-soundcloud-title = SoundCloud
feat-soundcloud-desc = Sign in once in your browser. Then search SoundCloud, play MP3 and Go+ AAC/HLS streams, like tracks, use your liked tracks as favorites, and open your playlists (read-only).
feat-spotify-title = Spotify
feat-spotify-desc = Browse your saved tracks, albums, playlists, and Discover, search, and scrobble. Play through Spotify’s Web Playback SDK or on any of your Spotify Connect devices. You need Spotify Premium to play music.
feat-miniplayer-title = Mini player
feat-miniplayer-desc = Open a small now-playing window from the bottom bar.
feat-tray-title = Minimize to tray
feat-tray-desc = Keep Kopuz playing in the system tray after you close the window. On Linux, you need an appindicator library.
feat-badges-title = File type badges
feat-badges-desc = Local tracks show their file format, like MP3, FLAC, or WAV.

## What's new
new-title = What’s new in Kopuz

## Performance
perf-skip-label = Incremental scans
perf-skip-desc = Rescans only index new and changed files instead of rebuilding your library.
perf-parallel-label = Parallel startup
perf-parallel-desc = Kopuz loads your library, playlists, and settings at the same time when it starts.
perf-art-label = Album art cache
perf-art-desc = Kopuz caches album covers so it loads each one once.
perf-lazy-label = Lazy images
perf-lazy-desc = Images load only when they scroll into view.
perf-io-label = Background scans
perf-io-desc = Library scans run in the background while you keep listening.
perf-http-label = Artwork downloads
perf-http-desc = Kopuz downloads artwork from your servers once and reuses it.
perf-sort-label = Fast sorting
perf-sort-desc = Sorting stays fast on lists with thousands of tracks.

## Install
install-cargo-title = Cargo (crates.io)
install-cargo-desc = Install the latest version with Cargo:
install-nix-title = Nix / NixOS
install-nix-run = Run it without installing:
install-nix-profile = Or add it to your profile:
install-nix-note = The NixOS flake uses a Cachix binary cache, so you don’t build Kopuz yourself.
install-aur-title = AUR (Arch Linux)
install-aur-desc = Install it with your AUR helper:
install-flatpak-title = Flatpak
install-flatpak-desc = Install Kopuz from Flathub:
install-flatpak-note = Flatpak updates Kopuz along with your other apps.
install-appimage-title = AppImage
install-appimage-desc-1 = Download it from GitHub Releases and run it. On Linux, make sure
install-appimage-desc-2 = { " " }and
install-appimage-desc-3 = { " " }are installed.
install-appimage-note-1 = On Arch, if it crashes with a WebKitNetworkProcess error, start it with
install-appimage-note-2 = .
install-macos-title = macOS
install-macos-desc-1 = Download the
install-macos-desc-2 = { " " }from GitHub Releases. If macOS blocks it, remove the quarantine flag:
install-macos-homebrew = Install the Apple silicon build from the Homebrew tap:
install-android-title = Android
install-android-desc = Get Kopuz from Google Play, or download the signed arm64-v8a APK from GitHub Releases. You need Android 7.0 (API 24) or later.
install-android-note = If your installed APK came from another source, uninstall it before you install this one.

## Nextcloud
nextcloud-subtitle = Add it in Settings → Media servers → Add → Nextcloud.
nextcloud-connect-title = Use an app password
nextcloud-connect-desc = Enter your Nextcloud URL, your username, and an app password. Create the app password in Nextcloud under Settings → Security; you can revoke it at any time.
nextcloud-folders-title = Choose library folders
nextcloud-folders-desc = Pick one or more library folders for each server. Kopuz reads the WebDAV folder tree, caches cover art, and reads track lengths from file headers.
nextcloud-webdav-title = Plain WebDAV
nextcloud-webdav-desc = Your library follows your folder layout, without playlists or radio. If Nextcloud Music offers a Subsonic endpoint, connect that instead for better metadata and playlists.
nextcloud-playback-title = Stream or download
nextcloud-playback-desc = Play files straight from your server or download them to listen offline. Your audio never passes through Kopuz servers.

## YouTube Music
ytmusic-subtitle = Add it in Settings → Media servers → Add → YouTube Music.
ytmusic-token-title = No helper needed
ytmusic-token-desc = You can play without signing in, and you don’t need to install a helper.
ytmusic-signin-title = Sign in with a browser
ytmusic-signin-desc = Kopuz opens Google sign-in in a separate browser profile and reads the session cookies from it. Once you sign in, you get your library, Liked Music, playlists, and followed artists.
ytmusic-signin-note = Kopuz uses your default browser. You can pick Firefox, Chrome, Chromium, Brave, Edge, Vivaldi, Helium, LibreWolf, Zen, or Floorp instead.
ytmusic-anon-title = Anonymous mode
ytmusic-anon-desc = Browse and search without signing in or storing cookies. You can open artist, album, and playlist pages, start mix radio, and play public tracks, but you can’t like tracks or see a library.
ytmusic-premium-title = Premium quality
ytmusic-premium-desc = With a Music Premium account, Kopuz plays and downloads the higher-quality streams through its own YouTube client. Anonymous mode can’t play Premium-only tracks.

## Apple Music
applemusic-subtitle = Add it in Settings → Media servers → Add → Apple Music.
applemusic-signin-title = Sign in or paste a token
applemusic-signin-desc = On desktop, Kopuz opens Apple Music in a separate Chromium-based browser profile. You can also paste a media-user-token yourself.
applemusic-playback-title = Widevine on desktop
applemusic-playback-desc = Apple Music playback needs a Widevine CDM, either from a browser you’ve installed or from Mozilla’s plugin service. Kopuz doesn’t include one.
applemusic-features-title = Supported features
applemusic-features-desc = Browse the catalog and your library, sync favorites, add and remove playlist tracks, start track and playlist radio, download, and read word-timed lyrics.
applemusic-android-title = Android status
applemusic-android-desc = You can sign in on Android through an in-app web view, but playback doesn’t work there yet.

## SoundCloud
soundcloud-subtitle = Add it in Settings → Media servers → Add → SoundCloud.
soundcloud-signin-title = One-time browser sign-in
soundcloud-signin-desc = You don’t need a URL, password, or token. Kopuz opens SoundCloud sign-in in a separate browser profile, and deletes that profile when you remove the source.
soundcloud-features-title = Supported features
soundcloud-features-desc = Search, play MP3 and Go+ AAC/HLS streams, use liked tracks as favorites, like and unlike tracks, and open playlists (read-only).

## Spotify setup
spotify-guide-subtitle = Kopuz uses Spotify’s official Web API and browser playback. It never sees your password or relays the audio.
spotify-step-1-title = Create a Spotify app
spotify-step-1-desc = In Spotify’s developer dashboard, create an app and turn on Web API and Web Playback SDK. Under User Management, add each account that will listen (up to five in Development Mode). Then set this redirect URI exactly:
spotify-step-2-title = Add your Client ID
spotify-step-2-desc = In Settings → Media servers → Add → Spotify, paste the Client ID and approve access. You don’t need a Client Secret. Port 8898 has to be free while you sign in.
spotify-step-3-title = Choose where to play
spotify-step-3-desc = Keep the in-app player tab open, or pick one of your Spotify Connect devices from the device menu.
spotify-requirement = You need Spotify Premium to play music. Use Chrome, Edge, Brave, Chromium, Vivaldi, Helium, or Safari on macOS. Firefox doesn’t work.
spotify-full-guide = Read the full Spotify setup and troubleshooting guide

## Feature guides
guides-title = Guides

## Privacy and storage
privacy-local-title = One local database
privacy-local-desc = Kopuz keeps your library, playlists, favorites, play counts, and source data in kopuz.db, and your preferences next to it in settings.toml.
privacy-accounts-title = Separate sign-ins
privacy-accounts-desc = Each source keeps its own sign-in and favorites. Browser sign-ins use separate profiles, not your everyday browser session.
privacy-files-title = Cache and logs
privacy-files-desc = Album art and offline tracks go in your system cache folder. You can open or export logs from Settings.

## Requirements
requirements-title = Requirements and limitations

## About the name
about-title = What is a kopuz?
about-desc-1 = The kopuz is an old Turkic string instrument that bards and shamans played. It’s thought to be an ancestor of several Central Asian lutes.
about-desc-2 = In Turkic legend, it belongs to Dede Korkut, a bard. The Kyrgyz komuz and the Kazakh kobyz are related instruments. The xomus, despite the similar name, isn’t.

## Community
community-title = Contribute to Kopuz
community-issues-title = Issues
community-issues-desc = Report a bug or pick up an open task.
community-discussions-title = Discussions
community-discussions-desc = Ask questions and suggest changes.
community-discord-desc = Chat with users and contributors.
community-contribute-title = Contribute
community-contribute-desc = Get the source code, set up a dev environment, or help translate.

## Platforms
platforms-title = Download Kopuz
platforms-windows = Windows
platforms-macos = macOS
platforms-macos-note = Apple silicon only
platforms-linux = Linux
platforms-android = Android
platforms-android-note = ARM64 · Android 7.0+ · API 24+

## Support
support-title = Support Kopuz
support-subtitle = Kopuz is free and open source. Your donations pay for its development.
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
home-title = Kopuz: free open-source music player

home-meta-desc = Kopuz is a free music player for desktop and Android. Play local files, SMB shares, Jellyfin, Subsonic, YouTube Music, Spotify, and more in one library.
home-meta-keywords = Kopuz, music player, Jellyfin, Navidrome, Subsonic, SMB, NAS, Clippsly, Nextcloud, YouTube Music, Apple Music, SoundCloud, Spotify, open source, Android, Linux, macOS, Windows, lyrics, equalizer
og-title = Kopuz: free open-source music player
og-desc = A free music player for local files, servers, and streaming services on desktop and Android, with synced lyrics, themes, and a 10-band equalizer.
og-image-alt = Kopuz music player
twitter-title = Kopuz: free open-source music player
twitter-desc = Play local files, SMB shares, Jellyfin, Subsonic, YouTube Music, SoundCloud, Spotify, and more in one library.

## Deep link (/j)
join-title = Opening Kopuz…
join-opening = Opening Kopuz…
join-fallback = Kopuz didn’t open. Don’t have it yet?
join-no-payload = This link doesn’t include a queue.
join-download = Download Kopuz

## Privacy policy
footer-privacy = Privacy
privacy-english-note = This policy is written in English. Translations of the site don’t change it.
privacy-back = Go to the home page

## Home page (2026 redesign)
nav-home = Home
footer-license-line = EUPL 1.2, free and open source
shelf-link-to-us = Link to us with an 88×31 button
hero-release = Latest release
hero-cta-download-for = Download for
hero-cta-all = All platforms
home-sources-title = Play music from
source-local = Local folders
source-navidrome = Navidrome and Subsonic
source-radio = Internet radio
moment-lyrics-title = Lyrics that keep time
moment-lyrics-alt = Kopuz playing a track with full-screen synced lyrics
moment-library-title = One library for every source
moment-library-alt = The Kopuz library with albums from several sources
moment-themes-title = Your colors, or your wallpaper’s
moment-themes-alt = The Kopuz theme editor
moment-themes-extra = Or let Kopuz build a theme from your desktop wallpaper.
theme-default = Gruvbox dark soft
theme-amoled = AMOLED
theme-wallpaper = Wallpaper
wallpaper-sunset = Sunset
wallpaper-lake = Lake
home-see-all-features = See all features
home-get-title = Get Kopuz
home-get-download = Download
home-also-on = You can also install Kopuz with Cargo, Nix, the AUR, Flatpak, or Homebrew.
home-sponsors-title = Sponsors
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
mode-simple = Simple site
mode-full = Full site
webbutton-alt = Kopuz 88x31 web button

## Page titles
features-page-title = Kopuz features: sources, lyrics, themes, and playback
download-page-title = Download Kopuz for Windows, macOS, Linux, and Android
guides-page-title = Kopuz setup guides for servers and streaming services
support-page-title = Support Kopuz: sponsors, donations, and community
notfound-page-title = Page not found | Kopuz
guide-title-suffix = setup guide | Kopuz

## Not found
notfound-title = Page not found.
notfound-desc = Nothing’s playing at this address.
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
install-other-title = Other ways to install
req-spotify = You need Premium, your own Client ID, and a supported browser to play music.
req-applemusic = Desktop playback needs a Widevine CDM. You can sign in on Android, but playback doesn’t work there yet.
req-nextcloud = Plain WebDAV doesn’t support playlists or radio. If Nextcloud Music offers a Subsonic endpoint, use that instead.
req-android = Release APKs need an arm64-v8a phone with Android 7.0 (API 24) or later. Discord status and the system tray aren’t available.
req-appimage = Needs webkit2gtk-4.1 and GTK 3. The tray also needs an appindicator library.
req-ytmusic = Anonymous mode can’t play Premium-only tracks. Sign in with a Music Premium account to play them.
req-crossfade = Crossfade and gapless playback work with native desktop playback, not with Spotify audio played in the browser.
req-spotify-limits-label = Spotify limits
req-spotify-limits = In Development Mode, an app allows up to five listeners, search returns up to ten results per type, playlists are read-only, and downloads, radio, tag editing, the equalizer, and crossfade are off.

## Support page
support-note = I’m Temidaradev, and I build Kopuz. I’m a student without a steady income, so sponsors are what let me keep working on Kopuz while I study.
support-goal-label = GitHub Sponsors goal
support-goal-amount = ${ $current } of ${ $goal } per month
support-goal-aria = GitHub Sponsors goal progress
support-crypto-summary = Crypto addresses
sponsors-tier-special = Special
sponsors-tier-monthly = Monthly
sponsors-tier-one-time = One-time


## Added in the 2026 copy pass
features-source-smb = SMB and NAS
features-source-clippsly = Clippsly
feat-smb-title = SMB and NAS
feat-smb-desc = Add an SMB 2 or 3 share, or a folder inside one, as a library. Kopuz reads embedded tags and cover art and plays straight from the share with seeking, so you don’t have to mount it.
feat-clippsly-title = Clippsly
feat-clippsly-desc = Sign in with your Clippsly username and an app password from clippsly.app. Stream and download from your locker, upload audio files, and check your storage use. Remote playlists aren’t supported yet.
feat-updates-title = Automatic updates
feat-updates-desc = On desktop, a banner tells you when a new version is out, and Kopuz downloads and installs it for you.
feat-cli-title = Daemon and command line
feat-cli-desc = The player runs as a daemon, kopuzd, that the window connects to. Use kopuzctl to control playback, set up sources, and start library scans from a terminal or a script, or talk to the gRPC API directly.
install-android-play = Get it on Google Play
footer-made-with = Website made with
footer-love = love
footer-made-by = by
source-smb = SMB and NAS
source-clippsly = Clippsly
player-prev-label = Previous section
player-next-label = Next section
player-seek-label = Position on the page
player-queue-label = Sections on this page
player-queue-title = On this page
dl-file-exe = Setup (.exe)
dl-file-msi = Installer (.msi)
dl-file-dmg = Disk image (.dmg)
dl-file-appimage = AppImage
dl-file-deb = Debian and Ubuntu (.deb)
dl-file-rpm = Fedora and RHEL (.rpm)
dl-file-apk = APK
dl-release-page = Release page
dl-this-device = This device
dl-dialog-ready = Latest release: { $version }. Choose the installer for your device.
dl-dialog-close = Close
dl-dialog-release = See the release on GitHub
guide-nextcloud-summary = Connect over WebDAV with an app password, then pick your library folders.
guide-ytmusic-summary = Sign in through your browser, or listen without an account.
guide-applemusic-summary = Sign in, get Widevine working on desktop, and see what works on Android.
guide-soundcloud-summary = Sign in once in your browser and see what the source supports.
guide-spotify-summary = Create a developer app, add its Client ID, and pick where to play.

## Player bar section names
chapter-home = Home
chapter-sources = Sources
chapter-lyrics = Lyrics
chapter-library = Library
chapter-themes = Themes
chapter-download = Download
chapter-sponsors = Sponsors
chapter-whats-new = What’s new
chapter-name = The name
chapter-not-found = Not found
chapter-privacy = Privacy
chapter-install = Install
chapter-requirements = Requirements
chapter-support = Support
chapter-community = Community
chapter-features = Features
chapter-guides = Guides

## SMB and NAS guide
guide-smb-summary = Point Kopuz at a share on your NAS and sign in with your NAS account.
smb-subtitle = Choose SMB / NAS in the source selector. You don’t need to mount the share first.
smb-share-title = Point it at a share
smb-share-desc = Enter the URL of a share, or of a folder inside one, for example:
smb-share-port = If your NAS uses a port other than the default, add it to the URL:
smb-signin-title = Sign in with your NAS account
smb-signin-desc = Use your NAS username and password. For a domain account, enter DOMAIN\username. Kopuz needs SMB 2 or 3 and an account: it doesn’t search the network for shares or sign in as a guest.
smb-library-title = How the library works
smb-library-desc = Kopuz scans the share and its subfolders for audio files and reads their embedded tags and cover art. It plays straight from the share, with seeking, on Android too. Your favorites are kept in Kopuz.
smb-limits-title = The share stays read-only
smb-limits-desc = Kopuz doesn’t change anything on the share, so you can’t edit tags there, keep playlists on it, or download its tracks for offline listening.

## Clippsly guide
guide-clippsly-summary = Sign in with an app password, stream your locker, and upload music.
clippsly-subtitle = Add Clippsly from the source settings.
clippsly-signin-title = Sign in with an app password
clippsly-signin-desc = Create an app password at clippsly.app. Then sign in to Kopuz with your Clippsly username and that app password.
clippsly-locker-title = Your locker
clippsly-locker-desc = Browse and stream the music in your Clippsly locker, and download tracks to play offline.
clippsly-upload-title = Upload music
clippsly-upload-desc = Upload audio files from the source’s settings card. Kopuz refreshes your library when the whole batch finishes. The card also shows how much storage you’re using; Clippsly sets your quota.
clippsly-bonus = Link Kopuz to your Clippsly account and Clippsly adds 3 GB of storage for free.
clippsly-limits-title = Playlists
clippsly-limits-desc = Playlists you keep on Clippsly don’t show up in Kopuz yet.
clippsly-site = Go to clippsly.app
