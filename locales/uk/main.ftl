## Language
lang-name = Українська

## Nav
nav-new = Нове
nav-announce = Сучасний UI, тема Vaxry, завантажувач yt-dlp, 20+ мов
nav-features = Можливості
nav-install = Встановити
nav-download = Завантажити
nav-sponsors = Спонсори
nav-github = GitHub ↗
nav-gallery = Галерея
nav-lang-label = Мова

## Hero
hero-title-1 = Твоя музика.
hero-title-2 = Твій стиль.
hero-desc = Kopuz — сучасний легкий музичний плеєр на Rust і Dioxus. Скануй локальні теки, стрім з Jellyfin або Navidrome, переглядай бібліотеку як зручно.
hero-cta-download = Завантажити
hero-cta-github = Переглянути на GitHub
hero-screenshot-alt = Kopuz — головний екран

## Features
features-title = Усе, що потрібно.
features-chip = Жодних підписок. Жодного стеження. Тільки твоя музика.
features-works-with = Працює з
features-source-local = Локальні файли
features-source-jellyfin = Jellyfin
features-source-navidrome = Navidrome
features-source-subsonic = Subsonic API

feat-local-title = Локально + стрімінг
feat-local-desc = Вкажи локальну теку або підключись до Jellyfin / Subsonic (Navidrome). Змішуй як хочеш.
feat-theming-title = Теми
feat-theming-desc = Динамічні теми з повним контролем змінних кольору. Створи свою тему з нуля або обери пресет.
feat-native-title = Нативна інтеграція
feat-native-desc = MPRIS на Linux, Now Playing на macOS, System Media Transport на Windows. Повністю підключено.
feat-lyrics-title = Синхронізовані тексти
feat-lyrics-desc = Прокручуваний текст у реальному часі, синхронізований або звичайний, з автопрокруткою.
feat-eq-title = Еквалайзер
feat-eq-desc = 5-смуговий еквалайзер з вбудованими пресетами та повним контролем звуку.
feat-fav-title = Обране та плейлисти
feat-fav-desc = Познач треки локально або синхронізуй обране з сервером. Створюй плейлисти, додавай цілі альбоми одразу.
feat-scrobble-title = Скроблінг
feat-scrobble-desc = Вбудований скроблінг ListenBrainz. Користувачі Jellyfin також можуть використовувати плагін listenbrainz.
feat-discord-title = Discord RPC
feat-discord-desc = Покажи друзям що ти слухаєш з вбудованим Discord Rich Presence. Без налаштування.
feat-search-title = Пошук
feat-search-desc = Пошук у реальному часі по виконавцях, альбомах і треках. Миттєві результати.
feat-genre-title = Перегляд за жанрами
feat-genre-desc = Переглядай всю бібліотеку за жанрами — працює для локальних файлів і серверної музики.
feat-logs-title = Логи прослуховування
feat-logs-desc = Лічильники відтворень локально. Бач що слухаєш найбільше з часом.
feat-i18n-title = i18n
feat-i18n-desc = Англійська, російська, німецька, французька, іспанська, турецька, українська, польська, арабська, грецька, іврит, угорська, індонезійська, японська, корейська, румунська, португальська, китайська та інші.
feat-ytdlp-title = Завантажувач yt-dlp
feat-ytdlp-desc = Завантажуй аудіо напряму з YouTube. Вивід у MP3, FLAC, WAV, MP4 або найкращій якості. SponsorBlock, поділ на розділи та обмеження швидкості.
feat-crossfade-title = Кросфейд і переходи
feat-crossfade-desc = Плавні переходи між треками. Підтримка кросфейду в нативних десктоп-збірках.
feat-channels-title = Режими каналів
feat-channels-desc = Стерео, моно, тільки лівий, тільки правий і L/R swap. Точне керування каналами.

## Performance
perf-title = Створено бути швидким.
perf-subtitle = Великі бібліотеки. Миттєвий запуск. Без зависань.
perf-skip-label = Пропуск проіндексованого
perf-skip-desc = Повторне сканування обробляє тільки нові файли. 10k треків + 5 нових = читаються тільки 5.
perf-parallel-label = Паралельний запуск
perf-parallel-desc-1 = Бібліотека, конфіг, плейлисти й обране завантажуються паралельно через
perf-parallel-desc-2 = . Майже миттєве відкриття.
perf-art-label = Кешування обкладинок
perf-art-desc = Обкладинки витягуються один раз, зберігаються на диск. Жодного повторного декодування.
perf-lazy-label = Лінива загрузка зображень
perf-lazy-desc = Сотні обкладинок у результатах пошуку — жодна не завантажується доки не з'явиться у видимості.
perf-io-label = Неблокуючий I/O
perf-io-desc-1 = Важка робота працює на
perf-io-desc-2 = { " " }потоках. UI лишається чуйним під час повного сканування.
perf-http-label = HTTP-кеш обкладинок
perf-http-desc-1 = Кастомний
perf-http-desc-2 = { " " }протокол віддає обкладинки із заголовками кешу на рік. Webview не перезапитує.

## Install
install-title = Встановлення
install-nix-title = Nix / NixOS
install-nix-run = Запуск без встановлення:
install-nix-profile = Або додай у свій профіль:
install-nix-note = Підтримка NixOS flake з Cachix binary cache.
install-flatpak-title = Flatpak
install-flatpak-desc = Встановлення з маніфесту:
install-flatpak-note = Лістинг на Flathub скоро.
install-appimage-title = AppImage
install-appimage-desc-1 = Завантаж з GitHub Releases і запусти. Потрібно
install-appimage-desc-2 = { " " }і
install-appimage-desc-3 = { " " }у системі.
install-appimage-note-1 = Користувачі Arch: якщо падає з помилкою WebKitNetworkProcess, запускай з префіксом
install-appimage-note-2 = .
install-macos-title = macOS
install-macos-chip = Apple Silicon
install-macos-desc-1 = Завантаж
install-macos-desc-2 = { " " }з GitHub Releases. Якщо macOS блокує, прибери прапор карантину:

## Platforms
platforms-title = Завантажити Kopuz
platforms-subtitle = Безкоштовно й з відкритим кодом. Усі релізи на GitHub.
platforms-windows = Windows
platforms-macos = macOS
platforms-macos-note = Тільки Apple Silicon
platforms-linux = Linux
platforms-download = Завантажити →

## Support
support-title = Підтримати Kopuz
support-subtitle = Kopuz безкоштовний і open source. Підтримка тримає його на плаву.
support-gh = GitHub Sponsors
support-bmc = Buy Me a Coffee
support-crypto-divider = — або відправ крипту —
support-usdt-note = (мережа Solana)

## Sponsors
sponsors-title = Спонсори
sponsors-subtitle = Люди, що підтримують Kopuz.
sponsors-cta = Стати спонсором

## Gallery
gallery-page-title = Галерея — Kopuz Music Player
gallery-page-desc = Скріншоти Kopuz у дії — головна, бібліотека, плейлист, повноекранний плеєр, тексти, редактор тем і більше.
gallery-title = Галерея
gallery-subtitle = Скріншоти Kopuz — стилі Normal та Modern.
gallery-home = Головна
gallery-home-styles = Normal / Modern
gallery-library = Бібліотека
gallery-library-styles = Normal / Vaxry
gallery-playlist = Плейлист
gallery-playlist-styles = Normal / Modern
gallery-fullscreen-title = Повноекранний плеєр
gallery-fullscreen-desc = Імерсивний повновіконний режим відтворення.
gallery-fullscreen-lyrics-title = Повноекранні тексти
gallery-fullscreen-lyrics-desc = Синхронізовані тексти в повновіконному режимі.
gallery-search-title = Пошук
gallery-search-desc = Пошук у реальному часі по виконавцях, альбомах і треках.
gallery-theme-title = Редактор тем
gallery-theme-desc = Повний контроль змінних кольору. Створи або обери пресет.
gallery-settings-title = Налаштування плеєра
gallery-settings-desc = Налаштуй звук, поведінку, інтеграції та більше.
gallery-downloader-title = Завантажувач
gallery-downloader-desc = Завантажуй треки напряму з Kopuz.
gallery-prev = ← Назад
gallery-next = Вперед →
gallery-label-normal-home = Normal — Головна
gallery-label-modern-home = Modern — Головна
gallery-label-normal-library = Normal — Бібліотека
gallery-label-vaxry-library = Vaxry — Бібліотека
gallery-label-normal-playlist = Normal — Плейлист
gallery-label-modern-playlist = Modern — Плейлист
gallery-label-fullscreen = Повноекранний плеєр
gallery-label-fullscreen-lyrics = Повноекранні тексти
gallery-label-search = Пошук
gallery-label-theme-editor = Редактор тем
gallery-label-player-settings = Налаштування плеєра
gallery-label-downloader = Завантажувач

## Footer
footer-license = MIT License — Безкоштовно й Open Source
footer-github = GitHub
footer-releases = Релізи
footer-issues = Issues
footer-discord = Discord

## Page meta
home-title = Kopuz — Music Player
home-meta-desc = Kopuz — сучасний легкий музичний плеєр на Rust і Dioxus. Стрім з Jellyfin або Navidrome, локальні файли, синхронізовані тексти, еквалайзер, теми і більше.
home-meta-keywords = Kopuz, музичний плеєр, Rust, Dioxus, Jellyfin, Navidrome, open source, Linux, macOS, Windows, MPRIS, тексти пісень
og-title = Kopuz — Music Player
og-desc = Сучасний легкий музичний плеєр на Rust. Локальні файли, Jellyfin, Navidrome, синхронізовані тексти, еквалайзер, Discord RPC і більше. Безкоштовно й open source.
og-image-alt = Музичний плеєр Kopuz
twitter-title = Kopuz — Music Player
twitter-desc = Сучасний легкий музичний плеєр на Rust. Безкоштовно й open source.
